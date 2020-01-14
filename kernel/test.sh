#!/bin/bash

RUSTFLAGS="-C link-arg=-Tlinker.ld" cargo xbuild --target ../i386-nyarn.json --tests --target-dir target_for_test
test_kernel=$(ls -t ./target_for_test/i386-nyarn/debug/ | xargs -0 echo | grep "^kernel-*" | grep -v "\.d" | head -n1)
cp target_for_test/i386-nyarn/debug/$test_kernel ../build/obj/kernel
