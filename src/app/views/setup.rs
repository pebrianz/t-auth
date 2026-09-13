use crate::app::{common::consts::MAX_PASSWORD_LENGTH, components::*};
use iocraft::prelude::*;
use iocraft_router::Router;
use std::sync::Arc;

#[component]
pub fn Setup(mut hooks: Hooks) -> impl Into<AnyElement<'static>> {
    let mut router = hooks.use_context::<Router>().clone();

    let mut password = hooks.use_state(|| String::from(""));
    let mut show_password = hooks.use_state(|| false);

    hooks.use_local_terminal_events({
        move |event| match event {
            TerminalEvent::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                KeyCode::Char('t') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    show_password.set(!show_password.get());
                }
                KeyCode::Backspace => {
                    let mut string = password.to_string();
                    if !string.is_empty() {
                        string.truncate(string.len() - 1);
                        password.set(string.clone());
                    }
                }
                KeyCode::Enter => {
                    router.navigate("confirm_password", Arc::new(password.to_string()))
                }
                _ => {}
            },
            _ => {}
        }
    });

    let password_string = password.to_string();
    let mut strength_score = 0;

    if password_string.len() >= 8 {
        strength_score += 1;
    };

    let mut has_lower = false;
    let mut has_upper = false;
    let mut has_digit = false;
    let mut has_alpanumeric = false;
    let mut has_non_ascii = false;

    for char in password_string.chars() {
        if char.is_lowercase() {
            has_lower = true;
        } else if char.is_uppercase() {
            has_upper = true;
        } else if char.is_ascii_digit() {
            has_digit = true;
        } else if char.is_alphanumeric() {
            has_alpanumeric = true;
        } else if !char.is_ascii() {
            has_non_ascii = true;
        };
    }

    if has_alpanumeric | has_non_ascii {
        strength_score += 1;
    };
    if has_digit {
        strength_score += 1;
    };
    if has_upper & has_lower {
        strength_score += 1;
    };

    let password_display = if !show_password.get() {
        "*".repeat(password_string.len())
    } else {
        password_string
    };

    element! {
        Fragment {
            Header(content: "Welcome, let's get started")
            Text(content: "📂️ No authenticator vault was found.")
            Text(content: "🔐 Create a master password to secure your 2FA accounts.")
            View(flex_direction: FlexDirection::Column) {
                Text(content: "🔑 New Password: ")
                View(padding_left: Padding::Length(1),border_style: BorderStyle::Custom(BorderCharacters { top_left: '┌', top_right: '┐', bottom_left: '└', bottom_right: '┘', left: '|', right: '|', top: '-', bottom: '-' })) {
                    TextInput(
                        has_focus: true,
                        value: password_display,
                        on_change: move |new_value: String| {
                            let prev_value = password.to_string();
                            if new_value.len() < MAX_PASSWORD_LENGTH as usize && new_value.len() > prev_value.len() {
                                let last_char = new_value.clone().chars().last().unwrap();
                                password.set(prev_value + &last_char.to_string());
                            }
                        }
                    )
                }
                View {
                    Text(content: "▰ ".repeat(strength_score)+ &"▱ ".repeat(4 - strength_score), color: Some(Color::Green))
                    Text(content: match strength_score {
                        4 => " (Very Strong)",
                        3 => " (Strong)",
                        2 => " (Moderate)",
                        _ => " (Weak)"
                    })
                }
            }
            TextBlock(contents: vec![
                String::from("💡 Recommended:"),
                String::from("  ▰ Minimum 8 characters"),
                String::from("  ▰ Uppercase + lowercase"),
                String::from("  ▰ At least one digit"),
                String::from("  ▰ Recommended: non-ASCII or special characters"),
            ])
            Footer {
                Text(content: "[Enter] Create", weight: Weight::Bold, color: Some(Color::Green))
                Text(content: "[Ctrl+T] Toggle visibility", weight: Weight::Bold, color: Some(Color::Yellow))
                Text(content: "[Ctrl+C] Exit", weight: Weight::Bold, color: Some(Color::Red))
            }
        }
    }
}
