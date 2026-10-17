use gpui_kit::{
    AppContext, Context, IntoElement, ParentElement, Render, Styled, Window, WindowOptions,
    application,
    component::button::{Button, ButtonVariants},
    div,
};

struct HelloWorld;

impl Render for HelloWorld {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .items_center()
            .justify_center()
            .gap_2()
            .child("Hello, World!")
            .child(
                Button::new("hello")
                    .primary()
                    .label("Click me")
                    .on_click(|_, _, _| println!("Clicked!")),
            )
    }
}

fn main() {
    application()
        .with_assets(gpui_kit::assets::Assets)
        .run(|cx| {
            gpui_kit::init(cx);

            gpui_kit::open_window(WindowOptions::default(), cx, |_, cx| cx.new(|_| HelloWorld))
                .expect("Failed to open window");
        });
}
