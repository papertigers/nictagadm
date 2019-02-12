use std::borrow::Cow;
use std::collections::HashMap;
use std::ffi::c_void;
use std::ffi::CStr;

mod ffi;
use self::ffi::{
    PropData, PropType, DI_PROP_TYPE_BOOLEAN, DI_PROP_TYPE_BYTE, DI_PROP_TYPE_INT,
    DI_PROP_TYPE_INT64, DI_PROP_TYPE_STRING, DI_PROP_TYPE_UNDEF_IT, DI_PROP_TYPE_UNKNOWN,
};

/// Figure out the prop type and parse out the data.
/// Note: bootparams only cares about string data but this function parses all types.
fn prop_type_guess(prop: ffi::di_prop_t, prop_data: PropData, prop_type: PropType) -> i32 {
    let mut len: i32 = 0;
    unsafe {
        let mut _type = ffi::di_prop_type(prop);
        match _type {
            DI_PROP_TYPE_UNDEF_IT => {
                *prop_data = std::ptr::null_mut();
                *prop_type = _type;
                return len;
            }
            DI_PROP_TYPE_BOOLEAN => {
                *prop_data = std::ptr::null_mut();
                *prop_type = _type;
                return len;
            }
            DI_PROP_TYPE_INT => len = ffi::di_prop_ints(prop, prop_data),
            DI_PROP_TYPE_INT64 => len = ffi::di_prop_int64(prop, prop_data),
            DI_PROP_TYPE_BYTE => len = ffi::di_prop_bytes(prop, prop_data),
            DI_PROP_TYPE_STRING => len = ffi::di_prop_strings(prop, prop_data),
            DI_PROP_TYPE_UNKNOWN => {
                len = ffi::di_prop_strings(prop, prop_data);
                if len > 0 && **(prop_data as *const *const u8) != 0 {
                    *prop_type = DI_PROP_TYPE_STRING;
                    return len;
                }
                len = ffi::di_prop_ints(prop, prop_data);
                _type = DI_PROP_TYPE_INT;
            }
            _ => len = -1,
        }

        if len > 0 {
            *prop_type = _type;
            return len;
        }

        len = ffi::di_prop_rawdata(prop, prop_data);
        if len < 0 {
            return -1;
        } else if len == 0 {
            *prop_type = DI_PROP_TYPE_BOOLEAN;
            return 0;
        }

        *prop_type = DI_PROP_TYPE_UNKNOWN;
        len
    }
}

/// Read a string property from a di_prop_t and return it as a tuple
fn read_prop<'a>(prop: ffi::di_prop_t) -> Option<(Cow<'a, str>, Cow<'a, str>)> {
    let mut data: *mut c_void = std::ptr::null_mut();
    let prop_data = &mut data as *mut _;
    let mut _type: i32 = 0;
    let prop_type: PropType = &mut _type as *mut i32;

    let nitems = prop_type_guess(prop, prop_data, prop_type);
    if nitems != 1 || _type != DI_PROP_TYPE_STRING {
        return None;
    };
    let name = unsafe { CStr::from_ptr(ffi::di_prop_name(prop)) };
    if _type == DI_PROP_TYPE_STRING {
        let val = unsafe { CStr::from_ptr(data as *const i8) };
        return Some((name.to_string_lossy(), val.to_string_lossy()));
    }
    None
}

/// Reads the root node's data.
/// The C version of bootparams doesn't do any sort of error handling so we mimic that here.
fn read_root_node(node: ffi::di_node_t) -> HashMap<String, String> {
    let mut map = HashMap::new();
    let mut prop: ffi::di_prop_t = std::ptr::null();
    unsafe {
        let name = CStr::from_ptr(ffi::di_node_name(node, prop));
        if name.to_str().unwrap() != "i86pc" {
            return map;
        }
        loop {
            prop = ffi::di_prop_next(node, prop);
            if prop.is_null() {
                break;
            }
            if let Some((k, v)) = read_prop(prop) {
                map.insert(k.to_string(), v.to_string());
            }
        }
    }
    map
}

/// Gets the bootparams from the system via devinfo and returns the data as a HashMap.
pub fn get_bootparams() -> HashMap<String, String> {
    let root_path = CStr::from_bytes_with_nul(b"/\0").unwrap();
    let root_ptr = root_path.as_ptr();

    unsafe {
        // The node we care about is the first one so no need to walk the structure
        let root_node = ffi::di_init(root_ptr, ffi::DINFOSUBTREE | ffi::DINFOPROP);
        if root_node.is_null() {
            eprintln!("di_init() failed");
            std::process::exit(1);
        }

        let map = read_root_node(root_node);
        ffi::di_fini(root_node);
        map
    }
}
