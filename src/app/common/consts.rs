pub const UNIX_HOME_VAR: &str = "HOME";
pub const UNIX_APP_DATA_VAR: &str = "XDG_DATA_HOME";
pub const UNIX_APP_DATA_DIR: &str = ".local/share";

pub const WINDOWS_HOME_VAR: &str = "USERPROFILE";
pub const WINDOWS_APP_DATA_VAR: &str = "APPDATA";

pub const APP_DIR: &str = "t-auth";
pub const APP_DATA: &str = ".t-auth.enc";

pub const NONCE_LENGTH: u8 = 12;
pub const SALT_LENGTH: u8 = 22;

pub const MAX_PASSWORD_LENGTH: u8 = 48;
