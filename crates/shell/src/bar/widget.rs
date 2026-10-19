use gpui_kit::{AnyElement, App, CursorStyle, ElementId, Window, div, prelude::*, white};

#[derive(IntoElement)]
pub struct BarWidget {
    id: ElementId,
    children: Vec<AnyElement>,
}

impl BarWidget {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            children: Vec::new(),
        }
    }
}

impl RenderOnce for BarWidget {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .id(self.id)
            .px_2()
            .h_full()
            .rounded_full()
            .hover(|r| r.bg(gpui_kit::white().opacity(0.2)))
            .cursor(CursorStyle::PointingHand)
            .children(self.children)
    }
}

impl ParentElement for BarWidget {
    fn extend(&mut self, elements: impl IntoIterator<Item = gpui_kit::AnyElement>) {
        self.children.extend(elements);
    }
}
