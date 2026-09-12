use iocraft::prelude::*;

#[derive(Default, Props)]
pub struct HeaderProps<'a> {
    pub content: &'a str,
}

#[component]
pub fn Header<'a>(props: &mut HeaderProps<'a>) -> impl Into<AnyElement<'static>> {
    element! {
        View {
            View(flex_direction: FlexDirection::Column, position: Position::Absolute, top: Inset::Length(-1)) {
                Text(content: props.content,  weight: Weight::Bold, align: TextAlign::Center)
            }
            View
        }
    }
}
