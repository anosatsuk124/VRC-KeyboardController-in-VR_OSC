#![allow(dead_code)]

use std::{borrow::Cow, process::Command};

#[derive(Clone, Debug)]
pub struct GlobalCommand<'a> {
    name: Cow<'a, str>,
    pub args: Option<Vec<Cow<'a, str>>>,
}

impl<'a> GlobalCommand<'a> {
    pub fn new(name: Cow<'a, str>) -> Self {
        Self {
            name: Cow::from(name),
            args: None,
        }
    }

    pub fn build(&self) -> Command {
        let mut command = std::process::Command::new(self.name.as_ref());
        if let Some(args) = &self.args {
            command.args(args.iter().map(|arg| arg.as_ref()));
        }
        command
    }
}

/// Create a global command with the given name and it will invoke the lowercase of it.
macro_rules! global_command {
    ($command:ident) => {
        pub static $command: once_cell::sync::Lazy<GlobalCommand> =
            once_cell::sync::Lazy::new(|| {
                GlobalCommand::new(std::borrow::Cow::from(stringify!($command).to_lowercase()))
            });
    };
    ($command_name:expr, $command:ident) => {
        pub static $command: once_cell::sync::Lazy<GlobalCommand> =
            once_cell::sync::Lazy::new(|| GlobalCommand::new($command_name));
    };
}

global_command!(CARGO);

global_command!(
    Cow::from(std::env::var("CONTAINER_RUNTIME").unwrap_or_else(|_| "docker".to_string())),
    CONTAINER_RUNTIME
);
