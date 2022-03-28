// Copyright (c) The diem-devtools Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

use clap::Parser;
use nextest_runner::dispatch::Opts;

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    opts.exec()
}
