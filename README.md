# stm32ral

This project provides a Rust RAL (register access layer) for all STM32
microcontrollers.

The underlying data is generated via the SVD files in
[stm32-rs](https://github.com/adamgreig/stm32-rs).

[Documentation](https://docs.rs/stm32ral)

[Repository](https://github.com/adamgreig/stm32ral)


## Quick Example

```rust
use stm32ral::stm32f405::{rcc, gpio};

// Field-level read/modify/write, with either named values or just literals
modify_reg!(rcc, RCC.ahb1enr, GPIOAEN: Enabled);
modify_reg!(gpio, GPIOA.moder, MODER1: Input, MODER2: Output, MODER3: Input);
while read_reg!(gpio, GPIOA.idr, IDR3 == High) {
    let pa1 = read_reg!(gpio, GPIOA.idr, IDR1);
    modify_reg!(gpio, GPIOA.odr, ODR2: pin);
}

// Whole-register read/modify/write
let port = read_reg!(gpio, GPIOA.idr);
write_reg!(gpio, GPIOA.odr, 0x12345678);
modify_reg!(gpio, GPIOA.moder, |r| r | (0b10 << 4));

// Or forego the macros and just use the constants yourself
let port = gpio::GPIOA.idr.read();
gpio::GPIOA.odr.write(0x12345678);
```

## Why Use stm32ral?

* Small and lightweight
* Simple
* Quick to compile
* Covers all STM32 devices in one crate
* Doesn't get in your way
* A bit like what you're used to from C header files

## Why not use stm32ral?

* Not nearly as much safety as leading competitors
* You need to fill up your disks
* A bit like what you're used to from C header files

## Instead, consider using...

* [svd2rust](https://github.com/japaric/svd2rust) is the obvious choice for
  generating Cortex-M device crates from SVD files
* [stm32-rs](https://github.com/adamgreig/stm32-rs) provides `svd2rust` crates
  for all STM32 devices supported by this crate, and they use the same
  underlying data
* [TockOS](https://www.tockos.org/blog/2018/mmio-registers/) has a nice looking
  API for register access using their `svd2regs` tool.
* [Bobbin](http://www.bobbin.io/) also has some good looking ideas for register
  access (see `bobbin-dsl`).
