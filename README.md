# Ferrox

The port of the 1000 lines of OS code using Rust.
Just for fun.

# Freestanding Rust

Meaning do not link to standard library.

Followed the instructions here.

https://os.phil-opp.com/freestanding-rust-binary/


# Sources

The inspiration [OS in 1,000 lines](https://operating-system-in-1000-lines.vercel.app/en/)
[Rust port](https://github.com/lapla-cogito/WritingOS1000lines-Rust/)

## Rust stuffs

Add the riscv32i target to your rust toolchain

```bash
rustup target add riscv32i-unknown-none-elf
```

## Build

The project comes with the default cargo config, located in `.cargo/config.toml`

This saves us time from specifying the target every time we build.  

Simply run:

```bash
cargo build
```

## Starting the kernel

The `run.sh` is using the debug build of the kernel.

```bash
$ ./run.sh

➜  rust-os git:(main) ✗ ./run.sh 
+ QEMU=qemu-system-riscv32
+ qemu-system-riscv32 -machine virt -bios default -nographic -serial mon:stdio --no-reboot -kernel ./target/riscv32i-unknown-none-elf/debug/rust-os

OpenSBI v1.2
   ____                    _____ ____ _____
  / __ \                  / ____|  _ \_   _|
 | |  | |_ __   ___ _ __ | (___ | |_) || |
 | |  | | '_ \ / _ \ '_ \ \___ \|  _ < | |
 | |__| | |_) |  __/ | | |____) | |_) || |_
  \____/| .__/ \___|_| |_|_____/|____/_____|
        | |
        |_|

Platform Name             : riscv-virtio,qemu
Platform Features         : medeleg
Platform HART Count       : 1
Platform IPI Device       : aclint-mswi
Platform Timer Device     : aclint-mtimer @ 10000000Hz
Platform Console Device   : uart8250
Platform HSM Device       : ---
Platform PMU Device       : ---
Platform Reboot Device    : sifive_test
Platform Shutdown Device  : sifive_test
Firmware Base             : 0x80000000
Firmware Size             : 208 KB
Runtime SBI Version       : 1.0

Domain0 Name              : root
Domain0 Boot HART         : 0
Domain0 HARTs             : 0*
Domain0 Region00          : 0x02000000-0x0200ffff (I)
Domain0 Region01          : 0x80000000-0x8003ffff ()
Domain0 Region02          : 0x00000000-0xffffffff (R,W,X)
Domain0 Next Address      : 0x80200000
Domain0 Next Arg1         : 0x87e00000
Domain0 Next Mode         : S-mode
Domain0 SysReset          : yes

Boot HART ID              : 0
Boot HART Domain          : root
Boot HART Priv Version    : v1.12
Boot HART Base ISA        : rv32imafdch
Boot HART ISA Extensions  : time,sstc
Boot HART PMP Count       : 16
Boot HART PMP Granularity : 4
Boot HART PMP Address Bits: 32
Boot HART MHPM Count      : 16
Boot HART MIDELEG         : 0x00001666
Boot HART MEDELEG         : 0x00f0b509


Hello World!
QEMU 8.2.2 monitor - type 'help' for more information
(qemu) q
➜  rust-os git:(main) ✗ ./run.sh
+ QEMU=qemu-system-riscv32
+ qemu-system-riscv32 -machine virt -bios default -nographic -serial mon:stdio --no-reboot -kernel ./target/riscv32i-unknown-none-elf/debug/rust-os

OpenSBI v1.2
   ____                    _____ ____ _____
  / __ \                  / ____|  _ \_   _|
 | |  | |_ __   ___ _ __ | (___ | |_) || |
 | |  | | '_ \ / _ \ '_ \ \___ \|  _ < | |
 | |__| | |_) |  __/ | | |____) | |_) || |_
  \____/| .__/ \___|_| |_|_____/|____/_____|
        | |
        |_|

Platform Name             : riscv-virtio,qemu
Platform Features         : medeleg
Platform HART Count       : 1
Platform IPI Device       : aclint-mswi
Platform Timer Device     : aclint-mtimer @ 10000000Hz
Platform Console Device   : uart8250
Platform HSM Device       : ---
Platform PMU Device       : ---
Platform Reboot Device    : sifive_test
Platform Shutdown Device  : sifive_test
Firmware Base             : 0x80000000
Firmware Size             : 208 KB
Runtime SBI Version       : 1.0

Domain0 Name              : root
Domain0 Boot HART         : 0
Domain0 HARTs             : 0*
Domain0 Region00          : 0x02000000-0x0200ffff (I)
Domain0 Region01          : 0x80000000-0x8003ffff ()
Domain0 Region02          : 0x00000000-0xffffffff (R,W,X)
Domain0 Next Address      : 0x80200000
Domain0 Next Arg1         : 0x87e00000
Domain0 Next Mode         : S-mode
Domain0 SysReset          : yes

Boot HART ID              : 0
Boot HART Domain          : root
Boot HART Priv Version    : v1.12
Boot HART Base ISA        : rv32imafdch
Boot HART ISA Extensions  : time,sstc
Boot HART PMP Count       : 16
Boot HART PMP Granularity : 4
Boot HART PMP Address Bits: 32
Boot HART MHPM Count      : 16
Boot HART MIDELEG         : 0x00001666
Boot HART MEDELEG         : 0x00f0b509


Hello World!
QEMU 8.2.2 monitor - type 'help' for more information
(qemu) q
```
