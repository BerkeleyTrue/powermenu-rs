use iced::{
    Border, Element,
    Length::Fill,
    Radians, Theme,
    gradient::Linear,
    widget::{Button, button, container, text},
};

pub struct PowerButton<T> {
    pub icon: String,
    pub message: T,
}

impl<T: Clone + 'static> PowerButton<T> {
    pub fn view(&self) -> Element<'static, T> {
        let button = Button::new(text(self.icon.clone()))
            .style(|theme: &Theme, _| {
                let palette = theme.palette();

                button::Style {
                    background: Some(
                        Linear::new(Radians::PI)
                            .add_stop(0.0, palette.text)
                            .add_stop(0.80, palette.text)
                            .add_stop(0.81, palette.primary)
                            .add_stop(0.87, palette.primary)
                            .add_stop(0.88, palette.success)
                            .into(),
                    ),
                    border: Border::default()
                        .rounded(1.0)
                        .color(palette.background)
                        .width(2.0),
                    ..Default::default()
                }
            })
            .on_press(self.message.clone());

        let button_wrapper = container(button)
            .style(|theme: &Theme| {
                let palette = theme.palette();

                container::Style {
                    border: Border::default()
                        .rounded(1.0)
                        .color(palette.text)
                        .width(3.0),
                    ..Default::default()
                }
            })
            .padding(1);

        container(button_wrapper).center_x(Fill).into()
    }
}
