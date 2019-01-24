use std::os::raw::{c_char, c_int, c_uint, c_void};

const DIIOC: c_uint = 0xdf << 8;
pub const DINFOSUBTREE: c_uint = (DIIOC | 0x01);
pub const DINFOPROP: c_uint = (DIIOC | 0x04);

/// Property types
#[allow(non_camel_case_types)]
pub const DI_PROP_TYPE_BOOLEAN: c_int = 0;
#[allow(non_camel_case_types)]
pub const DI_PROP_TYPE_INT: c_int = 1;
#[allow(non_camel_case_types)]
pub const DI_PROP_TYPE_STRING: c_int = 2;
#[allow(non_camel_case_types)]
pub const DI_PROP_TYPE_BYTE: c_int = 3;
#[allow(non_camel_case_types)]
pub const DI_PROP_TYPE_UNKNOWN: c_int = 4;
#[allow(non_camel_case_types)]
pub const DI_PROP_TYPE_UNDEF_IT: c_int = 5;
#[allow(non_camel_case_types)]
pub const DI_PROP_TYPE_INT64: c_int = 6;

// Opaque rust type not stablized yet
// See RFC 1861 extern_types
pub enum DiNode {}
pub enum DiProp {}

#[allow(non_camel_case_types)]
pub type di_node_t = *const DiNode;
#[allow(non_camel_case_types)]
pub type di_prop_t = *const DiProp;

pub type PropData = *mut *mut c_void;
pub type PropType = *mut i32;

#[link(name = "devinfo")]
extern "C" {
    pub fn di_init(phys_path: *const c_char, flag: c_uint) -> di_node_t;
    pub fn di_fini(root: di_node_t);
    pub fn di_node_name(node: di_node_t, prop: di_prop_t) -> *const c_char;
    pub fn di_prop_next(node: di_node_t, prop: di_prop_t) -> di_prop_t;
    pub fn di_prop_type(prop: di_prop_t) -> c_int;
    // prop_data is really a "int **"
    pub fn di_prop_ints(prop: di_prop_t, prop_data: PropData) -> c_int;
    // prop_data is really a "int **"
    pub fn di_prop_int64(prop: di_prop_t, prop_data: PropData) -> c_int;
    // prop_data is really a "uchar_t **"
    pub fn di_prop_bytes(prop: di_prop_t, prop_data: PropData) -> c_int;
    // prop_data is really a "char **"
    pub fn di_prop_strings(prop: di_prop_t, prop_data: PropData) -> c_int;
    // prop_data is really a "uchar_t **"; this is also not a public interface
    pub fn di_prop_rawdata(prop: di_prop_t, prop_data: PropData) -> c_int;
    pub fn di_prop_name(prop: di_prop_t) -> *const c_char;
}
