mod icon_names {
    pub use shipped::*;
    include!(concat!(env!("OUT_DIR"), "/icon_names.rs"));
}
mod app;
mod dead_internet;

use clap::Parser;
use gtk4::{
    CssProvider, STYLE_PROVIDER_PRIORITY_APPLICATION, STYLE_PROVIDER_PRIORITY_USER, gdk::Display,
};
use relm4::{RelmApp, gtk};
use tracing::{Level, debug};
use tracing_subscriber::FmtSubscriber;

use crate::app::{AppModel, InitApp};

fn load_css() {
    let default_provider = CssProvider::new();
    default_provider.load_from_resource("/com/bt/powermenu-rs/style.css");

    let user_provider = CssProvider::new();
    if let Some(config_dir) = dirs::config_dir() {
        let user_css = config_dir.join("powermenu/style.css");
        if user_css.exists() {
            user_provider.load_from_path(user_css);
        }
    }
    let display = Display::default().expect("Cannot get default display");
    gtk::style_context_add_provider_for_display(
        &display,
        &default_provider,
        STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
    gtk::style_context_add_provider_for_display(
        &display,
        &user_provider,
        STYLE_PROVIDER_PRIORITY_USER,
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
    app.with_args(gtk_args).run::<AppModel>(InitApp {
        no_focus: args.no_focus,
        dryrun: args.dryrun,
    });
}
