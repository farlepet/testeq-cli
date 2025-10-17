use std::pin::Pin;

use anyhow::{Result, bail};
use testeq_rs::equipment::BaseEquipment;

macro_rules! cmd_hdlr {
    ($cmd:expr, $help:expr, $func:ident) => {
        CommandHandler {
            cmd: $cmd.to_string(),
            args: None,
            help: $help.to_string(),
            func: |eq, args| Box::pin($func(eq, args)),
        }
    };
}
pub(crate) use cmd_hdlr;

macro_rules! cmd_hdlr_args {
    ($cmd:expr, $args:expr, $help:expr, $func:ident) => {
        CommandHandler {
            cmd: $cmd.to_string(),
            args: Some($args.to_string()),
            help: $help.to_string(),
            func: |eq, args| Box::pin($func(eq, args)),
        }
    };
}
pub(crate) use cmd_hdlr_args;

type CommandCallback<'a, T> =
    fn(&'a mut T, &'a [String]) -> Pin<Box<dyn Future<Output = Result<()>> + 'a>>;

pub struct CommandHandler<'a, T: BaseEquipment + ?Sized> {
    pub cmd: String,
    pub args: Option<String>,
    pub help: String,
    pub func: CommandCallback<'a, T>,
}

pub struct Commands<'a, T: BaseEquipment + ?Sized> {
    pub mod_name: String,
    pub handlers: Vec<CommandHandler<'a, T>>,
}
impl<'a, T: BaseEquipment + ?Sized> Commands<'a, T> {
    pub async fn run_command(&self, eq: &'a mut T, args: &'a [String]) -> Result<()> {
        if args.is_empty() {
            bail!("Missing command argument");
        }

        let cmd: &str = args[0].as_ref();
        if cmd == "help" {
            println!("{} commands:", self.mod_name);
            for hdlr in &self.handlers {
                if let Some(args) = &hdlr.args {
                    println!("  {} {}: {}", hdlr.cmd, args, hdlr.help);
                } else {
                    println!("  {}: {}", hdlr.cmd, hdlr.help);
                }
            }
            return Ok(());
        }

        let handlers: Vec<_> = self
            .handlers
            .iter()
            .filter(|hand| hand.cmd.starts_with(cmd))
            .collect();

        if handlers.is_empty() {
            bail!("No command matching '{cmd}'");
        } else if handlers.len() > 1 {
            bail!(
                "Ambiguous command, multiple matches: {:?}",
                handlers.iter().map(|hand| &hand.cmd).collect::<Vec<_>>()
            );
        }

        (handlers[0].func)(eq, args).await
    }
}
