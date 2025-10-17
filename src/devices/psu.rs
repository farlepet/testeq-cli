use std::sync::Arc;

use anyhow::{Result, bail};
use testeq_rs::{
    data::{Reading, Unit},
    equipment::psu::{PowerSupplyChannel, PowerSupplyEquipment},
};
use tokio::sync::Mutex;

use crate::command::{CommandHandler, Commands, cmd_hdlr, cmd_hdlr_args};

pub async fn handle_command<'a>(
    psu: &'a mut dyn PowerSupplyEquipment,
    args: &'a [String],
) -> Result<()> {
    psu.connect().await?;

    if args.is_empty() {
        return command_status(psu, args).await;
    }

    let commands = Commands {
        mod_name: "Power Supply".to_string(),
        handlers: vec![
            cmd_hdlr!("status", "Show status", command_status),
            cmd_hdlr_args!(
                "enable",
                "<chan> [<enable>]",
                "Get/set power supply channel enabled",
                command_enable
            ),
            cmd_hdlr_args!(
                "set_voltage",
                "<chan> [<voltage>]",
                "Get/set power supply channel set voltage",
                command_set_voltage
            ),
            cmd_hdlr_args!(
                "set_current",
                "<chan> [<current>]",
                "Get/set power supply channel set current",
                command_set_current
            ),
            cmd_hdlr_args!(
                "read_voltage",
                "<chan>",
                "Get power supply channel readback voltage",
                command_read_voltage
            ),
            cmd_hdlr_args!(
                "read_current",
                "<chan>",
                "Get power supply channel readback voltage",
                command_read_current
            ),
            cmd_hdlr_args!(
                "read_power",
                "<chan>",
                "Get power supply channel readback voltage",
                command_read_power
            ),
        ],
    };

    commands.run_command(psu, args).await
}

async fn command_status(psu: &mut dyn PowerSupplyEquipment, _args: &[String]) -> Result<()> {
    let mut chans = psu.get_channels().await?;
    for chan_mutex in &mut chans {
        let chan = chan_mutex.lock().await;

        println!("Channel {}", chan.name()?);
        println!("  state: {}", chan.get_enabled().await?);
        println!("  set voltage:  {} V", chan.get_voltage().await?);
        println!("  set current:  {} A", chan.get_current().await?);
        println!("  read voltage: {} V", chan.read_voltage().await?);
        println!("  read current: {} A", chan.read_current().await?);
        println!("  read power:   {} W", chan.read_power().await?);
    }

    Ok(())
}

async fn get_chan(
    psu: &mut dyn PowerSupplyEquipment,
    chan: &str,
) -> Result<Arc<Mutex<dyn PowerSupplyChannel>>> {
    /* TODO: Support channel by name */
    Ok(psu.get_channel(chan.parse()?).await?)
}

async fn command_enable(psu: &mut dyn PowerSupplyEquipment, args: &[String]) -> Result<()> {
    if (args.len() < 2) || (args.len() > 3) {
        bail!("Usage: ... enable <channel> [<state>]")
    }

    let channel = get_chan(psu, &args[1]).await?;
    let mut channel = channel.lock().await;

    if args.len() == 2 {
        let en = channel.get_enabled().await?;
        println!("{en}");
    } else {
        let state = match args[2].to_lowercase().as_ref() {
            "0" | "off" | "false" => false,
            "1" | "on" | "true" => true,
            _ => bail!("Invalid state value '{}'", args[2]),
        };

        channel.set_enabled(state).await?;
    }

    Ok(())
}

async fn command_set_voltage(psu: &mut dyn PowerSupplyEquipment, args: &[String]) -> Result<()> {
    if (args.len() < 2) || (args.len() > 3) {
        bail!("Usage: ... set_voltage <channel> [<voltage>]")
    }

    let channel = get_chan(psu, &args[1]).await?;
    let mut channel = channel.lock().await;

    if args.len() == 2 {
        println!(
            "{}",
            Reading::new(Unit::Voltage, channel.get_voltage().await? as f64)
        );
    } else {
        let voltage = args[2].parse()?;
        channel.set_voltage(voltage).await?;
    }

    Ok(())
}

async fn command_set_current(psu: &mut dyn PowerSupplyEquipment, args: &[String]) -> Result<()> {
    if (args.len() < 2) || (args.len() > 3) {
        bail!("Usage: ... set_current <channel> [<current>]")
    }

    let channel = get_chan(psu, &args[1]).await?;
    let mut channel = channel.lock().await;

    if args.len() == 2 {
        println!(
            "{}",
            Reading::new(Unit::Current, channel.get_current().await? as f64)
        );
    } else {
        let current = args[2].parse()?;
        channel.set_current(current).await?;
    }

    Ok(())
}

async fn command_read_voltage(psu: &mut dyn PowerSupplyEquipment, args: &[String]) -> Result<()> {
    if args.len() != 2 {
        bail!("Usage: ... read_voltage <channel>")
    }

    let channel = get_chan(psu, &args[1]).await?;
    let channel = channel.lock().await;

    println!(
        "{}",
        Reading::new(Unit::Voltage, channel.read_voltage().await? as f64)
    );

    Ok(())
}

async fn command_read_current(psu: &mut dyn PowerSupplyEquipment, args: &[String]) -> Result<()> {
    if args.len() != 2 {
        bail!("Usage: ... read_current <channel>")
    }

    let channel = get_chan(psu, &args[1]).await?;
    let channel = channel.lock().await;

    println!(
        "{}",
        Reading::new(Unit::Current, channel.read_current().await? as f64)
    );

    Ok(())
}

async fn command_read_power(psu: &mut dyn PowerSupplyEquipment, args: &[String]) -> Result<()> {
    if args.len() != 2 {
        bail!("Usage: ... read_power <channel>")
    }

    let channel = get_chan(psu, &args[1]).await?;
    let channel = channel.lock().await;

    println!(
        "{}",
        Reading::new(Unit::Power, channel.read_power().await? as f64)
    );

    Ok(())
}
