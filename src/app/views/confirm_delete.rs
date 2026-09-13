use crate::app::{
    AppData,
    components::*,
    helpers::create_vault,
    views::{main::AccountData, unlock_app::Data},
};
use argon2::Argon2;
use iocraft::prelude::*;
use iocraft_router::Router;
use std::{sync::Arc, thread};

#[component]
pub fn ConfirmDelete(mut hooks: Hooks) -> impl Into<AnyElement<'static>> {
    let app_data = hooks.use_context::<State<AppData>>().clone();
    let argon2i = hooks.use_context::<Argon2>().clone();
    let mut router = hooks.use_context::<Router>().clone();

    let mut input = hooks.use_state(|| String::from(""));

    let account_data = router.get_state::<AccountData>();

    hooks.use_local_terminal_events({
        let account_data = Arc::clone(&account_data);

        move |event| match event {
            TerminalEvent::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                KeyCode::Backspace => {
                    let mut value = input.to_string();
                    if !value.is_empty() {
                        value.truncate(value.len() - 1);
                        input.set(value.clone());
                    }
                }
                KeyCode::Enter => {
                    if input.to_string() == account_data.name {
                        let mut accounts = account_data.accounts.to_vec();
                        accounts.remove(account_data.selected);
                        let accounts_string = serde_json::to_string(&accounts).unwrap();
                        let argon2i = argon2i.clone();
                        let password = account_data.password.to_string();
                        let mv_accounts_string = accounts_string.clone();
                        thread::spawn(move || {
                            create_vault(
                                argon2i,
                                &password.to_string(),
                                &app_data.read().path,
                                mv_accounts_string.as_bytes(),
                            )
                        });
                        router.navigate(
                            "main",
                            Arc::new(Data {
                                decryption: accounts_string,
                                password: account_data.password.to_string(),
                            }),
                        );
                    }
                }
                KeyCode::Char('q') => router.go_back(),
                _ => {}
            },
            _ => {}
        }
    });

    let account_name = account_data.name.to_string();

    let label = format!("Type account name: \"{account_name}\" to confirm delete account");
    let issuer = account_data.issuer.to_string();

    element! {
        Fragment {
            Header(content: "Confirm Deletion")
            View {
                Text(content: format!("Issuer: {issuer}"))
            }
            Text(content: label)
            View(flex_direction: FlexDirection::Column) {
                View(padding_left: Padding::Length(1), border_style: BorderStyle::Custom(BorderCharacters { top_left: '┌', top_right: '┐', bottom_left: '└', bottom_right: '┘', left: '|', right: '|', top: '-', bottom: '-' })) {
                    TextInput(
                        has_focus: true,
                        value: input.to_string(),
                        on_change: move |new_value: String| {
                            let prev_value = input.to_string();
                            if new_value.len() > prev_value.len() {
                                let last_char = new_value.clone().chars().last().unwrap();
                                input.set(prev_value + &last_char.to_string());
                            }
                        },
                    )
                }
             }
             Footer {
                Text(content: "[Enter] Confirm", weight: Weight::Bold, color: Some(Color::Green))
                Text(content: "[Q] Back", weight: Weight::Bold, color: Some(Color::Red))
             }
         }
    }
}
