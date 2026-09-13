mod common;
mod components;
mod views;

use argon2::{Algorithm, Argon2, Params, Version};
use common::{consts, helpers};
use components::Loading;
use iocraft::prelude::*;
use iocraft_router::*;
use std::{fs, path::PathBuf, thread};
use views::*;

#[derive(Clone, Default)]
pub struct AppData {
    path: PathBuf,
    encryption: Option<Vec<u8>>,
}

#[component]
pub fn App(mut hooks: Hooks) -> impl Into<AnyElement<'static>> {
    let mut system = hooks.use_context_mut::<SystemContext>();
    let (w, _h) = hooks.use_terminal_size();
    let w = w - 4;

    let mut app_data = hooks.use_state(|| AppData::default());
    let mut app_exit = hooks.use_state(|| false);
    let mut loading = hooks.use_state(|| true);

    let argon2i =
        hooks.use_const(|| Argon2::new(Algorithm::Argon2i, Version::V0x13, Params::DEFAULT));

    hooks.use_effect(
        move || {
            thread::spawn(move || {
                let app_data_dir = helpers::get_app_data_dir();
                fs::create_dir_all(&app_data_dir).unwrap();

                let app_data_path = app_data_dir.join(consts::APP_DATA);
                let encryption = fs::read(&app_data_path).ok();

                app_data.set(AppData {
                    encryption,
                    path: app_data_path,
                });

                loading.set(false);
            });
        },
        (),
    );

    if app_exit.get() {
        system.exit();
    }

    hooks.use_terminal_events({
        move |event| match event {
            TerminalEvent::Key(key) if key.kind != KeyEventKind::Release => {
                if key.code == KeyCode::Esc {
                    app_exit.set(true);
                }
            }
            _ => {}
        }
    });

    let default_router = if app_data.read().encryption.is_some() {
        "unlock_app"
    } else {
        "setup"
    };

    if loading.get() {
        return element! {View {Loading}};
    }

    element!(
        View(width: Size::Length(w.into()), display: Display::Flex, justify_content: JustifyContent::Center) {
            View(padding_right: Padding::Length(1), padding_left: Padding::Length(1), border_style: BorderStyle::Single){
                ContextProvider(value: Context::Owned(Box::new(app_data))){
                    ContextProvider(value: Context::Owned(Box::new(argon2i))) {
                        View(flex_direction: FlexDirection::Column, gap: Gap::Length(1)) {
                            RouterProvider(default: default_router, routes: vec![
                                Route("unlock_app", || element! {UnlockApp}.into()),
                                Route("confirm_password", || element! {ConfirmPassword}.into()),
                                Route("loading", || element! {Loading}.into()),
                                Route("main", || element! {Main}.into()),
                                Route("password_missmatch", || element! {PasswordMissmatch}.into()),
                                Route("setup", || element! {Setup}.into()),
                                Route("vault_status", || element! {VaultStatus}.into()),
                                Route("add_account", || element! {AddAccount}.into()),
                                Route("confirm_delete", || element! {ConfirmDelete}.into())
                            ]
                        )}
                    }
                }
            }
        }
    )
}
