use std::fs;
use std::os::unix::fs::OpenOptionsExt;

use anyhow::Result;

use crate::consts::PASSWORD_STORE_UMASK;
use crate::util;
use crate::PassrsError;

pub fn mv(source: String, dest: String, force: bool) -> Result<()> {
    let source_path = util::canonicalize_path(&source)?;
    let is_file = match fs::metadata(&source_path) {
        Ok(meta) => meta.is_file(),
        Err(_) => false,
    };
    let dest_path = if is_file {
        if dest.ends_with('/') {
            let name = source.rfind('/').unwrap_or(0);
            let oldname = &source[name..];

            util::exact_path(&format!("{}{}.gpg", dest, oldname))?
        } else {
            util::exact_path(&format!("{}.gpg", dest))?
        }
    } else {
        util::exact_path(&dest)?
    };

    if is_file {
        if !util::path_exists(&source_path)? {
            return Err(PassrsError::NotInStore(source).into());
        }

        if !force && util::path_exists(&dest_path)? {
            let prompt = format!("An entry exists for {}. Overwrite it?", dest);

            if util::prompt_yesno(prompt)? {
                fs::OpenOptions::new()
                    .mode(0o666 - (0o666 & *PASSWORD_STORE_UMASK))
                    .write(true)
                    .truncate(true)
                    .open(&dest_path)?;
            } else {
                return Err(PassrsError::UserAbort.into());
            }
        }

        util::copy(&source_path, &dest_path, None)?;
        util::remove_dirs_to_file(&source_path)?;

        if util::get_closest_gpg_id(&dest_path)? != util::get_closest_gpg_id(&source_path)? {
            if dest_path.is_file() {
                util::recrypt_file(&dest_path, None)?;
            } else if dest_path.is_dir() {
                util::recrypt_dir(&dest_path, None)?;
            }
        }

        util::commit(format!("Rename {} to {}", source, dest))?;
    } else {
        if !util::path_exists(&source_path)? {
            return Err(PassrsError::PathDoesntExist(source).into());
        }

        if !force && util::path_exists(&dest_path)? {
            let prompt = format!("An entry exists for {}. Overwrite it?", dest);

            if util::prompt_yesno(prompt)? {
                fs::remove_dir_all(&dest_path)?;
            } else {
                return Err(PassrsError::UserAbort.into());
            }
        }

        util::copy(&source_path, &dest_path, None)?;
        fs::remove_dir_all(&source_path)?;
        util::recrypt_dir(&dest_path, None)?;
        util::commit(format!("Rename {} to {}", source, dest))?;
    }

    Ok(())
}
