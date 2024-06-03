use clap::{Parser, Subcommand};

use commands::PackageManager;

mod commands;
mod enums;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Main commands
    #[clap(flatten)]
    main_command: MainCommands,

    #[command(subcommand)]
    extra_commands: Option<Subcommands>,
}

#[derive(Debug, PartialEq, Eq, Subcommand)]
enum Subcommands {
    #[command(external_subcommand)]
    All(Vec<String>),
}

#[derive(Parser, Debug)]
#[group(required = true, multiple = false)]
pub struct MainCommands {
    /// Run install command
    #[arg(short, long)]
    install: bool,

    /// Run upgrade command
    #[arg(short, long)]
    update: bool,

    /// Run upgrade command
    #[arg(long)]
    upgrade: bool,

    /// Run remove command
    #[arg(short, long)]
    remove: bool,
}

fn main() {
    let args = Args::parse();

    let additional_args =
        args.extra_commands.map(|Subcommands::All(options)| options);

    PackageManager::new().run_command(&args.main_command, additional_args);
}
