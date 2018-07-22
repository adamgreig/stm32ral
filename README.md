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
#[macro_use]
extern crate stm32ral;
use stm32ral::{rcc, gpio};

// For safe access we have to first `take()` the peripheral instance.
// This can only be done once (it will return None thereafter) to ensure
// that no other code can be simultaneously accessing the perihperal,
// which could lead to a race condition. See below for unsafe use.
let gpioa = gpio::GPIOA::take().unwrap();
let rcc = rcc::RCC::take().unwrap();

// Field-level read/modify/write, with either named values or just literals.
// Most of your code will look like this.
modify_reg!(rcc, rcc, AHB1ENR, GPIOAEN: Enabled);
modify_reg!(gpio, gpioa, MODER, MODER1: Input, MODER2: Output, MODER3: Input);
while read_reg!(gpio, gpioa, IDR, IDR3 == High) {
    let pa1 = read_reg!(gpio, gpioa, IDR, IDR1);
    modify_reg!(gpio, gpioa, ODR, ODR2: pa1);
}

// You can also reset whole registers or specific fields
reset_reg!(gpio, gpioa, GPIOA, MODER, MODER13, MODER14, MODER15);
reset_reg!(gpio, gpioa, GPIOA, MODER);

// Whole-register read/modify/write.
// Rarely used but nice to have the option.
let port = read_reg!(gpio, gpioa, IDR);
write_reg!(gpio, gpioa, ODR, 0x12345678);
modify_reg!(gpio, gpioa, MODER, |r| r | (0b10 << 4));

// Or forego the macros and just use the constants yourself.
// The macros above just expand to these forms for you, bringing
// the relevant constants into scope. Nothing else is going on.
let pa1 = (gpioa.IDR.read() & gpio::IDR::IDR1::mask) >> gpio::IDR::IDR1::offset;
gpioa.ODR.write(gpio::ODR::ODR2::RW::Output << gpio::ODR::ODR2::offset);

// For unsafe access, you don't need to first call `take()`, just use `GPIOA`:
unsafe { modify_reg!(gpio, GPIOA, MODER, MODER1: Output) };

// Or you can use `get()` to unsafely get an instance:
let gpioa = unsafe { gpio::GPIOA::get() };
modify_reg!(gpio, gpioa, MODER, MODER1: Output);
```

## Why use stm32ral?

* Small and lightweight (~30MB total file size, ~2MB compressed)
* Simple (just 4 macros and a lot of constants)
* Quick to compile (~2s build time)
* Covers [all STM32 devices](supported_devices.md) in one crate
* Supports `cortex-m-rt` via the (default) `rt` feature, including interrupts
* Doesn't get in your way
* A bit like what you're used to from C header files

## Why not use stm32ral?

* Still experimental, might have breaking changes to API design
* Won't keep you warm burning CPU time
* A bit like what you're used to from C header files

## Instead, consider using...

* [svd2rust](https://github.com/japaric/svd2rust) is the obvious choice for
  generating Cortex-M device crates from SVD files, and provided inspiration
  for this project
* [stm32-rs](https://github.com/adamgreig/stm32-rs) provides `svd2rust` crates
  for all STM32 devices supported by this crate, and uses the same
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
features = ["stm32f405"]
```
Replace `stm32f405` with the required chip name. See
[Supported Devices](supported_devices.md) for the full list.

Then, in your code:

```rust
#[macro_use]
extern crate stm32ral;

let gpioa = stm32ral::gpio::GPIOA::take().unwrap();
modify_reg!(stm32ral::gpio, gpioa, MODER, MODER1: Input, MODER2: Output, MODER3: Input);
```

### Crate Features

* `inline-asm`: enables `inline-asm` on the `cortex_m` dependency.
  Recommended if you're using a nightly compiler that supports it,
  which is why it's on by default, but you can disable it to run on
  stable.
* `rt`: enables `device` on the `cortex_m_rt` dependency, and
  provides the relevant interrupt linker scripts.
  Recommended for most users which is why it's on by default, but you can
  disable it if you want to handle interrupts yourself.
* `doc`: makes all devices visible in the output without using any of them
  at the top level. Ideal for generating documentation. Not useful for
  actually building code.
* CPU features like `armv7em`: brings in peripherals from the CPU core itself,
  the relevant one is automatically included by the device features.
* Device features: one per supported device, for example, `stm32f405`.
  You should enable precisely one of these.

To disable the default `inline-asm` and `rt` features, in your `Cargo.toml`:

```toml
[dependencies.stm32ral]
version = "0.1.0"
default-features = false
features = ["stm32f405"]
```

### Internal Structure

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
`.read()` and `.write(value)` methods.

```rust
// Inside a peripheral module such as `stm32ral::stm32f4::stm32f405::gpio`

pub struct RegisterBlock {
    pub MODER: RWRegister<u32>,
    pub OTYPER: RWRegister<u32>,
    // ...
}
```

