# stm32ral

This project provides a Rust RAL (register access layer) for all STM32
microcontrollers.

The underlying data is generated via the patched SVD files in
[stm32-rs](https://github.com/adamgreig/stm32-rs).

[Documentation](https://docs.rs/stm32ral)

[Repository](https://github.com/adamgreig/stm32ral)

[Supported Devices](supported_devices.md)

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
very quickly. It covers all registers of all STM32 devices, including core
Cortex-M peripherals, and aims to include full enumerated values for each
field as soon as possible.

Please consider trying it out and contributing or leaving feedback!

## Quick Example

```rust
use stm32ral::{rcc, gpio};

// Field-level read/modify/write, with either named values or just literals.
// Most of your code will look like this.
modify_reg!(rcc, RCC, AHB1ENR, GPIOAEN: Enabled);
modify_reg!(gpio, GPIOA, MODER, MODER1: Input, MODER2: Output, MODER3: Input);
while read_reg!(gpio, GPIOA, IDR, IDR3 == High) {
    let pa1 = read_reg!(gpio, GPIOA, IDR, IDR1);
    modify_reg!(gpio, GPIOA, ODR, ODR2: pa1);
}

// You can also reset whole registers or specific fields
reset_reg!(gpio, GPIOA, MODER, MODER13, MODER14, MODER15);
reset_reg!(gpio, GPIOB, MODER);

// Whole-register read/modify/write.
// Rarely used but nice to have the option.
// It's a bit shorter for single-field registers.
let port = read_reg!(gpio, GPIOA, IDR);
write_reg!(gpio, GPIOA, ODR, 0x12345678);
modify_reg!(gpio, GPIOA, MODER, |r| r | (0b10 << 4));

// Or forego the macros and just use the constants yourself.
// The macros above just expand to these forms for you, bringing
// the relevant constants into scope. Nothing else is going on.
let pa1 = (gpio::GPIOA.IDR.read() & gpio::IDR::IDR1::mask)
          >> gpio::IDR::IDR1::offset;
gpio::GPIOA.ODR.write(gpio::ODR::ODR2::RW::Output << gpio::ODR::ODR2::offset);
```

## Why use stm32ral?

* Small and lightweight (~20MB total file size, <2MB compressed)
* Simple (just 4 macros and a lot of constants)
* Quick to compile (~2s build time)
* Covers [all STM32 devices](supported_devices.md) in one crate
* Supports `cortex-m-rt` via the `rt` feature, including interrupts
* Doesn't get in your way
* A bit like what you're used to from C header files

## Why not use stm32ral?

* Still experimental, might have breaking changes to API design
* Takes an assume-safe-by-default approach so edge cases may be missed
  (but you'll probably notice when you start writing pointers to registers)
* Won't keep you warm burning CPU time
* A bit like what you're used to from C header files

## Instead, consider using...

* [svd2rust](https://github.com/japaric/svd2rust) is the obvious choice for
  generating Cortex-M device crates from SVD files, and provided inspiration
  for this project
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

modify_reg!(stm32ral::gpio, GPIOA, MODER, MODER1: Input, MODER2: Output, MODER3: Input);
```

### Register Definitions

At the top level of `stm32ral`, there is a module for each supported family
of devices, such as `stm32ral::stm32f4`. Inside each family are modules for
each supported device, such as `stm32ral::stm32f4::stm32f405`. When you
specify a device feature, everything inside that module is re-exported
at the top level, so that for example `stm32ral::stm32f4::stm32f405::gpio`
is also accessible as `stm32ral::gpio`. This means for many devices you can
simply change which feature you build `stm32ral` with, and not have to
change any of the code that uses it (since the paths will remain the same).

Inside each device module there is a module for each peripheral, such as
`stm32f405::gpio`. Inside each peripheral module is a module for each register,
such as `stm32f405::gpio::MODER`, and inside each register module is a module
for each field, such as `stm32f405::gpio::MODER::MODER15`.

Inside each field is a `mask` and `offset` constant which define the bitmask
for that field and its bit offset within the register. Each field also
contains an `R`, `W`, and `RW` module, which contain values which may be read,
written, or read+written from this module (mapping to `enumeratedValues`
from the SVD).

An example so far:
```rust
// Equivalent to stm32ral crate root with `--features stm32f405`
pub mod stm32f4 {
    pub mod stm32f405 {
        pub mod gpio {
            pub mod MODER {
                pub mod MODER15 {
                    pub const offset: u32 = 30;
                    pub const mask: u32 = 0b11 << offset;
                    pub mod R {}
                    pub mod W {}
                    pub mod RW {
                        pub const Input: u32 = 0b00;
                        pub const Output: u32 = 0b01;
                        pub const Alternate: u32 = 0b10;
                        pub const Analog: u32 = 0b11;
                    }
                }
            }
        }
    }
}
pub use stm32f4::stm32f405::*;
```

Next there is the `RegisterBlock`, a struct which contains the registers
for all instances of this peripheral. Each register is one of `RWRegister`,
`RORegister`, `WORegister`, or the `Unsafe*` variants thereof. These provide
`.read()` and `.write(value)` methods. For each instance of the peripheral,
there is a const which implements `Deref` to the relevant RegisterBlock:

```rust
// Inside a peripheral module such as `stm32ral::stm32f4::stm32f405::gpio`

pub struct RegisterBlock {
    pub MODER: RWRegister<u32>,
    pub OTYPER: RWRegister<u32>,
    // ...
}

pub const GPIOA //...
pub const GPIOB //...
pub const GPIOC //...
```

These constants are what permit access to the relevant registers:

```rust
// In reality, you'd use write_reg!(gpio, GPIOA, MODER, 0x1234)
// and read_reg!(gpio, GPIOA, MODER)
gpio::GPIOA.MODER.write(0x1234);
let _ = gpio::GPIOA.MODER.read();
```

Finally there is also a `ResetValues` struct defined which contains an
integer constant reset value for each register. These structs live inside
the same `GPIOA` instance consts, so you can:

```rust
// In reality, you'd use reset_reg!(gpio, GPIOA, MODER);
gpio::GPIOA.MODER.write(GPIOA.reset.MODER);
```

As an implementation detail, many structs are actually refactored to live
in the family level, with the original definitions replaced by `pub use`
statements, to reduce duplication and bloat in the crate source files.
The same is also true of duplicated values in the `R`, `W`, and `RW` modules.

### Macros

To simplify using all the constants and registers, four macros are provided.
For full details please check out
[the documentation](https://docs.rs/stm32ral).

#### `write_reg!(peripheral, INSTANCE, REGISTER, value)`

* Directly writes `value` to `INSTANCE.REGISTER`.

```rust
// Set PA3 high (and all other GPIOA pins low).
write_reg!(stm32ral::gpio, GPIOA, ODR, 1<<3);
```

#### `write_reg!(peripheral, INSTANCE, REGISTER, FIELD1: value1, FIELD2: value2, ...)`

* Writes `value`s to `FIELD`s and all other fields to 0 (for one or more `FIELD`s)
* You can use any `FIELD` which is a submodule of `REGISTER`.
* You can specify any arbitrary `value`, or you can also use any constant
  value inside the `W` or `RW` modules of the `FIELD` module.

```rust
// Set PA3 to Output, PA4 to Analog, PA5 to 0b01 (also Output), everything
// else gets set to 0 (Input).
// (In reality, be careful, as this operation will change the state of the
//  JTAG/SWD pins PA13-15, possibly breaking debugger access.
//  Use modify_reg!() instead.)
write_reg!(stm32ral::gpio, GPIOA, MODER, MODER3: Output, MODER4: Analog, MODER5: 0b01);
```

#### `read_reg!(peripheral, INSTANCE, REGISTER)`

* Reads and returns the current value of `INSTANCE.REGISTER`

```rust
// Get the value of the whole register IDR
let val = read_reg!(stm32ral::gpio, GPIOA, IDR);
```

#### `read_reg!(peripheral, INSTANCE, REGISTER, FIELD)`

* Reads and returns the current value of `FIELD` inside `INSTANCE.REGISTER`

```rust
// Get the value of IDR2 (masked and shifted down to the LSbits)
let val = read_reg!(stm32ral::gpio, GPIOA, IDR, IDR2);
```

#### `read_reg!(peripheral, INSTANCE, REGISTER, FIELD EXPRESSION)`

* Reads the current value of `FIELD` and returns the value of
  `FIELD EXPRESSION`
* `EXPRESSION` can be any token tree that makes sense in context, but
  is typically something like `== value`, `!= value`
* As with `write_reg!()`, all the values from `R` and `RW` modules of `FIELD`
  are brought into scope, so you can use `MODER5 == Output` for example

```rust
// Busy wait while PA2 is high
while read_reg!(stm32ral::gpio, GPIOA, IDR, IDR2 == High) {}
```

#### `modify_reg!(peripheral, INSTANCE, REGISTER, |r| fn(r))`

* Reads `INSTANCE.REGISTER` as `r`, then writes `fn(r)` to it
* Any lambda or function taking the register's type is acceptable

```rust
// Set PA3 high without affecting any other bits
// (in reality, use the BSRR register for this).
modify_reg!(stm32ral::gpio, GPIOA, ODR, |reg| reg | (1<<3));
```

#### `modify_reg!(peripheral, INSTANCE, REGISTER, FIELD1: VALUE1, FIELD2: VALUE2, ...)`

* Updates only the specified `FIELD`s to the new `VALUE`s, without changing
  any other fields
* Reads `INSTANCE.REGISTER`, masks out the bits corresponding to the specified
  `FIELD`s, sets those bits to the specified `VALUE`s, and writes back the
  result
* As with `write_reg!()`, all the values from the `W` and `RW` modules of
  `FIELD` are brought into scope

```rust
// Set PA3 to Output and PA4 to Analog, but without affecting any other pins.
modify_reg!(stm32ral::gpio, GPIOA, MODER, MODER3: Output, MODER4: Analog);
```

#### `reset_reg!(peripheral, INSTANCE, REGISTER)`

* Writes the reset value to `INSTANCE.REGISTER`

```rust
// Reset GPIOA back to reset state, with JTAG/SWD pins on PA13, PA14, PA15.
reset_reg!(stm32ral::gpio, GPIOA, MODER);
```

#### `reset_reg!(peripheral, INSTANCE, REGISTER, FIELD1, FIELD2)`

* Writes the reset value to the specified `FIELD`s without changing the
  other fields
* Reads `INSTANCE.REGISTER`, masks off the specified `FIELD`s, sets those
  bits to their reset values, and writes back the result

```rust
// Reset PA13, PA14, PA15 to their reset state.
reset_reg!(stm32ral::gpio, GPIOA, MODER, MODER13, MODER14, MODER15);
```

### Runtime Support & Interrupts

Use the `rt` feature to bring in `cortex-m-rt` support, providing a suitable
`device.x` linker script and interrupt definitions.

You can then specify your own interrupt handler:

```rust
interrupt!(TIM2, my_tim2_handler);
fn my_tim2_handler() {
    write_reg!(stm32ral::tim2, TIM2, SR, UIF: 0);
}
```

If you're using `cortex-m`, the `Interrupt` enum is compatible (it implements
[`Nr`](https://docs.rs/bare-metal/0.2.0/bare_metal/trait.Nr.html)):

```rust
peripherals.NVIC.enable(stm32ral::Interrupt::TIM2);
```

## Safety

Safety is approached by marking some registers as unsafe, which require unsafe
blocks/functions to access.
Unsafe registers are those where access could lead to [undefined
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

You only need to do this if you are planning on modifying `stm32ral.py` or
otherwise changing how `stm32ral` is put together; it's not required just to
use it in another Rust project.

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
