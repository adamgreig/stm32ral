# stm32ral

This project provides a Rust RAL (register access layer) for all STM32
microcontrollers.

The underlying data is generated via the patched SVD files in
[stm32-rs](https://github.com/adamgreig/stm32-rs).

[Documentation](https://docs.rs/stm32ral)

[Repository](https://github.com/adamgreig/stm32ral)

## What is it?

stm32ral is an experiment into a lightweight register access layer. It provides
access to every register, and provides constants which define the fields and
possible field values in those registers. In that sense it is a lot like C
device header files. However, it also provides a couple of simple macros that
permit very easy register access, with very simple generated code which
is efficient even without optimisations enabled.

The main aims are simplicity, compactness, and completeness. You get a module
structure that contains a struct for each peripheral comprising just its
registers in order, and you get a lot of constants for field widths, positions,
and possible values. There's not much else, so it takes little disk and builds
very quickly. It covers all registers of all STM32 devices, and aims to
include full enumerated values for each field as soon as possible.

Please consider trying it out and contributing or leaving feedback!

## Quick Example

```rust
use stm32ral::{rcc, gpio};

// Field-level read/modify/write, with either named values or just literals.
// Most of your code will look like this.
modify_reg!(rcc, RCC.AHB1ENR, GPIOAEN: Enabled);
modify_reg!(gpio, GPIOA.MODER, MODER1: Input, MODER2: Output, MODER3: Input);
while read_reg!(gpio, GPIOA.IDR, IDR3 == High) {
    let pa1 = read_reg!(gpio, GPIOA.IDR, IDR1);
    modify_reg!(gpio, GPIOA.ODR, ODR2: pin);
}

// Whole-register read/modify/write.
// Rarely used but nice to have the option.
// It's a bit shorter for single-field registers.
let port = read_reg!(gpio, GPIOA.IDR);
write_reg!(gpio, GPIOA.ODR, 0x12345678);
modify_reg!(gpio, GPIOA.MODER, |r| r | (0b10 << 4));

// Or forego the macros and just use the constants yourself.
// The macros above just expand to these forms for you, bringing
// the relevant constants into scope. Nothing else is going on.
let pa1 = (gpio::GPIOA.IDR.read() >> gpio::IDR::IDR1::_offset)
          & gpio::IDR::IDR1::_mask;
gpio::GPIOA.ODR.write(gpio::ODR::ODR2::Output << gpio::ODR::ODR2::_offset);
```

## Why use stm32ral?

* Small and lightweight
* Simple
* Quick to compile
* Covers all STM32 devices in one crate
* Supports `cortex-m-rt` via the `rt` feature, including interrupts
* Doesn't get in your way
* A bit like what you're used to from C header files

## Why not use stm32ral?

* Not nearly as much safety as leading competitors
* Won't keep you warm burning CPU time
* A bit like what you're used to from C header files

## Instead, consider using...

* [svd2rust](https://github.com/japaric/svd2rust) is the obvious choice for
  generating Cortex-M device crates from SVD files, and provided inspiration
  for this one
* [stm32-rs](https://github.com/adamgreig/stm32-rs) provides `svd2rust` crates
  for all STM32 devices supported by this crate, and they use the same
  underlying patched SVD files
* [TockOS](https://www.tockos.org/blog/2018/mmio-registers/) has a nice looking
  API for register access using their `svd2regs` tool.
* [Bobbin](http://www.bobbin.io/) also has some good looking ideas for register
  access (see `bobbin-dsl`).

## Using in your own crates

In your `Cargo.toml`:
```toml
[dependencies.stm32ral]
version = "0.1.0"
features = ["stm32f405", "rt"]
```
Replace `stm32f405` with the required chip name. See
[Supported Devices](supported_devices.md) for the full list.

Then, in your code:
```rust
#[macro_use]
extern crate stm32ral;

modify_reg!(stm32ral::gpio, GPIOA.MODER, MODER1: Input, MODER2: Output, MODER3: Input);
```

### Runtime Support & Interrupts

Use the `rt` feature to bring in `cortex-m-rt`, which provides a suitable
`device.x` linker script and a default handler for the interrupt handlers.

You can then specify your own interrupt handler:
```rust
interrupt!(TIM2, my_tim2_handler);
fn my_tim2_handler() {
    write_reg!(stm32ral::tim2, TIM2.SR, UIF: 0);
}
```

If you're using `cortex-m`, the `Interrupt` enum is compatible:
```rust
peripherals.NVIC.enable(stm32ral::Interrupt::TIM2);
```

## Safety

Safety is approached by marking some registers as unsafe; those require either
unsafe blocks/functions to access, or that the `unsafe` feature of this crate
is enabled. Unsafe registers are those where access could lead to [undefined
behaviour](https://doc.rust-lang.org/reference/behavior-considered-undefined.html),
such as DMA source and target registers, cache control registers, etc. Most
registers will not be unsafe and can be directly accessed in safe code. The
macros provided for field access ensure values are masked for the field width,
but otherwise nothing prevents safe code writing arbitrary values to registers
not specifically marked unsafe. This is considered a usability trade-off;
while some illegal values in some device registers will surely cause unexpected
behaviour; so will many _legal_ values (Rust cannot prevent you setting an
output low which is hardwired to the supply rail, for example). Aside from
a few specific registers, writing those values should not cause undefined
behaviour in Rust itself, so our tradeoff is to try and prevent UB while not
trying to use the safety system to enforce that all register fields may only
be written with legal values.

## Contributing

Contributions are very welcome!

To add new named values for registers or fix errors in the registers, please
instead update the SVD files in
[stm32-rs](https://github.com/adamgreig/stm32-rs), which will then be used
in this crate.

Changes to this crate are primarily concerned with how the RAL is generated
from the SVD files.

## Building stm32ral

First set up the stm32-rs submodule:
```
$ git submodule update --init
$ cd stm32-rs/svd
$ ./extract.sh
$ cd ../..
```

Now you should simply be able to run make, which will automatically run
`make patch` inside the stm32-rs submodule to produce up-to-date patched SVDs.
```
$ make
```

Be sure to update the submodule (`git submodule update`) if it's been changed
upstream to make sure you're using the latest available SVD patches.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