Then there is the `ResetValues` struct, which has an integer field for each
register in the `RegisterBlock`. Each instance of the peripheral will include
one `ResetValues` appropriately initialised, so you can:

```rust
// In reality, you'd use reset_reg!(gpio, gpioa, GPIOA, MODER);
gpioa.MODER.write(stm32ral::gpio::GPIOA::reset.MODER);
```

Finally there is a module for each instance of the peripheral, containing
its `ResetValues` and a `take()` and unsafe `get()` function to obtain
a `&RegisterBlock`. The safe `take()` returns an `Option<&RegisterBlock>`
which will be `Some` the first time you call it and `None` every subsequent
time, to ensure that you have exclusive access to the peripheral and cannot
encounter data races. The unsafe `get()` just returns a `&RegisterBlock`;
it's then up to you to ensure no data races will occur.

```rust
// Inside a peripheral module such as `stm32ral::stm32f4::stm32f405::gpio`

pub mod GPIOA {
    pub const reset: ResetValues = ResetValues { ... };
    pub unsafe fn get() -> &'static RegisterBlock { ... };
    pub fn take() -> Option<&'static RegisterBlock> { ... };
}

pub mod GPIOB { ... }
pub mod GPIOC { ... }
// and so on
```

These constants are what permit access to the relevant registers:

```rust
// In reality, you'd use write_reg!(gpio, gpioa, MODER, 0x1234)
// and read_reg!(gpio, gpioa, MODER)
let gpioa = gpio::GPIOA::take().unwrap();
gpioa..MODER.write(0x1234);
let _ = gpioa.MODER.read();
```

For convenience in unsafe code, there is also a raw pointer for each
`RegisterBlock`:

```rust
pub const GPIOA: *const RegisterBlock = ...;
```

This permits direct use in macros without requiring you to
first call `take()` or `get()` (see below).

As an implementation detail, many structs are actually refactored to live
in the family level, with the original definitions replaced by `pub use`
statements, to reduce duplication and bloat in the crate source files.
The same is also true of duplicated values in the `R`, `W`, and `RW` modules.

### Macros

