use crate::run;
use std::{
    fs::create_dir_all,
};
use anyhow::Result;

const TARGET: &str = "i386-nyarn";

pub fn build_run() -> Result<()> {
    create_dir_all("build/obj/")?;
    create_dir_all("build/img/")?;
    build_bootloader()?;
    build_kernel()?;
    run("dd if=/dev/zero of=build/img/nyarn.img count=10000", ".")?;
    run("dd if=build/obj/bootloader of=build/img/nyarn.img conv=notrunc", ".")?;
    run("dd if=build/obj/kernel of=build/img/nyarn.img seek=1 conv=notrunc", ".")?;
    Ok(())
}

fn build_bootloader() -> Result<()> {
    run(
        format!("cargo xbuild --target ../{}.json --release", TARGET).as_str(),
        "bootloader",
    )?;
    run(
        format!("objcopy -O binary -j .text -j .rodata -j .signature ./target/{}/release/bootloader ../build/obj/bootloader", TARGET).as_str(),
        "bootloader",
    )?;
    Ok(())
}

fn build_kernel() -> Result<()> {
    run(
        format!("cargo xbuild --target ../{}.json --release", TARGET).as_str(),
        "kernel",
    )?;
    run(
        format!("cp target/{}/release/kernel build/obj/kernel", TARGET).as_str(),
        ".",
    )?;
    Ok(())
}
