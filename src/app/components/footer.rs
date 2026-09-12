use iocraft::prelude::*;

#[derive(Default, Props)]
pub struct FooterProps {
    pub children: Vec<AnyElement<'static>>,
}

#[component]
pub fn Footer(props: &mut FooterProps) -> impl Into<AnyElement<'static>> {
    element! {
        View(flex_direction: FlexDirection::Column) {
            View(flex_wrap: FlexWrap::Wrap, column_gap: Gap::Length(4)) {
                #(props.children.iter_mut())
            }
            View(flex_direction: FlexDirection::Column, border_style: BorderStyle::Custom(BorderCharacters { top_left: '-', top_right: '-', bottom_left: '-', bottom_right: '-', left: '-', right: '-', top: '-', bottom: '-' }), border_edges: Edges::Top) {
                View(justify_content: JustifyContent::SpaceBetween, position: Position::Absolute, width: Size::Percent(100.0)) {
                    Text(content: "⌊")
                    Text(content: "⌋")
                }
                Text(content: "Version: 1.0.0", align: TextAlign::Center)
            }
        }
    }
}
