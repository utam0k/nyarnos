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

image:
	sudo docker build -t nyarn docker/

docker_build:
	docker/docker.sh make

login:
	docker/docker.sh /bin/bash

qemu:
	qemu-system-i386 -serial mon:stdio build/img/nyarn.img

run: docker qemu

clean:
	rm -r build/
	make -C bootloader clean
	make -C kernel clean
