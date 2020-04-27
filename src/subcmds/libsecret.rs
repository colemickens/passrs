extern crate dbus;

use anyhow::Result;
use dbus::blocking::LocalConnection;
use dbus::tree::Factory;

use std::error::Error;
use std::io::{self, Write};
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use crate::consts::STORE_STRING;
use crate::libsecretdbus::*;
use crate::tree;
use crate::PassrsError;

pub(crate) fn libsecret() -> Result<()> {
    // start a daemon
    // list of requests over libdbus / libsecret protocol
    // r/w to `libsecret/<secret-name>`
    // Let's start by starting up a connection to the session bus and request a name.

    writeln!(
        io::stdout(),
        "TODO: actually run the libsecret dbus provider..."
    )?;

    // TODO: see here: https://github.com/diwic/dbus-rs/blob/master/dbus-codegen/examples/adv_server_codegen.rs

    writeln!(io::stdout(), "Running libsecret dbus provider...")?;

    Ok(())
}
