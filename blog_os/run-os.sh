#!/bin/bash
cargo bootimage
qemu-system-x86_64 -drive format=raw,file=target/x86_64_os_spec/debug/bootimage-blog_os.bin
