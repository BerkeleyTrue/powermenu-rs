use relm4::{ComponentParts, ComponentSender, RelmApp, SimpleComponent, gtk::{self, prelude::{GtkWindowExt, OrientableExt}}};

struct AppModel {}

#[derive(Debug)]
enum AppMessage {}

#[relm4::component]
impl SimpleComponent for AppModel {
    type Init = u8;
    type Input = AppMessage;
    type Output = ();
    view! {
        gtk::Window {
            set_title: Some("PowerMenu"),
            set_default_width: 300,
            set_default_height: 300,

            gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                gtk::Label {
                    set_label: "Hello World",
                }
            }
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = AppModel {};
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }
}

fn main() {
    // gtk::init().unwrap();

    let app = RelmApp::new("com.bt.powermenu");
    app.run::<AppModel>(0);
}
