use crate::app::{
    AppData, Router,
    common::{
        consts,
        utils::{decrypt, kdf},
    },
    components::*,
};
use argon2::Argon2;
use iocraft::prelude::*;
use std::{sync::Arc, thread};

#[derive(Clone)]
pub struct Data {
    pub decryption: String,
    pub password: String,
}

#[component]
pub fn UnlockApp(mut hooks: Hooks) -> impl Into<AnyElement<'static>> {
    let router = hooks.use_context::<Router>();
    let app_data = hooks.use_context::<State<AppData>>().clone();
    let argon2i = hooks.use_context::<Argon2>().clone();

    let mut input = hooks.use_state(|| String::from(""));
    let mut unlock = hooks.use_state(|| false);

    if unlock.get() {
        {
            let mut router = router.clone();
            thread::spawn(move || {
                let encryption = app_data.read().clone().encryption.unwrap();
                let nonce_length = consts::NONCE_LENGTH;
                let salt_index_end = nonce_length + consts::SALT_LENGTH;

                let nonce = &encryption[..nonce_length.into()];
                let salt = &encryption[nonce_length.into()..salt_index_end.into()];
                let ciphertext = &encryption[salt_index_end.into()..encryption.len()];

                let key = kdf(argon2i, &input.to_string(), salt);
                let decryption = decrypt(&key, nonce, ciphertext);

                if decryption.is_none() {
                    router.go_to("password_missmatch");
                    unlock.set(false);
                } else {
                    router.navigate(
                        "main",
                        Arc::new(Data {
                            decryption: String::from_utf8(decryption.unwrap()).unwrap(),
                            password: input.to_string(),
                        }),
                    );
                }
            });
        }
        return element! {View {Loading}};
    }

    hooks.use_local_terminal_events({
        move |event| match event {
            TerminalEvent::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                KeyCode::Backspace => {
                    let mut string = input.to_string();
                    if !string.is_empty() {
                        string.truncate(string.len() - 1);
                        input.set(string.clone());
                    }
                }
                KeyCode::Enter => unlock.set(true),
                _ => {}
            },
            _ => {}
        }
    });

    element! {
        View(
            flex_direction: FlexDirection::Column,
            gap: Gap::Length(1),
            width: Percent(100.0),
        ) {
            Header(content: "Unlock App")
            View(flex_direction: FlexDirection::Column) {
                Text(content: "🔑️ Enter your master password.")
                View(padding_left: Padding::Length(1), border_style: BorderStyle::Custom(BorderCharacters { top_left: '┌', top_right: '┐', bottom_left: '└', bottom_right: '┘', left: '|', right: '|', top: '-', bottom: '-' })) {
                    TextInput(
                        has_focus: true,
                        on_change: move |value: String| {
                            let prev_value = input.to_string();
                            if value.len() > prev_value.len() {
                                let last_char = value.clone().chars().last().unwrap();
                                input.set(prev_value + &last_char.to_string());
                            }
                        },
                        value: "*".repeat(input.to_string().len()),
                    )
                }
            }
            Footer {
                Text(content: "[Enter] Unlock", weight: Weight::Bold, color: Some(Color::Green))
            }
        }
    }
}
