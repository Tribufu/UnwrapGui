// Copyright (c) Tribufu. All Rights Reserved.
// SPDX-License-Identifier: MIT

use anyhow::Result;
use std::process;

pub trait UnwrapGui<T> {
    fn unwrap_gui(self) -> T;
}

impl<T> UnwrapGui<T> for Option<T> {
    fn unwrap_gui(self) -> T {
        match self {
            Some(value) => value,
            None => show_dialog("Unwrapped an Option that was None"),
        }
    }
}

impl<T> UnwrapGui<T> for Result<T> {
    fn unwrap_gui(self) -> T {
        match self {
            Ok(value) => value,
            Err(e) => show_dialog(e.to_string()),
        }
    }
}

fn show_dialog(error: impl Into<String>) -> ! {
    #[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
    if let Err(_) = native_dialog::MessageDialog::new()
        .set_title("Fatal Error")
        .set_text(&error.into())
        .set_type(native_dialog::MessageType::Error)
        .show_alert()
    {}

    process::exit(0)
}
