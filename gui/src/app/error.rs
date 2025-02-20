use std::convert::From;
use std::io::ErrorKind;

use liana::{config::ConfigError, descriptors::LianaDescError};

use crate::{
    app::{settings::SettingsError, wallet::WalletError},
    daemon::DaemonError,
};

#[derive(Debug)]
pub enum Error {
    Config(String),
    Wallet(WalletError),
    Daemon(DaemonError),
    Unexpected(String),
    HardwareWallet(async_hwi::Error),
    Desc(LianaDescError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Config(e) => write!(f, "{}", e),
            Self::Wallet(e) => write!(f, "{}", e),
            Self::Daemon(e) => match e {
                DaemonError::Unexpected(e) => write!(f, "{}", e),
                DaemonError::NoAnswer => write!(f, "Daemon did not answer"),
                DaemonError::DaemonStopped => write!(f, "Daemon stopped"),
                DaemonError::Transport(Some(ErrorKind::ConnectionRefused), _) => {
                    write!(f, "Failed to connect to daemon")
                }
                DaemonError::Transport(kind, e) => {
                    if let Some(k) = kind {
                        write!(f, "{} [{:?}]", e, k)
                    } else {
                        write!(f, "{}", e)
                    }
                }
                DaemonError::Start(e) => {
                    write!(f, "Failed to start daemon: {}", e)
                }
                DaemonError::ClientNotSupported => {
                    write!(f, "Daemon client is not supported")
                }
                DaemonError::Rpc(code, e) => {
                    write!(f, "[{:?}] {}", code, e)
                }
            },
            Self::Unexpected(e) => write!(f, "Unexpected error: {}", e),
            Self::HardwareWallet(e) => write!(f, "{}", e),
            Self::Desc(e) => write!(f, "Liana descriptor error: {}", e),
        }
    }
}

impl From<ConfigError> for Error {
    fn from(error: ConfigError) -> Self {
        Error::Config(error.to_string())
    }
}

impl From<WalletError> for Error {
    fn from(error: WalletError) -> Self {
        Error::Wallet(error)
    }
}

impl From<SettingsError> for Error {
    fn from(error: SettingsError) -> Self {
        Error::Wallet(WalletError::Settings(error))
    }
}

impl From<DaemonError> for Error {
    fn from(error: DaemonError) -> Self {
        Error::Daemon(error)
    }
}

impl From<async_hwi::Error> for Error {
    fn from(error: async_hwi::Error) -> Self {
        Error::HardwareWallet(error)
    }
}
