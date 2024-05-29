use clap::{Parser, Subcommand};

use commands::PackageManager;

use crate::enums::AvailableCommands;

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
struct MainCommands {
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

    let package_managers = PackageManager::new();

    let other_args: Option<Vec<String>> =
        args.extra_commands.map(|Subcommands::All(options)| options);

    if args.main_command.install {
        package_managers.run_command(AvailableCommands::Install, other_args);
    } else if args.main_command.update || args.main_command.upgrade {
        package_managers.run_command(AvailableCommands::Update, None);
    } else if args.main_command.remove {
        package_managers.run_command(AvailableCommands::Remove, other_args);
    }
}
