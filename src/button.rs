use iced::{
    Element,
    Length::Fill,
    widget::{button, container, text},
};

pub struct PowerButton<T> {
    pub icon: String,
    pub message: T,
}

impl<T: Clone + 'static> PowerButton<T> {
    pub fn view(&self) -> Element<'static, T> {
        container(button(text(self.icon.clone())).on_press(self.message.clone()))
            .center_x(Fill)
            .into()
    }
}
