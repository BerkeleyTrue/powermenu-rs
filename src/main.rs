mod icon_names {
    pub use shipped::*;
    include!(concat!(env!("OUT_DIR"), "/icon_names.rs"));
}
mod app;
mod dead_internet;

use gtk4::{
    CssProvider, STYLE_PROVIDER_PRIORITY_APPLICATION, STYLE_PROVIDER_PRIORITY_USER, gdk::Display,
};
use relm4::{RelmApp, gtk};

use crate::app::AppModel;

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

fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();
    relm4_icons::initialize_icons(icon_names::GRESOURCE_BYTES, icon_names::RESOURCE_PREFIX);
    dead_internet::init_resources();
    load_css();

    gtk::init().unwrap();

    let app = RelmApp::new("com.bt.powermenu");
    app.run::<AppModel>(());
}
