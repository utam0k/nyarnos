use crate::run;
use anyhow::{anyhow, Result};
use std::{fs, io};

const TARGET: &str = "i386-nyarn";

pub fn build_run() -> Result<()> {
    prepare()?;
    build_kernel()?;
    run("dd if=/dev/zero of=build/img/nyarn.img count=10000", ".")?;
    run(
        "dd if=build/obj/bootloader of=build/img/nyarn.img conv=notrunc",
        ".",
    )?;
    run(
        "dd if=build/obj/kernel of=build/img/nyarn.img seek=1 conv=notrunc",
        ".",
    )?;
    Ok(())
}

pub fn test_build_run() -> Result<()> {
    prepare()?;
    build_for_test()?;
    run("dd if=/dev/zero of=build/img/nyarn.img count=10000", ".")?;
    run(
        "dd if=build/obj/bootloader of=build/img/nyarn.img conv=notrunc",
        ".",
    )?;
    run(
        "dd if=build/obj/kernel of=build/img/nyarn.img seek=1 conv=notrunc",
        ".",
    )?;
    Ok(())
}

fn prepare() -> Result<()> {
    fs::create_dir_all("build/obj/")?;
    fs::create_dir_all("build/img/")?;
    build_bootloader()?;
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

fn is_kernel_bin(entry: &fs::DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("kernel-") && !s.ends_with(".d"))
        .unwrap_or(false)
}

fn build_for_test() -> Result<()> {
    run(
        format!("cargo xbuild --target ../{}.json --tests", TARGET).as_str(),
        "kernel",
    )?;

    let mut entries = fs::read_dir(format!("target/{}/debug", TARGET))?
        .filter(|res| res.as_ref().map(|e| is_kernel_bin(e)).unwrap_or(false))
        .collect::<Result<Vec<_>, io::Error>>()?;

    // TODO: remove unwrap().
    entries.sort_by(|e1, e2| {
        e1.metadata()
            .unwrap()
            .modified()
            .unwrap()
            .cmp(&e2.metadata().unwrap().modified().unwrap())
    });

    if let Some(target_kernel) = entries.first() {
        run(
            format!("cp {} build/obj/kernel", target_kernel.path().display()).as_str(),
            ".",
        )?;
    } else {
        return Err(anyhow!("Not found kernel."));
    }

    Ok(())
}
