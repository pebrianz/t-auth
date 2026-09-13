use crate::app::{
    Router,
    components::{Footer, Header},
    views::unlock_app::Data,
};
use arboard::Clipboard;
use iocraft::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json;
use std::{sync::Arc, thread, time::Duration};
use totp_rs::Totp;

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct Account {
    pub otpauth: String,
}

#[derive(Default, Props)]
pub struct AccountRowProps {
    values: Vec<String>,
    align: TextAlign,
    selected: usize,
    symbol: bool,
}

#[component]
pub fn AccountRow(props: &AccountRowProps) -> impl Into<AnyElement<'static>> {
    element! {
        View(flex_direction: FlexDirection::Column, border_style:BorderStyle::Single, border_edges: Edges::Right, padding_right: Padding::Length(1), padding_left: Padding::Length(1)) {
            #(props.values.iter().enumerate().map(|(i,value)| {
                let selected = i == props.selected;
                let color = if selected { Color::Green } else { Color::White };
                let symbol = if selected { "▶ " } else {"  "};
                let content = if props.symbol {&format!("{symbol}{value}")} else {value};

                return element! {
                    Text(content: content, align: props.align, color: color)
                }
            }))
        }
    }
}

#[derive(Default, Props)]
pub struct AccountTableProps {
    accounts: Vec<Account>,
}

pub struct AccountData {
    pub selected: usize,
    pub accounts: Arc<Vec<Account>>,
    pub password: String,
    pub name: String,
    pub issuer: String,
}

#[component]
pub fn AccountTable(mut hooks: Hooks, props: &AccountTableProps) -> impl Into<AnyElement<'static>> {
    let mut router = hooks.use_context::<Router>().clone();
    let data = hooks.use_context::<Arc<Data>>().clone();
    let mut update = hooks.use_state(|| false);

    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(1));
            update.set(true);
        }
    });

    let accounts: Vec<Totp> = props
        .accounts
        .iter()
        .map(|account| {
            let totp = Totp::from_url_unchecked(account.otpauth.clone());
            return totp.unwrap();
        })
        .collect();

    let issuers: Vec<String> = accounts
        .iter()
        .map(|account| account.issuer().unwrap_or("").to_string())
        .collect();
    let names: Vec<String> = accounts
        .iter()
        .map(|account| account.account_name().to_string())
        .collect();
    let code: Vec<String> = accounts
        .iter()
        .map(|account| account.generate_current().to_string())
        .collect();

    let ttl: Vec<String> = accounts
        .iter()
        .map(|account| account.ttl().to_string() + "s")
        .collect();

    let mut selected = hooks.use_state(|| 0);

    let names_len = names.len();

    let names_arc = Arc::new(names.clone());
    let code_arc = Arc::new(code.clone());
    let accounts_arc = Arc::new(props.accounts.clone());
    let issuer_arc = Arc::new(issuers.clone());

    hooks.use_terminal_events({
        move |event| match event {
            TerminalEvent::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                KeyCode::Down => {
                    if selected < (names_len - 1) {
                        selected.set(selected + 1);
                    }
                }
                KeyCode::Up => {
                    if selected > 0 {
                        selected.set(selected - 1);
                    }
                }
                KeyCode::Char('c') => {
                    let selected = selected.get();
                    let value = code_arc[selected].clone();
                    thread::spawn(move || {
                        let mut clipboard = Clipboard::new().unwrap();
                        clipboard.set_text(value).unwrap();

                        thread::sleep(Duration::from_secs(1000));
                    });
                }
                KeyCode::Char('d') => {
                    let selected = selected.get();
                    router.navigate(
                        "confirm_delete",
                        Arc::new(AccountData {
                            selected: selected,
                            accounts: accounts_arc.clone(),
                            password: data.password.to_string(),
                            name: names_arc[selected].clone(),
                            issuer: issuer_arc[selected].clone(),
                        }),
                    );
                }
                _ => {}
            },
            _ => {}
        }
    });

    let selected = selected.get();

    element! {
        View(border_style: BorderStyle::Single, border_edges: Edges::Top) {
            AccountRow(values: issuers, selected: selected, symbol: true)
            AccountRow(values: names, selected: selected)
            AccountRow(values: code, selected: selected)
            AccountRow(values: ttl, align: TextAlign::Right, selected: selected)
        }
    }
}

#[component]
pub fn Main(mut hooks: Hooks) -> impl Into<AnyElement<'static>> {
    let mut system = hooks.use_context_mut::<SystemContext>();
    let router = hooks.use_context::<Router>();

    let mut app_exit = hooks.use_state(|| false);

    if app_exit.get() {
        system.exit();
    }

    let data: Arc<Data> = router.get_state::<Data>();
    let string: String = data.decryption.to_string();

    let accounts: Vec<Account> =
        serde_json::from_str::<Vec<Account>>(&string).unwrap_or(Vec::new());

    hooks.use_terminal_events({
        let mut router = router.clone();
        let data = data.clone();
        move |event| match event {
            TerminalEvent::Key(key) if key.kind != KeyEventKind::Release => match key.code {
                KeyCode::Char('a') => router.navigate(
                    "add_account",
                    Arc::new(Data {
                        decryption: string.clone(),
                        password: data.password.clone(),
                    }),
                ),
                KeyCode::Char('q') => app_exit.set(true),
                _ => {}
            },
            _ => {}
        }
    });

    element! {
        Fragment {
            Header(content: "2FA Vault")
            View(flex_direction: FlexDirection::Column) {
                View(justify_content: JustifyContent::SpaceBetween, padding_left: Padding::Length(1), padding_right: Padding::Length(1)) {
                    Text(content: "◈ Accounts")
                    Text(content: "◈ OTP")
                }
                ContextProvider(value: Context::Owned(Box::new(router.clone()))) {
                    ContextProvider(value: Context::Owned(Box::new(data))) {
                        AccountTable(accounts: accounts)
                    }
                }
            }
            Footer {
                Text(content: "[A] Add")
                Text(content: "[D] Delete")
                Text(content: "[C] Copy Code")
                Text(content: "[Q] Exit")
            }
        }
    }
}
