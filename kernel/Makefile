RUSTC_TERGET := i386-nyarn

build: src/*.rs src/*.S ../i386-nyarn.json ./linker.ld
	RUSTFLAGS="-C link-arg=-Tlinker.ld" cargo xbuild --target ../$(RUSTC_TERGET).json --release
	cp target/$(RUSTC_TERGET)/release/kernel ../build/obj/kernel

test:
	RUSTFLAGS="-C link-arg=-Tlinker.ld" cargo xbuild --target ../$(RUSTC_TERGET).json --tests --target-dir target_for_test
	$(eval test_kernel=$(shell ls -t target_for_test/i386-nyarn/debug/ | xargs -0 echo | grep "^kernel-*" | grep -v "\.d" | head -n1))
	cp target_for_test/i386-nyarn/debug/$(test_kernel) ../build/obj/kernel

clean:
	cargo clean
