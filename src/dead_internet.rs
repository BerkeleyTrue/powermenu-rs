use iced::{Element, Subscription, Task, advanced::image, time, widget::Space};
use tokio::task;
use tracing::error;

use crate::atlas::AtlasFrame;

const DEAD_INTERNET_BYTES: &[u8] = include_bytes!("../resources/redlotoo_dead-internet-atlas.png");
const FRAMES: u32 = 30;

#[derive(Debug, Clone)]
pub enum Message {
    Tick,
    VideoLoaded(Option<image::Handle>),
}

#[derive(Default)]
pub struct DeadInternet {
    handle: Option<image::Handle>,
    index: u32,
}

impl DeadInternet {
    async fn load_video() -> Message {
        let handle = task::spawn_blocking(|| image::Handle::from_bytes(DEAD_INTERNET_BYTES))
            .await
            .inspect_err(|err| error!("load image err: {err:?}"))
            .ok();
        Message::VideoLoaded(handle)
    }

    pub fn new() -> (Self, Task<Message>) {
        (Self::default(), Task::future(DeadInternet::load_video()))
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Tick => {
                self.index = (self.index + 1) % FRAMES;
            }
            Message::VideoLoaded(handle) => self.handle = handle,
        }
        Task::none()
    }

    pub fn subscriptions(&self) -> Subscription<Message> {
        time::every(time::Duration::from_millis(33)).map(|_| Message::Tick)
    }

    pub fn view(&self) -> Element<'_, Message> {
        match self.handle.as_ref() {
            Some(handle) => AtlasFrame::new(handle.clone(), 8, 910.0, 512.0, self.index).into(),
            None => Space::default().into(),
        }
    }
}
