use iocraft::prelude::*;

#[component]
pub fn Loading() -> impl Into<AnyElement<'static>> {
    element! {
        View {
            Text(content: "Loading....")
        }
    }
}
