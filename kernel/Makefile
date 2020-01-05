RUSTC_TERGET := i386-nyarn

build: src/*.rs src/*.S ../i386-nyarn.json ./linker.ld
	RUSTFLAGS="-C link-arg=-Tlinker.ld" cargo xbuild --target ../$(RUSTC_TERGET).json $(CARGO_FLAG) --release
	cp target/$(RUSTC_TERGET)/release/kernel ../build/obj/kernel

clean:
	cargo clean
