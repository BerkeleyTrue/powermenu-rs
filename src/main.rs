mod icon_names {
    pub use shipped::*;
    include!(concat!(env!("OUT_DIR"), "/icon_names.rs"));
}
mod app;
use relm4::{RelmApp, gtk};

use crate::app::AppModel;

fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();
    relm4_icons::initialize_icons(icon_names::GRESOURCE_BYTES, icon_names::RESOURCE_PREFIX);

    gtk::init().unwrap();

    let app = RelmApp::new("com.bt.powermenu");
    app.run::<AppModel>(0);
}
