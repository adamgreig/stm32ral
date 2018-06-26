# stm32ral

This project provides a Rust RAL (register access layer) for all STM32
microcontrollers.

The underlying data is generated via the SVD files in
[stm32-rs](https://github.com/adamgreig/stm32-rs).

[Documentation](https://docs.rs/stm32ral)

[Repository](https://github.com/adamgreig/stm32ral)


## Quick Example

```rust
use stm32ral::{rcc, gpio};

// Field-level read/modify/write, with either named values or just literals.
modify_reg!(rcc, RCC.ahb1enr, GPIOAEN: Enabled);
modify_reg!(gpio, GPIOA.moder, MODER1: Input, MODER2: Output, MODER3: Input);
while read_reg!(gpio, GPIOA.idr, IDR3 == High) {
    let pa1 = read_reg!(gpio, GPIOA.idr, IDR1);
    modify_reg!(gpio, GPIOA.odr, ODR2: pin);
}

// Whole-register read/modify/write.
let port = read_reg!(gpio, GPIOA.idr);
write_reg!(gpio, GPIOA.odr, 0x12345678);
modify_reg!(gpio, GPIOA.moder, |r| r | (0b10 << 4));

// Or forego the macros and just use the constants yourself.
// The macros above just expand to these forms for you, nothing else
// is going on.
let pa1 = (gpio::GPIOA.idr.read() >> gpio::idr::IDR1::_offset)
          & gpio::idr::IDR1::_mask;
gpio::GPIOA.odr.write(gpio::odr::ODR2::Output << gpio::odr::ODR2::_offset);
```

## Why use stm32ral?

* Small and lightweight
* Simple
* Quick to compile
* Covers all STM32 devices in one crate
* Doesn't get in your way
* A bit like what you're used to from C header files

## Why not use stm32ral?

* Not nearly as much safety as leading competitors
* You have too much disk space and need to fill it up somehow
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

## Using in your own crates

In your `Cargo.toml`:
```toml
[dependencies.stm32ral]
features = ["stm32f405"]
```
Replace `stm32f405` with the required chip name. Consider using the `unsafe`
feature as well, which makes all register access safe.

Then, in your code:
```rust
#[macro_use]
extern crate stm32ral;

modify_reg!(stm32ral::gpio, GPIOA.moder, MODER1: Input, MODER2: Output, MODER3: Input);
```

## Contributing

Contributions are very welcome!

To add new named values for registers or fix errors in the registers, please
instead update the SVD files in
[stm32-rs](https://github.com/adamgreig/stm32-rs), which will then be used
in this crate.

Changes to this crate are concerned with how the RAL is generated from the SVD
files.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
