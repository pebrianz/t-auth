use crate::app::{components::*, consts, helpers};
use iocraft::prelude::*;
use std::{fs, thread};

#[component]
pub fn VaultStatus(mut hooks: Hooks) -> impl Into<AnyElement<'static>> {
    let mut system = hooks.use_context_mut::<SystemContext>();
    let mut exit = hooks.use_state(|| false);
    let mut loading = hooks.use_state(|| true);
    let mut vault_path = hooks.use_state(|| String::from(""));
    let mut status = hooks.use_state(|| String::from(""));

    hooks.use_local_terminal_events({
        move |event| match event {
            TerminalEvent::Key(key) if key.kind == KeyEventKind::Press => {
                if key.code == KeyCode::Enter {
                    exit.set(true);
                }
            }
            _ => {}
        }
    });

    if exit.get() {
        system.exit();
    }

    hooks.use_effect(
        move || {
            thread::spawn(move || {
                let app_data_dir = helpers::get_app_data_dir();
                fs::create_dir_all(&app_data_dir).unwrap();

                let app_data_path = app_data_dir.join(consts::APP_DATA);
                let encryption = fs::read(&app_data_path).ok();

                if encryption.is_some() {
                    status.set(String::from("Created"))
                } else {
                    status.set(String::from("Not Found"))
                }

                vault_path.set(app_data_path.to_string_lossy().to_string());

                loading.set(false);
            });
        },
        (),
    );

    if loading.get() {
        return element! {Fragment{Loading}};
    }

    let vault_path = vault_path.read().to_string();
    let status = status.read().to_string();

    element! {
        Fragment {
            Header(content: "Vault Status")
            View(flex_direction: FlexDirection::Column) {
                View(border_edges: Edges::Bottom, border_style: BorderStyle::Single){
                    Text(content: "📂️ Vault File")
                }
                TextBlock(contents: vec![
                    format!("Status: {status}"),
                    format!("Location: {vault_path} ")
                ])
            }
            Footer {
                Text(content: "[Enter] Exit", weight: Weight::Bold, color: Some(Color::Green))
            }
        }
    }
}
