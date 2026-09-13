use crate::app::{
    AppData,
    common::{consts::MAX_PASSWORD_LENGTH, helpers::create_vault},
    components::*,
};
use argon2::Argon2;
use iocraft::prelude::*;
use iocraft_router::*;
use std::{cell::RefCell, thread};

#[component]
pub fn ConfirmPassword(mut hooks: Hooks) -> impl Into<AnyElement<'static>> {
    let app_data = hooks.use_context::<State<AppData>>().clone();
    let argon2i = hooks.use_context::<Argon2>().clone();
    let mut router = hooks.use_context::<Router>().clone();

    let decryption = hooks
        .try_use_context::<Vec<u8>>()
        .unwrap_or(RefCell::new(Vec::new()).borrow())
        .clone();

    let mut input = hooks.use_state(|| String::from(""));
    let mut show_input = hooks.use_state(|| false);
    let mut create_password = hooks.use_state(|| false);

    let password = router.get_state::<String>();

    if create_password.get() {
        {
            let mut router = router.clone();
            let password = password.clone();

            thread::spawn(move || {
                create_vault(
                    argon2i,
                    &password,
                    &app_data.read().path,
                    decryption.as_slice(),
                );
                router.go_to("vault_status");
            });
        }

        router.go_to("loading");
    }

    let input_string = input.to_string();

    let password_display = if !show_input.get() {
        "*".repeat(input_string.len())
    } else {
        input_string
    };

    hooks.use_local_terminal_events(move |event| match event {
        TerminalEvent::Key(key) if key.kind == KeyEventKind::Press => match key.code {
            KeyCode::Char('t') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                show_input.set(!show_input.get())
            }
            KeyCode::Backspace => {
                let mut value = input.to_string();
                if !value.is_empty() {
                    value.truncate(value.len() - 1);
                    input.set(value.clone());
                }
            }
            KeyCode::Enter => {
                if input.to_string() != *password {
                    router.go_to("password_missmatch");
                } else {
                    create_password.set(true);
                }
            }
            _ => {}
        },
        _ => {}
    });

    element! {
        Fragment {
            Header(content: "Confirm Master Password")
            Text(content: "🔒️ Re-enter your password to verify your vault protection.")

            View(flex_direction: FlexDirection::Column) {
                Text(content: "🔑 Password")
                View(padding_left: Padding::Length(1), border_style: BorderStyle::Custom(BorderCharacters { top_left: '┌', top_right: '┐', bottom_left: '└', bottom_right: '┘', left: '|', right: '|', top: '-', bottom: '-' })) {
                    TextInput(
                        has_focus: true,
                        value: password_display.clone(),
                        on_change: move |new_value: String| {
                            let prev_value = input.to_string();
                            if new_value.len() < MAX_PASSWORD_LENGTH as usize && new_value.len() > prev_value.len() {
                                let last_char = new_value.clone().chars().last().unwrap();
                                input.set(prev_value + &last_char.to_string());
                            }
                        },
                    )
                }
             }

             Text(content: "💡 Make sure it matches your previous password.")
             Footer {
                Text(content: "[Enter] Create", weight: Weight::Bold, color: Some(Color::Green))
                Text(content: "[Ctrl+t] Toggle visibility", weight: Weight::Bold, color: Some(Color::Yellow))
                Text(content: "[Ctrl+c] Exit", weight: Weight::Bold, color: Some(Color::Red))
             }
         }
    }
}
