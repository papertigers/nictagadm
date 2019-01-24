use crate::errors::SdcConfigError;
use crate::utils;

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::Command;


/// Return `true` if boot-time networking is enabled.
pub fn boot_file_config_enabled() -> std::io::Result<bool> {
    let out = Command::new("/usr/lib/sdc/net-boot-config")
        .arg("--enabled")
        .output()?;
    Ok(out.status.success())
}

/// Lookup a svcprop by forking svcprop. Ignoring exit status
/// In the future it would be nice to have an svc crate.
pub fn svc_prop<S: AsRef<str>>(prop: S, svc: S) -> Result<String, SdcConfigError> {
    let output = Command::new("svcprop")
        .args(&["-p", prop.as_ref(), svc.as_ref()])
        .output()?;

    if !output.status.success() {
        return Err(SdcConfigError::BadExitStatus("svcprop".to_string(), output.status));
    }
    Ok(String::from(std::str::from_utf8(&output.stdout)?.trim()))

}

/// Mapping of SDC config variables to values
#[derive(Debug)]
pub struct SdcConfig(HashMap<String, String>);

impl SdcConfig {
    // XXX add impl
}

/// SDC config paths
#[derive(Debug)]
struct SdcConfigPaths {
    pub config_file: PathBuf,
    pub config_inc_dir: PathBuf,
}

/// Returns an `SdcConfigPaths` with the location of the config file. This can
/// come from the USB key, /opt/smartdc/config/node.config, or (if on an unsetup
/// CN) /var/tmp/node.config/node.config
fn get_sdc_config_filename() -> Result<SdcConfigPaths, SdcConfigError> {
    // default config
    let default = Path::new("/opt/smartdc/config/node.config");
    let mut sdc_config_inc = String::new();
    let mut cn_config = String::new();

    let prop = svc_prop(
        "joyentfs/usb_copy_path",
        "svc:/system/filesystem/smartdc:default"
    )?;
    let mut sdc_config = format!("{}/config", &prop);
    if !Path::new(&sdc_config).exists() {
        let prop = svc_prop(
            "joyentfs/usb_mountpoint",
            "svc:/system/filesystem/smartdc:default"
        )?;
        sdc_config = format!("/mnt/{}/config", &prop);
    }

    if Path::new(&sdc_config).exists() {
        let dir = Path::new(&sdc_config).parent().unwrap_or_else(|| Path::new("."));
        sdc_config_inc = format!("{}/config.inc", dir.to_string_lossy());
    } else if !Path::new(&default).exists() &&
        Path::new("/var/tmp/node.config/node.config").exists() {
        cn_config = "/var/tmp/node.config/node.config".into();
    }

    if Path::new(&cn_config).exists() {
        if Path::new(&sdc_config).exists() {
            // unwrap because we demand nictagadm be ran as root
            let mut dev_msg = File::open("/dev/msglog").unwrap();
            let _ = writeln!(&mut dev_msg, "WARNING: ignoring config at {} since we have {}",
                &cn_config, sdc_config);
        } else {
            sdc_config_inc = Path::new(&cn_config)
                .parent()
                .unwrap_or_else(|| Path::new("."))
                .to_string_lossy()
                .into();
            sdc_config = cn_config;
        }
    }

    Ok(SdcConfigPaths {
        config_file: sdc_config.into(),
        config_inc_dir: sdc_config_inc.into()
    })
}

fn sdc_config_to_map(
    config: &SdcConfigPaths,
    headnode: bool)
    -> Result<SdcConfig, SdcConfigError> {
    let mut map = HashMap::new();
    // XXX check for GEN

    let f = match File::open(&config.config_file) {
        Ok(f) => f,
        Err(_) if headnode => {
            println!("FATAL: Unable to load headnode config.");
            std::process::exit(1);
        }
        Err(e) => return Err(e.into()),
    };
    let f = BufReader::new(f);
    utils::parse_config_file(f, Some(r"^[a-zA-Z].*$"), &mut map);
    map.insert("config_inc_dir".into(),
        config.config_inc_dir.to_str().unwrap().into());

    Ok(SdcConfig(map))
}

/// Loads sdc config variables
pub fn load_sdc_config(headnode: bool) -> Result<SdcConfig, SdcConfigError> {
    let paths = get_sdc_config_filename()?;
    let config = sdc_config_to_map(&paths, headnode)?;
    Ok(config)
}
