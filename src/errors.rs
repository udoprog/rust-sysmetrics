use tokio_timer::TimerError;

use std::io;
use std::sync;
use log;
use nom;

error_chain! {
    foreign_links {
        Timer(TimerError);
        IO(io::Error);
        SetLogger(log::SetLoggerError);
    }

    errors {
        Message(msg: String) {
            description("error")
            display("error: {}", msg)
        }

        Poison(msg: String) {
            description("poison error")
            display("poison error: {}", msg)
        }

        ConfigParse(path: String) {
            description("parse error")
            display("parse error: {}", path)
        }

        ConfigKey(key: String) {
            description("missing config key")
            display("missing config key: {}", key)
        }

        MissingPlugin(name: String) {
            description("missing plugin")
            display("missing plugin: {}", name)
        }

        Nom(info: String) {
            description("nom error")
            display("nom error: {}", info)
        }

        Poll {
        }

        Update {
        }

        Setup {
        }
    }
}

impl <T> From<sync::PoisonError<T>> for Error {
    fn from(err: sync::PoisonError<T>) -> Error {
        ErrorKind::Poison(err.to_string()).into()
    }
}

impl From<nom::IError> for Error {
    fn from(err: nom::IError) -> Error {
        match err {
            nom::IError::Error(err) => {
                ErrorKind::Nom(err.to_string()).into()
            },
            nom::IError::Incomplete(_) => {
                ErrorKind::Nom("input incomplete".to_owned()).into()
            }
        }
    }
}
