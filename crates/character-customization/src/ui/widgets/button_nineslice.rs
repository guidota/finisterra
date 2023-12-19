use std::borrow::Cow;

use roma::ui::{
    event::{EventInterest, EventResponse, WidgetEvent},
    input::MouseButton,
    style::*,
    widget::*,
    widgets::*,
    *,
};

/**
A button containing some text.

Responds with [ButtonResponse].

Shorthand:
```rust
# let _handle = yakui_widgets::DocTest::start();
if yakui::button("Hello").clicked {
    println!("The button was clicked");
}
```
*/
#[derive(Debug)]
#[non_exhaustive]
pub struct NineSliceButton {
    pub text: Cow<'static, str>,
    pub padding: Pad,
    pub border_radius: f32,
    pub texture: ManagedTextureId,
    pub style: DynamicButtonStyle,
    pub hover_style: DynamicButtonStyle,
    pub down_style: DynamicButtonStyle,
}

/// Contains styles that can vary based on the state of the button.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct DynamicButtonStyle {
    pub text: TextStyle,
    pub fill: Color,
}

impl Default for DynamicButtonStyle {
    fn default() -> Self {
        let mut text = TextStyle::label();
        text.align = TextAlignment::Center;

        Self {
            text,
            fill: Color::GRAY,
        }
    }
}

impl NineSliceButton {
    pub fn unstyled(text: impl Into<Cow<'static, str>>, texture: ManagedTextureId) -> Self {
        Self {
            text: text.into(),
            padding: Pad::ZERO,
            border_radius: 0.0,
            style: DynamicButtonStyle::default(),
            hover_style: DynamicButtonStyle::default(),
            down_style: DynamicButtonStyle::default(),
            texture,
        }
    }

    pub fn styled(text: impl Into<Cow<'static, str>>, texture: ManagedTextureId) -> Self {
        let style = DynamicButtonStyle {
            fill: colors::BACKGROUND_3,
            ..Default::default()
        };

        let hover_style = DynamicButtonStyle {
            fill: colors::BACKGROUND_3.adjust(1.2),
            ..Default::default()
        };

        let down_style = DynamicButtonStyle {
            fill: colors::BACKGROUND_3.adjust(0.8),
            ..Default::default()
        };

        let mut text_style = TextStyle::label();
        text_style.align = TextAlignment::Center;

        Self {
            text: text.into(),
            padding: Pad::balanced(20.0, 10.0),
            border_radius: 6.0,
            style,
            hover_style,
            down_style,
            texture,
        }
    }

    pub fn show(self) -> Response<NineSliceButtonWidget> {
        roma::ui::util::widget::<NineSliceButtonWidget>(self)
    }
}

#[derive(Debug)]
pub struct NineSliceButtonWidget {
    props: Option<NineSliceButton>,
    hovering: bool,
    mouse_down: bool,
    clicked: bool,
}

#[derive(Debug)]
pub struct ButtonResponse {
    pub hovering: bool,
    pub clicked: bool,
}

impl Widget for NineSliceButtonWidget {
    type Props = NineSliceButton;
    type Response = ButtonResponse;

    fn new() -> Self {
        Self {
            props: None,
            hovering: false,
            mouse_down: false,
            clicked: false,
        }
    }

    fn update(&mut self, props: Self::Props) -> Self::Response {
        self.props = Some(props);

        let mut color = props.style.fill;
        let mut text_style = props.style.text.clone();

        if self.mouse_down {
            let style = &props.down_style;
            color = style.fill;
            text_style = style.text.clone();
        } else if self.hovering {
            let style = &props.hover_style;
            color = style.fill;
            text_style = style.text.clone();
        }

        let alignment = match text_style.align {
            TextAlignment::Start => Alignment::CENTER_LEFT,
            TextAlignment::Center => Alignment::CENTER,
            TextAlignment::End => Alignment::CENTER_RIGHT,
        };

        let mut container = RoundRect::new(props.border_radius);
        container.color = color;
        container.show_children(|| {
            roma::ui::pad(props.padding, || {
                roma::ui::align(alignment, || {
                    let mut text = RenderText::label(props.text.clone());
                    text.style = text_style;
                    text.show();
                });
            });
        });

        let clicked = self.clicked;
        self.clicked = false;

        Self::Response {
            hovering: self.hovering,
            clicked,
        }
    }

    fn event_interest(&self) -> EventInterest {
        EventInterest::MOUSE_INSIDE | EventInterest::MOUSE_OUTSIDE
    }

    fn event(&mut self, _ctx: EventContext<'_>, event: &WidgetEvent) -> EventResponse {
        match event {
            WidgetEvent::MouseEnter => {
                self.hovering = true;
                EventResponse::Sink
            }
            WidgetEvent::MouseLeave => {
                self.hovering = false;
                EventResponse::Sink
            }
            WidgetEvent::MouseButtonChanged {
                button: MouseButton::One,
                down,
                inside,
                ..
            } => {
                if *inside {
                    if *down {
                        self.mouse_down = true;
                        EventResponse::Sink
                    } else if self.mouse_down {
                        self.mouse_down = false;
                        self.clicked = true;
                        EventResponse::Sink
                    } else {
                        EventResponse::Bubble
                    }
                } else {
                    if !*down {
                        self.mouse_down = false;
                    }

                    EventResponse::Bubble
                }
            }
            _ => EventResponse::Bubble,
        }
    }
}
