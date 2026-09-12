use crate::app::{Router, components::*};
use iocraft::prelude::*;

#[component]
pub fn PasswordMissmatch(mut hooks: Hooks) -> impl Into<AnyElement<'static>> {
    let mut router = hooks.use_context::<Router>().clone();

    hooks.use_local_terminal_events({
        move |event| match event {
            TerminalEvent::Key(key) if key.kind == KeyEventKind::Press => {
                if key.code == KeyCode::Enter {
                    router.go_back()
                }
            }
            _ => {}
        }
    });

    element! {
        Fragment {
            Header(content: "Password Mismatch")
            Text(content: "The passwords do not match. Please try again")
            Footer {
                Text(content: "[Enter] Retry", weight: Weight::Bold, color: Some(Color::Green))
            }
        }
    }
}
