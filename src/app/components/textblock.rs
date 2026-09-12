use iocraft::prelude::*;

#[derive(Default, Props)]
pub struct TextBlockProps {
    pub contents: Vec<String>,
    pub align: TextAlign,
}

#[component]
pub fn TextBlock(props: &TextBlockProps) -> impl Into<AnyElement<'static>> {
    element! {
        View(flex_direction: FlexDirection::Column){
            #(props.contents.iter().map(|content| element! {
                Text(content: content, align: props.align)
            }))
        }
    }
}