To simplify using all the constants and registers, four macros are provided.
For full details please check out
[the documentation](https://docs.rs/stm32ral).

#### `write_reg!(peripheral, instance, REGISTER, value)`

* Directly writes `value` to `instance.REGISTER`.

```rust
// Set PA3 high (and all other GPIOA pins low).
write_reg!(stm32ral::gpio, gpioa, ODR, 1<<3);
```

#### `write_reg!(peripheral, instance, REGISTER, FIELD1: value1, FIELD2: value2, ...)`

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
write_reg!(stm32ral::gpio, gpioa, MODER, MODER3: Output, MODER4: Analog, MODER5: 0b01);
```

#### `read_reg!(peripheral, instance, REGISTER)`

* Reads and returns the current value of `instance.REGISTER`

```rust
// Get the value of the whole register IDR
let val = read_reg!(stm32ral::gpio, gpioa, IDR);
```

#### `read_reg!(peripheral, instance, REGISTER, FIELD)`

* Reads and returns the current value of `FIELD` inside `instance.REGISTER`

```rust
// Get the value of IDR2 (masked and shifted down to the LSbits)
let val = read_reg!(stm32ral::gpio, gpioa, IDR, IDR2);
```

#### `read_reg!(peripheral, instance, REGISTER, FIELD EXPRESSION)`

* Reads the current value of `FIELD` and returns the value of
  `FIELD EXPRESSION`
* `EXPRESSION` can be any token tree that makes sense in context, but
  is typically something like `== value`, `!= value`
* As with `write_reg!()`, all the values from `R` and `RW` modules of `FIELD`
  are brought into scope, so you can use `MODER5 == Output` for example

```rust
// Busy wait while PA2 is high
while read_reg!(stm32ral::gpio, gpioa, IDR, IDR2 == High) {}
```

#### `modify_reg!(peripheral, instance, REGISTER, |r| fn(r))`

* Reads `instance.REGISTER` as `r`, then writes `fn(r)` to it
* Any lambda or function taking the register's type is acceptable

```rust
// Set PA3 high without affecting any other bits
// (in reality, use the BSRR register for this).
modify_reg!(stm32ral::gpio, gpioa, ODR, |reg| reg | (1<<3));
```

#### `modify_reg!(peripheral, instance, REGISTER, FIELD1: VALUE1, FIELD2: VALUE2, ...)`

* Updates only the specified `FIELD`s to the new `VALUE`s, without changing
  any other fields
* Reads `instance.REGISTER`, masks out the bits corresponding to the specified
  `FIELD`s, sets those bits to the specified `VALUE`s, and writes back the
  result
* As with `write_reg!()`, all the values from the `W` and `RW` modules of
  `FIELD` are brought into scope

```rust
// Set PA3 to Output and PA4 to Analog, but without affecting any other pins.
modify_reg!(stm32ral::gpio, gpioa, MODER, MODER3: Output, MODER4: Analog);
```

#### `reset_reg!(peripheral, instance, INSTANCE, REGISTER)`

* Writes the reset value to `instance.REGISTER`
* Note you have to specify both an `instance` (the actual `&RegisterBlock`)
  and `INSTANCE` (the name of the instance module inside the peripheral
  module, e.g. `peripheral::INSTANCE::reset` must exist).

```rust
// Reset GPIOA back to reset state, with JTAG/SWD pins on PA13, PA14, PA15.
reset_reg!(stm32ral::gpio, gpioa, GPIOA, MODER);
```

#### `reset_reg!(peripheral, instance, INSTANCE, REGISTER, FIELD1, FIELD2)`

* Writes the reset value to the specified `FIELD`s without changing the
  other fields
* Reads `instance.REGISTER`, masks off the specified `FIELD`s, sets those
  bits to their reset values, and writes back the result
* Note you have to specify both an `instance` (the actual `&RegisterBlock`)
  and `INSTANCE` (the name of the instance module inside the peripheral
  module, e.g. `peripheral::INSTANCE::reset` must exist).

```rust
// Reset PA13, PA14, PA15 to their reset state.
reset_reg!(stm32ral::gpio, gpioa, GPIOA, MODER, MODER13, MODER14, MODER15);
```

#### Unsafe Macro Use

For convenience, when using the macros in an `unsafe` context,
you do not need to first `take()`/`get()` the instance and can instead
specify it directly:

```rust
// The `GPIOE` is effectively the same as `GPIOE::get()` here.
unsafe { write_reg!(stm32ral::gpio, GPIOE, 0x01010101) };

// The macro is effectively doing this:
unsafe { (*stm32ral::gpio::GPIOE).MODER.write( 0x01010101 ) };
```

This works because each instance also exists as a `*const RegisterBlock`
in the peripheral module, which the macros bring into scope and dereference.

### Runtime Support & Interrupts

Use the default `rt` feature to bring in `cortex-m-rt` support, providing a
suitable `device.x` linker script and interrupt definitions.

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

First, a safety preface. This crate considers safety strictly in the Rust
sense of avoiding [undefined behaviour](https://doc.rust-lang.org/reference/behavior-considered-undefined.html),
and not in any more general sense related to embedded hardware. We use safety
to avoid data races but not to avoid shorting out your hardware: that's on you.
Given the low-level nature of this crate, it is expected it will often (though
not always!) be used in an `unsafe` context, and is designed to facilitate this
as much as possible. Higher level crates such as HALs seem a better place
to facilitate safe abstractions.

There are two major safety concerns with a register access crate.

First is the possibility that peripherals will perform actions on unrelated
memory, for example a DMA peripheral or a cache control register. Such
registers are marked as unsafe and reading or writing them will always
require an `unsafe` block or function. Under the hood, they use
the `UnsafeXXRegister` types instead of the usual `XXRegister`. Since such
registers could potentially cause undefined behaviour, the user must make sure
when accessing them to provide their own safety guarantees.

Most registers will not be unsafe and can be directly accessed in safe code.
The macros provided for field access ensure values are masked for the field
width, but otherwise nothing prevents safe code writing arbitrary values
to registers not specifically marked unsafe. This is considered a usability
trade-off; while some illegal values in some device registers will surely cause
unexpected behaviour; so will many _legal_ values (Rust cannot prevent you
setting an output low which is hardwired to the supply rail, for example).
Aside from a few specific registers, writing those values should not cause
undefined behaviour in Rust itself, so our tradeoff is to try and prevent UB
while not trying to use the safety system to enforce that all register fields
may only be written with legal values.

The second safety issue is around synchronised access to peripherals, which
are effectively global shared memory. The safety concern is around data races:
if you are reading and writing from a peripheral but halfway through an
interrupt routine wants to access the same peripheral, you will race it,
leading to undefined behaviour.

The solution provided here is similar to `svd2rust`, though more granular:
every peripheral instance has a `take() -> Option<&RegisterBlock>` function
which returns the reference the first time it is called, and None thereafter.
You can therefore use this safe function in your code to obtain a reference
once, and pass it on to any other functions that require it, while ensuring
no other threads (or interrupt routines) can access the peripheral in safe
code. The references are `!Sync` due to an interior `UnsafeCell`, so you
cannot pass the `&RegisterBlock` between threads.

However, you will often need to use peripherals in other contexts where it is
awkward or impossible to safety pass the `&RegisterBlock` you first obtained.
This crate provides both an `unsafe get() -> &RegisterBlock` which always
returns the reference, and a `*const RegisterBlock` which can be used for
convenience with the macros. When using these unsafe features, you must ensure
no data races will happen yourself (for instance, because an interrupt will
only fire after you are doing initialising the peripheral and don't access it
thereafter, or because you use your own mutex to ensure exclusive access, etc).

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
