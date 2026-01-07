use relm4::{
    gtk::{self, gio, glib, prelude::*},
    prelude::*,
};
use tracing::error;

// embeds resource
pub const GRESOURCE_BYTES: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/resources.gresource"));

// resource id
pub const RESOURCE_PREFIX: &str = "/com/bt/powermenu-rs";

pub fn init_resources() {
    // wraps the bytes?
    let gbytes = glib::Bytes::from_static(GRESOURCE_BYTES);
    // parses gresource format
    let resource =
        gio::Resource::from_data(&gbytes).expect("Failed to load dead_internet gresouce");
    // register resource globally
    gio::resources_register(&resource);
}

pub struct DeadInternet {
    media: gtk::MediaFile,
}

#[relm4::component(pub)]
impl SimpleComponent for DeadInternet {
    type Init = ();
    type Input = ();
    type Output = ();

    view! {
        gtk::Picture {
            add_css_class: "gif",
            set_paintable: Some(&model.media),
        }
    }

    fn init(
        _init: Self::Init,
        _root: Self::Root,
        _sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let file = gio::File::for_uri(&format!(
            "resource://{}/redlotoo_dead-internet.mp4",
            RESOURCE_PREFIX
        ));

        let media = gtk::MediaFile::for_file(&file);

        // Debug: check for errors
        media.connect_error_notify(|media| {
            if let Some(error) = media.error() {
                error!("MediaFile error: {:?}", error);
            }
        });
        media.set_loop(true);
        media.play();

        let model = DeadInternet { media };
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }
}
