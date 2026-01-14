mod app;
mod button;
mod atlas;
mod dead_internet;

use clap::Parser;
use iced::{
    Color, Element,
    Length::Fill,
    Subscription, Task, Theme, color,
    theme::{self, Style},
    widget::container,
};
use iced_layershell::{
    reexport::{Anchor, KeyboardInteractivity},
    settings::LayerShellSettings,
    to_layer_message,
};
use tracing::{Level, debug};
use tracing_subscriber::FmtSubscriber;

use crate::app::{REM, SIZE};

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

#[to_layer_message]
#[derive(Debug)]
enum Message {
    App(app::Message),
}

struct LayerApp {
    app: app::App,
}

impl LayerApp {
    fn new(init: app::Init) -> (Self, Task<Message>) {
        let (app, task) = app::App::new(init);
        (Self { app }, task.map(Message::App))
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::App(message) => self.app.update(message).map(Message::App),
            _ => Task::none(),
        }
    }

    fn view(&self) -> Element<'_, Message> {
        container(self.app.view().map(Message::App))
            .style(container::transparent)
            .height(Fill)
            .width(Fill)
            .padding(REM)
            .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        self.app.subscription().map(Message::App)
    }
}

fn main() -> iced_layershell::Result {
    let args = Cli::parse();

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

    let model_init = app::Init::from(args);

    iced_layershell::application(
        move || LayerApp::new(model_init.clone()),
        || "Powermenu".to_string(),
        LayerApp::update,
        LayerApp::view,
    )
    .layer_settings(LayerShellSettings {
        size: Some(SIZE).map(|(w, h)| (w + REM as u32, h + REM as u32)),
        anchor: Anchor::empty(),
        keyboard_interactivity: KeyboardInteractivity::OnDemand,
        ..Default::default()
    })
    .theme(Theme::custom(
        "dead_internet",
        theme::Palette {
            background: color!(0x170f2b),
            text: color!(0xffe3de),
            primary: color!(0xff8bbb),
            success: color!(0xa955e8),
            warning: Color::default(),
            danger: Color::default(),
        },
    ))
    .style(|_layer, theme| Style {
        background_color: Color::TRANSPARENT,
        text_color: theme.palette().text,
    })
    .subscription(LayerApp::subscription)
    .run()
}
