mod icon_names {
    pub use shipped::*;
    include!(concat!(env!("OUT_DIR"), "/icon_names.rs"));
}
mod app;
mod dead_internet;

use clap::Parser;
use gtk4::{CssProvider, STYLE_PROVIDER_PRIORITY_APPLICATION, gdk::Display};
use relm4::{RelmApp, gtk};
use tracing::{Level, debug};
use tracing_subscriber::FmtSubscriber;

use crate::app::{AppModel, InitApp};

fn load_css() {
    let default_provider = CssProvider::new();
    default_provider.load_from_resource("/com/bt/powermenu-rs/style.css");

    let display = Display::default().expect("Cannot get default display");
    gtk::style_context_add_provider_for_display(
        &display,
        &default_provider,
        STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

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

    #[arg(allow_hyphen_values = true, trailing_var_arg = true)]
    gtk_options: Vec<String>,
}

fn get_user() -> String {
    std::env::var("USER").unwrap_or("Anon".to_string())
}

fn get_host() -> Option<String> {
    std::fs::read_to_string("/proc/sys/kernel/hostname")
        .map(|s| s.trim().to_string())
        .ok()
}

fn main() {
    let args = Cli::parse();
    let program_invoc = std::env::args().next().unwrap();
    let mut gtk_args = vec![program_invoc];

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

    gtk_args.extend(args.gtk_options.clone());

    // setup gtk
    relm4_icons::initialize_icons(icon_names::GRESOURCE_BYTES, icon_names::RESOURCE_PREFIX);
    dead_internet::init_resources();
    load_css();
    gtk::init().unwrap();

    let app = RelmApp::new("com.bt.powermenu");
    let user = get_user();
    let host = get_host();

    debug!("user: {}, host: {:?}", user, host);

    app.with_args(gtk_args).run::<AppModel>(InitApp {
        no_focus: args.no_focus,
        dryrun: args.dryrun,
        user: user,
        host: host,
    });
}
