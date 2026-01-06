mod app;
use relm4::{RelmApp, gtk};

use crate::app::AppModel;

fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    gtk::init().unwrap();

    let app = RelmApp::new("com.bt.powermenu");
    app.run::<AppModel>(0);
}
