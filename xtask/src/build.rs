use crate::run;
use anyhow::Result;

pub fn build_run() -> Result<()> {
    build_kernel()
}

fn build_kernel() -> Result<()> {
    let target = "i386-nyarn";
    run(
        format!("cargo xbuild --target ../{}.json --release", target).as_str(),
        "kernel/",
    )?;
    run(
        format!("cp target/{}/release/kernel build/obj/kernel", target).as_str(),
        ".",
    )?;
    Ok(())
}
