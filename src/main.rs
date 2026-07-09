use viewkit::app::{App, ViewContext, WindowOptions};
use viewkit::components::{
    Background, BorderStyle, HStack, Padding, Rectangle, RectangleColor, Spacer, VStack,
};
use viewkit::layout::{StackAlignment, StackGap, ViewExt};
use viewkit::theme::Color;
use viewkit::view::View;

struct TestApp;

impl App for TestApp {
    type Body = Box<dyn View + 'static>;

    fn new() -> Self {
        Self
    }

    fn window(&self) -> WindowOptions {
        WindowOptions::new("ViewKit Test")
            .size(360.0, 260.0)
            .resizable(false)
    }

    fn body(&self, _context: &ViewContext) -> Self::Body {
        Box::new(
            Background::new()
                .background(Rectangle::new().color(RectangleColor::Custom(Color::rgb(18, 24, 38))))
                .content(
                    Padding::all(24.0).content(
                        Background::new()
                            .background(
                                Rectangle::new()
                                    .color(RectangleColor::Custom(Color::rgb(245, 247, 251)))
                                    .border(BorderStyle::custom(Color::rgb(15, 23, 42), 2.0)),
                            )
                            .content(
                                VStack::new()
                                    .alignment(StackAlignment::Stretch)
                                    .gap(StackGap::None)
                                    .child(
                                        Rectangle::new()
                                            .color(RectangleColor::Custom(Color::rgb(44, 96, 210)))
                                            .height(42.0),
                                    )
                                    .child(
                                        Padding::all(24.0).content(
                                            HStack::new()
                                                .alignment(StackAlignment::Start)
                                                .gap(StackGap::Custom(18.0))
                                                .child(
                                                    Rectangle::new()
                                                        .color(RectangleColor::Custom(Color::rgb(
                                                            56, 189, 248,
                                                        )))
                                                        .frame(118.0, 92.0),
                                                )
                                                .child(
                                                    VStack::new()
                                                        .alignment(StackAlignment::Start)
                                                        .gap(StackGap::Custom(14.0))
                                                        .child(
                                                            Rectangle::new()
                                                                .color(RectangleColor::Custom(
                                                                    Color::rgb(34, 197, 94),
                                                                ))
                                                                .frame(128.0, 42.0),
                                                        )
                                                        .child(
                                                            Rectangle::new()
                                                                .color(RectangleColor::Custom(
                                                                    Color::rgb(249, 115, 22),
                                                                ))
                                                                .frame(92.0, 36.0),
                                                        ),
                                                )
                                                .child(Spacer::new()),
                                        ),
                                    )
                                    .child(Spacer::new()),
                            ),
                    ),
                ),
        )
    }
}

fn main() -> Result<(), viewkit::ViewKitError> {
    viewkit::run::<TestApp>()
}
