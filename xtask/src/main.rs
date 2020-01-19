//! This binary is integrated into the `cargo` command line by using an alias in
//! `.cargo/config`.

use pico_args::Arguments;
use xtask::{build_run, test_build_run, Result};

fn main() -> Result<()> {
    let mut args = Arguments::from_env();
    let subcommand = args.subcommand()?.unwrap_or_default();

    match subcommand.as_str() {
        "build" => {
            args.finish()?;
            build_run()?;
            Ok(())
        }
        "test" => {
            args.finish()?;
            test_build_run()?;
            Ok(())
        }
        _ => {
            eprintln!(
                "\
cargo xtask
Run custom build command.

USAGE:
    cargo xtask <SUBCOMMAND>

SUBCOMMANDS:
    build
    test"
            );
            Ok(())
        }
    }
}
