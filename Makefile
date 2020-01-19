RUSTC_TERGET := i386-nyarn
CARGO_FLAG := --release
CURRENT := $(shell pwd)

.PHONY: build
build:
	mkdir -p build/obj
	mkdir -p build/img
	make -C bootloader build
	make -C kernel build
	dd if=/dev/zero of=build/img/nyarn.img count=10000
	dd if=build/obj/bootloader of=build/img/nyarn.img conv=notrunc
	dd if=build/obj/kernel of=build/img/nyarn.img seek=1 conv=notrunc

test_build:
	cargo xtask test

test: test_build qemu_nox

image:
	sudo docker build -t nyarn docker/

docker_build:
	docker/docker.sh make

docker_test:
	docker/docker.sh make test

login:
	docker/docker.sh /bin/bash

qemu:
	qemu-system-i386 -serial mon:stdio build/img/nyarn.img

qemu_nox:
	qemu-system-i386 -nographic -device isa-debug-exit,iobase=0xf4,iosize=0x04 -serial mon:stdio -display none build/img/nyarn.img || if [ $$? -eq 33 ]; then echo "Success"; fi

run: docker_build qemu

clean:
	rm -r build/
	make -C bootloader clean
	make -C kernel clean
