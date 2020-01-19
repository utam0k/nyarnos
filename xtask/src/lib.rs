mod build;

use anyhow::Context;
pub use anyhow::Result;

use std::{
    env,
    path::{Path, PathBuf},
    process::{Command, Output, Stdio},
};

pub use crate::build::{build_run, test_build_run};

pub fn project_root() -> PathBuf {
    Path::new(
        &env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| env!("CARGO_MANIFEST_DIR").to_owned()),
    )
    .ancestors()
    .nth(1)
    .unwrap()
    .to_path_buf()
}

pub fn run(cmdline: &str, dir: &str) -> Result<()> {
    do_run(cmdline, dir, &mut |c| {
        c.stdout(Stdio::inherit());
    })
    .map(|_| ())
}

pub fn run_with_output(cmdline: &str, dir: &str) -> Result<Output> {
    do_run(cmdline, dir, &mut |_| {})
}

fn do_run(cmdline: &str, dir: &str, f: &mut dyn FnMut(&mut Command)) -> Result<Output> {
    eprintln!("\nwill run: {}", cmdline);
    let proj_dir = project_root().join(dir);
    let mut args = cmdline.split_whitespace();
    let exec = args.next().unwrap();
    let mut cmd = Command::new(exec);
    f(cmd
        .args(args)
        .current_dir(proj_dir)
        .stderr(Stdio::inherit()));
    let output = cmd
        .output()
        .with_context(|| format!("running `{}`", cmdline))?;
    if !output.status.success() {
        anyhow::bail!("`{}` exited with {}", cmdline, output.status);
    }
    Ok(output)
}
