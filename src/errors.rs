use tokio_timer::TimerError;

use std::io;
use std::sync;
use log;
use nom;
use toml;
use getopts;

error_chain! {
    foreign_links {
        Timer(TimerError);
        IO(io::Error);
        SetLogger(log::SetLoggerError);
        Getopts(getopts::Fail);
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

        TomlParse(errors: Vec<toml::ParserError>) {
            description("parse error")
            display("parse error: {:?}", errors)
        }

        TomlDecode {
            description("decode error")
            display("decode error")
        }

        TomlKey(errors: Vec<toml::ParserError>) {
            description("parse error")
            display("parse error: {:?}", errors)
        }

        Config(path: String) {
            description("error in config")
            display("error in config: {}", path)
        }

        ConfigSection(section: String) {
            description("error in section")
            display("error in section: {}", section)
        }

        ConfigField(field: String, reason: String) {
            description("error in field")
            display("error in field: {}: {}", field, reason)
        }

        MissingPlugin(key: String) {
            description("no such plugin")
            display("no such plugin: {}", key)
        }

        MissingField(name: String) {
            description("missing field")
            display("missing field: {}", name)
        }

        Nom(info: String) {
            description("nom error")
            display("nom error: {}", info)
        }

        Shutdown {
            description("shutdown")
            display("shutdown")
        }

        Poll {
        }

        Update {
        }

        Setup {
        }
    }
}

impl<T> From<sync::PoisonError<T>> for Error {
    fn from(err: sync::PoisonError<T>) -> Error {
        ErrorKind::Poison(err.to_string()).into()
    }
}

impl From<nom::IError> for Error {
    fn from(err: nom::IError) -> Error {
        match err {
            nom::IError::Error(err) => ErrorKind::Nom(err.to_string()).into(),
            nom::IError::Incomplete(_) => ErrorKind::Nom("input incomplete".to_owned()).into(),
        }
    }
}

impl From<toml::DecodeError> for Error {
    fn from(err: toml::DecodeError) -> Error {
        if let Some(ref field) = err.field {
            ErrorKind::ConfigField(field.clone(), format!("{}", err)).into()
        } else {
            ErrorKind::Message(format!("{}", err)).into()
        }
    }
}
