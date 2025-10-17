use std::env;

use anyhow::{Result, bail};
use testeq_rs::equipment::{self, Equipment};

use crate::devices::psu;

mod command;
mod devices;

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<_> = env::args().collect();

    if args.len() < 2 {
        bail!("Usage: testeq-cli <uri> ...")
    }

    let uri = &args[1];

    let equipment = equipment::equipment_from_uri(uri).await?;

    match equipment {
        Equipment::PowerSupply(mut psu) => psu::handle_command(psu.as_mut(), &args[2..]).await?,
        _ => bail!("Unsupported equipment type"),
    }

    Ok(())
}
