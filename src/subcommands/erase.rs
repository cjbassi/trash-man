use std::path::PathBuf;

use crate::common::{pretty_error, prompt_user_for_confirmation};
use crate::TRASH;

pub fn run(files: &[PathBuf], no_confirm: bool) {
    if !no_confirm && !prompt_user_for_confirmation("Permanently erase files?") {
        return;
    }

    files.iter().for_each(|file| {
        if let Err(e) = TRASH.erase_file(file) {
            eprintln!("{}", pretty_error(&e.into()));
        }
    });
}
