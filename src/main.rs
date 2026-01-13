use std::env;

use anyhow::{Result, bail};
use testeq_rs::equipment::{self, Equipment};

use crate::devices::{dmm, psu};

mod command;
mod devices;

fn handle_args(args: &mut Vec<String>) -> Result<bool> {
    let i = 1;

    while i < args.len() {
        if !args[i].starts_with('-') {
            /* End of flags */
            break;
        }

        match args[i].as_str() {
            "-v" | "--version" => {
                println!("Version: {}", env!("CARGO_PKG_VERSION"));
                println!("testeq-rs version: {}", testeq_rs::version());

                return Ok(true);
            }
            "-h" | "--help" => {
                print_usage();
                return Ok(true);
            }
            "-V" | "--verbose" => {
                /* TODO, leaving unimplemented just to be able to keep loop
                 * without warnings. */
            }
            flag => {
                print_usage();
                println!();
                bail!("Unknown option `{flag}`")
            }
        }

        args.remove(i);
    }

    Ok(false)
}

fn print_usage() {
    println!("Usage: testeq-cli [flags] <uri> ...");
    println!();
    println!("flags:");
    println!("  -v --version   Print version of testeq-cli");
    println!("  -h --help      Show this help message");
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let mut args: Vec<_> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        println!();
        bail!("Missing <uri>");
    }

    if handle_args(&mut args)? {
        return Ok(());
    }

    let uri = &args[1];

    let equipment = equipment::equipment_from_uri(uri).await?;

    match equipment {
        Equipment::Multimeter(mut dmm) => dmm::handle_command(dmm.as_mut(), &args[2..]).await?,
        Equipment::PowerSupply(mut psu) => psu::handle_command(psu.as_mut(), &args[2..]).await?,
        _ => bail!("Unsupported equipment type"),
    }

    Ok(())
}
