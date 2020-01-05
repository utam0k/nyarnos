RUSTC_TERGET := i386-nyarn

build: src/*.rs src/*.S ../i386-nyarn.json ./linker.ld
	RUSTFLAGS="-C link-arg=-Tlinker.ld" cargo xbuild --target ../$(RUSTC_TERGET).json --release
	objcopy -O binary -j .text -j .rodata -j .signature ./target/$(RUSTC_TERGET)/release/bootloader ../build/obj/bootloader

clean:
	cargo clean
