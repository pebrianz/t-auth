use crate::app::common::{consts::*, utils::*};

use aes_gcm::aead::rand_core;
use argon2::{Argon2, password_hash::SaltString};
use std::{env, fs, path::PathBuf};

pub fn get_app_data_dir() -> PathBuf {
    let app_data_dir = PathBuf::new();

    if cfg!(unix) {
        if let Ok(dir) = env::var(UNIX_APP_DATA_VAR) {
            return app_data_dir.join(dir).join(APP_DIR);
        }

        return app_data_dir
            .join(
                env::var(UNIX_HOME_VAR)
                    .unwrap_or_else(|_| panic!("env variable '{UNIX_HOME_VAR}' should be set")),
            )
            .join(UNIX_APP_DATA_DIR)
            .join(APP_DIR);
    }

    if cfg!(windows) {
        if let Ok(dir) = env::var(WINDOWS_APP_DATA_VAR) {
            return app_data_dir.join(dir).join(APP_DIR);
        }

        return app_data_dir.join(env::var(WINDOWS_HOME_VAR).unwrap());
    }

    panic!("unsupported operating system: cannot determine user home directory.");
}

pub fn create_vault(argon2: Argon2, password: &str, path: &PathBuf, data: &[u8]) {
    let salt = SaltString::generate(&mut rand_core::OsRng);
    let salt = salt.as_str().as_bytes();
    let key = kdf(argon2.clone(), password, salt);
    let (ciphertext, nonce) = encrypt(&key, data);

    fs::write(
        path,
        [nonce.as_slice(), salt, ciphertext.as_slice()].concat(),
    )
    .unwrap_or_else(|_| panic!("failed to write encryption to '{}'", path.display()));
}
