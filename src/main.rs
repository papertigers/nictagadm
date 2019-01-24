/*
 * WIP conversion of nictagadm to rust.
 */

mod config;
mod bootparams;

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

    if unsafe { geteuid() != 0  } {
        eprintln!("This program can only be run as root");
        std::process::exit(1);
    }

    // XXX use clap to handle the arguments...

    config::load_sdc_config();
}
