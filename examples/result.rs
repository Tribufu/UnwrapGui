// Copyright (c) Tribufu. All Rights Reserved.
// SPDX-License-Identifier: MIT

use anyhow::{Error, Result};
use unwrap_gui::*;

fn main() {
    let result: Result<()> = Err(Error::msg(
        "With this trait you can display a message and exit the app.",
    ));

    result.unwrap_gui();
}
