use std::sync::Arc;

use anyhow::{Result, bail};
use testeq_rs::equipment::multimeter::{MultimeterChannel, MultimeterEquipment};
use tokio::sync::Mutex;

use crate::command::{CommandHandler, Commands, cmd_hdlr, cmd_hdlr_args};

pub async fn handle_command<'a>(
    dmm: &'a mut dyn MultimeterEquipment,
    args: &'a [String],
) -> Result<()> {
    dmm.connect().await?;

    if args.is_empty() {
        return command_status(dmm, args).await;
    }

    let commands = Commands {
        mod_name: "Multimeter".to_string(),
        handlers: vec![
            cmd_hdlr!("status", "Show status", command_status),
            cmd_hdlr_args!(
                "mode",
                "<chan> [<mode>]",
                "Get/set DMM channel mode",
                command_mode
            ),
            cmd_hdlr_args!(
                "read",
                "<chan>",
                "Read current DMM channel reading",
                command_read
            ),
            cmd_hdlr_args!(
                "read_now",
                "<chan>",
                "Trigger DMM and get reading from channel",
                command_read_now
            ),
            cmd_hdlr_args!(
                "trig_source",
                "[<source>]",
                "Get/set DMM trigger source",
                command_trig_source
            ),
            cmd_hdlr!("arm", "Arm trigger", command_arm),
        ],
    };

    commands.run_command(dmm, args).await
}

async fn command_status(dmm: &mut dyn MultimeterEquipment, _args: &[String]) -> Result<()> {
    println!("trig mode: {}", dmm.get_trigger_source().await?);
    let mut chans = dmm.get_channels().await?;
    for chan_mutex in &mut chans {
        let chan = chan_mutex.lock().await;

        println!("Channel {}", chan.name()?);
        println!("  mode: {}", chan.get_mode().await?);
        /* Do not perform reading here, as trigger mode might not be immediate or bus. */
    }

    Ok(())
}

async fn get_chan(
    dmm: &mut dyn MultimeterEquipment,
    chan: &str,
) -> Result<Arc<Mutex<dyn MultimeterChannel>>> {
    /* TODO: Support channel by name */
    Ok(dmm.get_channel(chan.parse()?).await?)
}

async fn command_mode(dmm: &mut dyn MultimeterEquipment, args: &[String]) -> Result<()> {
    if (args.len() < 2) || (args.len() > 3) {
        bail!("Usage: ... mode <channel> [<mode>]")
    }

    let channel = get_chan(dmm, &args[1]).await?;
    let mut channel = channel.lock().await;

    if args.len() == 2 {
        let mode = channel.get_mode().await?;
        println!("{mode}");
    } else {
        let mode = args[2].to_lowercase().parse()?;
        channel.set_mode(mode, None).await?;
    }

    Ok(())
}

async fn command_read(dmm: &mut dyn MultimeterEquipment, args: &[String]) -> Result<()> {
    if args.len() != 2 {
        bail!("Usage: ... read <channel>")
    }

    let channel = get_chan(dmm, &args[1]).await?;
    let channel = channel.lock().await;

    let reading = channel.get_reading().await?;
    println!("{reading}");

    Ok(())
}

async fn command_read_now(dmm: &mut dyn MultimeterEquipment, args: &[String]) -> Result<()> {
    if args.len() != 2 {
        bail!("Usage: ... read_now <channel>")
    }

    let channel = get_chan(dmm, &args[1]).await?;
    let channel = channel.lock().await;

    dmm.trigger_arm().await?;
    dmm.trigger_now().await?;

    let reading = channel.get_reading().await?;
    println!("{reading}");

    Ok(())
}

async fn command_trig_source(dmm: &mut dyn MultimeterEquipment, args: &[String]) -> Result<()> {
    if args.len() > 2 {
        bail!("Usage: ... trig_source [<source>]")
    }

    if args.len() == 1 {
        let source = dmm.get_trigger_source().await?;
        println!("{source}");
    } else {
        let source = args[1].to_lowercase().parse()?;
        dmm.set_trigger_source(source).await?;
    }

    Ok(())
}

async fn command_arm(dmm: &mut dyn MultimeterEquipment, _args: &[String]) -> Result<()> {
    dmm.trigger_arm().await?;

    Ok(())
}
