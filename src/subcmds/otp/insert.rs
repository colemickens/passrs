use std::fs;
use std::io::{self, Write};
use std::os::unix::fs::OpenOptionsExt;

use anyhow::Result;
use termion::input::TermRead;

use crate::subcmds::otp::validate;
use crate::util;
use crate::PassrsError;

// TODO: pass otp insert -e -i goo => insert otpauth:// URI
// Insert into <label>.gpg
// secret => only ask for secret (don't need full URI)
//   also requires --issuer or --account
// if pass_name is not specified, convert issuer:accountname URI label to a path
// in the form of issuer/accountname

// 1. pass_name becomes optional
// 2. from_secret becomes a bool (whether or not the user will provide the full uri or just the secret)
// 3. issuer is a string and required when `from_secret` is true, part of the label
// 4. account is a string and required when `from_secret` is true if issuer is not specified, part of the label

// if from_secret, issuer
pub fn insert(force: bool, echo: bool, secret_name: String, from_secret: bool) -> Result<()> {
    let path = util::canonicalize_path(&secret_name)?;

    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    if !force && util::path_exists(&path)? {
        write!(
            stdout,
            "An entry exists for {}. Overwrite it? [y/N] ",
            secret_name
        )?;
        io::stdout().flush()?;

        match stdin.read_line()? {
            Some(reply)
                if reply.chars().nth(0) == Some('y') || reply.chars().nth(0) == Some('Y') =>
            {
                fs::OpenOptions::new()
                    .mode(0o600)
                    .write(true)
                    .truncate(true)
                    .open(&path)?;
            }
            _ => return Err(PassrsError::UserAbort.into()),
        }
    }

    if from_secret {
        let secret = util::prompt_for_secret(echo, &secret_name)?;

        // if we prompted the user for a secret and got one
        if let Some(secret) = secret {
            let secret = format!("otpauth://totp/{}?secret={}", secret_name, secret);
            validate::validate(&secret)?;
            util::encrypt_bytes_into_file(secret.as_bytes(), path)?;
        }
    } else {
        let secret = util::prompt_for_secret(echo, &secret_name)?;

        if let Some(secret) = secret {
            validate::validate(&secret)?;
            util::encrypt_bytes_into_file(secret.as_bytes(), path)?;
        }
    }

    util::commit(format!("Add OTP secret for {} to store", secret_name))?;

    Ok(())
}
