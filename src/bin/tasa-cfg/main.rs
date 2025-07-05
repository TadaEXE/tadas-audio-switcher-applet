mod cmd_impl;

use tadas_audio_switcher_applet::config;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "tasa-cfg")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command()]
    ShowConfig {},
    #[command()]
    ListSinks {},
    #[command()]
    MakeSlot {},
    #[command(arg_required_else_help = true)]
    RemoveSlot { slot_id: u32 },
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::ShowConfig {} => println!("{:#?}", config::load_config()),
        Commands::ListSinks {} => cmd_impl::list_sinks_cmd(),
        Commands::MakeSlot {} => cmd_impl::make_slot_cmd(),
        Commands::RemoveSlot { slot_id } => todo!(),
    }
}
