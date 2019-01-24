use crate::bootparams;

use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Lookup a svcprop by forking svcprop. Ignoring exit status
/// In the future it would be nice to have an svc crate.
macro_rules! svc_prop {
    ($svc:expr, $prop:expr) => {{
        let output = Command::new("svcprop")
            .args(&["-p", $prop, $svc])
            .output()
            .expect("failed to execute svcprop")
            .stdout;
        String::from(std::str::from_utf8(&output)
            .expect("invalid utf8 data from svcprop")
            .trim())
    };
}}

/// SDC config paths
#[derive(Debug)]
struct SdcConfig {
    pub config_file: PathBuf,
    pub config_inc_dir: PathBuf,
}

/// Returns an `SdcConfig` with the location of the config file. This can
/// come from the USB key, /opt/smartdc/config/node.config, or (if on an unsetup
/// CN) /var/tmp/node.config/node.config
fn get_sdc_config_filename() -> SdcConfig {
    // default config
    let default = Path::new("/opt/smartdc/config/node.config");
    let mut sdc_config = String::new();
    let mut sdc_config_inc = String::new();
    let mut cn_config = String::new();

    let prop = svc_prop!(
        "svc:/system/filesystem/smartdc:default",
        "joyentfs/usb_copy_path"
    );
    sdc_config = format!("{}/config", &prop);
    if !Path::new(&sdc_config).exists() {
        let prop = svc_prop!(
            "svc:/system/filesystem/smartdc:default",
            "joyentfs/usb_mountpoint"
        );
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

    SdcConfig {
        config_file: sdc_config.into(),
        config_inc_dir: sdc_config_inc.into()
    }
}

fn sdc_config_to_map(conf: &SdcConfig) -> HashMap<String, String> {
    let map = HashMap::new();
    map
}

/// Loads sdc config variables
pub fn load_sdc_config() {
    let mut headnode = false;

    let bootparams = bootparams::get_bootparams();
    if let Some(val) = bootparams.get("headnode") {
        if val == "true" {
            headnode = true
        };
    }

    let sdc_config = get_sdc_config_filename();
    let config = sdc_config_to_map(&sdc_config);
    println!("{:?}", config);
}
