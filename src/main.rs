mod app;
// mod dead_internet;

use clap::Parser;
use tracing::{Level, debug};
use tracing_subscriber::FmtSubscriber;

use crate::app::{AppModel, SIZE};

#[derive(Parser, Debug)]
#[command(version, about)]
struct Cli {
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
    /// don't quite on focus lost
    #[arg(short, long)]
    no_focus: bool,

    /// don't run commands, echo them out instead
    #[arg(short, long)]
    dryrun: bool,
}


fn main() -> iced::Result {
    let args = Cli::parse();
    let program_invoc = std::env::args().next().unwrap();

    // initialize tracing
    let log_level = match args.verbose {
        0 => Level::ERROR,
        1 => Level::INFO,
        2 => Level::DEBUG,
        _ => Level::TRACE,
    };
    let subscriber = FmtSubscriber::builder().with_max_level(log_level).finish();
    tracing::subscriber::set_global_default(subscriber).expect("Failed to setup tracing");

    debug!("cli args {:?}", &args);
    if args.no_focus {
        println!("Powermenu: Running in no focus mode");
    }

    if args.dryrun {
        println!("Powermenu: Running in dryrun mode");
    }

    iced::application(AppModel::new, AppModel::update, AppModel::view)
        .subscription(AppModel::subscription)
        .window_size(SIZE)
        .title("Powermenu")
        .run()
}
