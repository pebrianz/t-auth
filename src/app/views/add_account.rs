use crate::app::{
    AppData,
    components::{Footer, Header, TextBlock},
    helpers::create_vault,
    views::{main::Account, unlock_app::Data},
};
use arboard::Clipboard;
use argon2::Argon2;
use image;
use iocraft::prelude::*;
use iocraft_router::Router;
use std::{sync::Arc, thread};
use xcap::Monitor;

#[component]
pub fn AddAccount(mut hooks: Hooks) -> impl Into<AnyElement<'static>> {
    let app_data = hooks.use_context::<State<AppData>>().clone();
    let argon2i = hooks.use_context::<Argon2>().clone();
    let router = hooks.use_context::<Router>();
    let mut otpauth = hooks.use_state(|| String::from(" ◌ Waiting for otpauth url...  "));

    let mut router = router.clone();
    hooks.use_terminal_events({
        move |event| match event {
            TerminalEvent::Key(key) if key.kind != KeyEventKind::Release => match key.code {
                KeyCode::Char('s') => {
                    let monitors = Monitor::all().unwrap();

                    let screenshot = monitors[0].capture_image().unwrap();
                    let gray = image::DynamicImage::ImageRgba8(screenshot).into_luma8();
                    let mut prepared = rqrr::PreparedImage::prepare(gray);
                    let grids = prepared.detect_grids();

                    if grids.len() > 0 {
                        let (_meta, content) = grids[0].decode().unwrap();
                        if content.starts_with("otpauth") {
                            otpauth.set(content);
                        }
                    }
                }
                KeyCode::Char('p') => {
                    let mut clipboard = Clipboard::new().unwrap();

                    let text = clipboard.get_text().unwrap();
                    if text.starts_with("outpauth") {
                        otpauth.set(text)
                    }
                }
                KeyCode::Enter => {
                    let otpauth = otpauth.to_string();
                    if otpauth.starts_with("otpauth") {
                        let data = router.get_state::<Data>();
                        let string: String = data.decryption.to_string();
                        let mut accounts: Vec<Account> =
                            serde_json::from_str::<Vec<Account>>(&string).unwrap_or(Vec::new());

                        accounts.push(Account { otpauth: otpauth });
                        let string: String = serde_json::to_string(&accounts).unwrap();
                        let mvstring = string.clone();
                        let argon2i = argon2i.clone();
                        let mvdata = data.clone();
                        thread::spawn(move || {
                            create_vault(
                                argon2i,
                                &mvdata.password.to_string(),
                                &app_data.read().path,
                                &mvstring.as_bytes(),
                            )
                        });
                        router.navigate(
                            "main",
                            Arc::new(Data {
                                decryption: string,
                                password: data.password.to_string(),
                            }),
                        );
                    }
                }
                KeyCode::Char('q') => {
                    router.go_back();
                }
                _ => {}
            },
            _ => {}
        }
    });

    element! {
        Fragment {
            Header(content: "Add Account")
            View(padding_left: Padding::Length(1), border_style: BorderStyle::Custom(BorderCharacters { top_left: '┌', top_right: '┐', bottom_left: '└', bottom_right: '┘', left: '|', right: '|', top: '-', bottom: '-' })) {
                Text(content: otpauth.to_string())
            }
            TextBlock(contents: vec![
                String::from("[S] Scan QR code in current monitor"),
                String::from("[P] Paste otpauth URI"),
            ])
            Footer {
                Text(content: "<Enter> Add", weight: Weight::Bold, color: Some(Color::Green))
                Text(content: "[Q] Back", weight: Weight::Bold, color: Some(Color::Red))
            }
        }
    }
}
