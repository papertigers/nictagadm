/*
 * WIP conversion of nictagadm to rust.
 */

mod bootparams;
mod config;
mod errors;
mod utils;

use errors::SdcConfigError;

fn find_config_paths(smartos: bool) -> Result<bool, SdcConfigError> {
    let tmp_conf = "/tmp/.nic-tags";
    match config::boot_file_config_enabled() {
        Err(e) => {
            eprintln!("failed to determine boot-time networking: {}", e);
            std::process::exit(1);
        }
        Ok(val) => {
            // set some values
            //return Ok(true);
        }
    }

    let usb_config_copy = config::svc_prop(
        "joyentfs/usb_copy_path",
        "svc:/system/filesystem/smartdc:default",
    )?;
    let usb_mnt = config::svc_prop(
        "joyentfs/usb_mountpoint",
        "svc:/system/filesystem/smartdc:default",
    )?;

    Ok(true)
}

/// Entry point
fn main() {
    let zonename = zonename::getzonename().expect("failed to get the zone name");
    if zonename != "global" {
        std::process::exit(1);
    }

    // avoid pulling in the libc crate for now if we can
    extern "C" {
        pub fn geteuid() -> u32;
    }

    if unsafe { geteuid() != 0 } {
        eprintln!("This program can only be run as root");
        std::process::exit(1);
    }

    // XXX use clap to handle the arguments...

    // XXX unwrap for now...handle errors when we get further
    let mut headnode = false;
    let mut smartos = false;

    let bootparams = bootparams::get_bootparams();
    if let Some(val) = bootparams.get("headnode") {
        if val == "true" {
            headnode = true
        };
    }
    if let Some(val) = bootparams.get("smartos") {
        if val == "true" {
            smartos = true
        };
    }

    let config = config::load_sdc_config(headnode).unwrap();
    println!("{:?}", config);
    let paths = find_config_paths(smartos).unwrap();
}
