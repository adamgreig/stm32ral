# stm32ral

This project provides a Rust RAL (register access layer) for all STM32
microcontrollers.

The underlying data is generated via the patched SVD files in
[stm32-rs](https://github.com/adamgreig/stm32-rs).

[![Version](https://img.shields.io/crates/v/stm32ral.svg)](https://crates.io/crates/stm32ral)
[![Documentation](https://docs.rs/stm32ral/badge.svg)](https://docs.rs/stm32ral)
[![Build Status](https://travis-ci.org/adamgreig/stm32ral.svg?branch=master)](https://travis-ci.org/adamgreig/stm32ral)
![License](https://img.shields.io/crates/l/stm32ral.svg)

[Documentation](https://docs.rs/stm32ral) ·
[Repository](https://github.com/adamgreig/stm32ral) ·
[Supported Devices](supported_devices.md) ·
[Example Project](https://github.com/adamgreig/stm32ral-example)

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
// This only returns Some(Instance) if that instance is not already
// taken; otherwise it returns None. This ensures that no other code can be
// simultaneously accessing the peripheral, which could lead to a race
// condition. There's `release()` to return it. See below for unsafe use.
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

// Once you're done with a peripheral, you can release it so it is available
// to `take()` again. You can't use `gpioa` after this line.
gpio::GPIOA::release(gpioa);

// For unsafe access, you don't need to first call `take()`, just use `GPIOA`:
unsafe { modify_reg!(gpio, GPIOA, MODER, MODER1: Output) };
// With the `nosync` feature set, this is the only way to access registers.
```

See [the example project](https://github.com/adamgreig/stm32ral-example) for
a more complete example that should build out of the box.

## Why use stm32ral?

* Small and lightweight (~30MB total file size, ~2MB compressed)
* Simple (just 4 macros and a lot of constants)
* Quick to compile (~2s build time)
* Covers [all STM32 devices](supported_devices.md) in one crate
* Supports `cortex-m-rt` via the `rt` feature, including interrupts
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
  Recommended if you're using a nightly compiler that supports it.
* `rt`: enables `device` on the `cortex_m_rt` dependency, and
  provides the relevant interrupt linker scripts.
  Recommended for most users, but you can leave it off
  if you want to handle interrupts yourself.
* `doc`: makes all devices visible in the output without using any of them
  at the top level. Ideal for generating documentation. Not useful for
  actually building code.
* `nosync`: disables all synchronised access (`take()`/`release()` functions).
  The only way to access registers is with the direct `unsafe` access, such as
  `write_reg!(stm32ral::gpio, GPIOA, MODER, MODER1: Input)`.
  Removes all associated synchronisation overhead, but of course the user
  must ensure they do not cause race conditions. "C" mode.
  Especially useful if enabled by a HAL crate which will perform its own
  synchronisation but can still permit unsafe direct access to peripherals by
  users (which is why this is a 'negative' feature).
* CPU features like `armv7em`: brings in peripherals from the CPU core itself,
  the relevant one is automatically included by the device features.
* Device features: one per supported device, for example, `stm32f405`.
  You should enable precisely one of these.

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

There is an `Instance` struct which represents a value you can own and move
around and give out references to, which Derefs to a `RegisterBlock`
to actually access the registers. There's only one `Instance` for each
peripheral instance; you can get it using `take()` and return it for someone
else using `release()` (see below).

```rust
// Inside a peripheral module such as `stm32ral::stm32f4::stm32f405::gpio`

pub struct Instance {
    addr: u32,
}
impl Deref for Instance {
    type Target = RegisterBlock;
    fn deref(&self) -> &RegisterBlock { ... }
}
```

Finally there is a module for each instance of the peripheral, containing
its `ResetValues`, its one `Instance`, and a `take()` function to obtain it.
`take()` returns an `Option<Instance>` which will be `Some` if the instance
was available and `None` if not. This ensures you have exclusive acess
to the peripheral and cannot encounter data races with other safe code.
You can call `release()` to return the instance for others to `take()`.

```rust
// Inside a peripheral module such as `stm32ral::stm32f4::stm32f405::gpio`

pub mod GPIOA {
    pub const reset: ResetValues = ResetValues { ... };
    const INSTANCE: Instance = Instance { ... };
    pub fn take() -> Option<Instance> { ... };
    pub fn release(Instance) { ... };
}

pub mod GPIOB { ... }
pub mod GPIOC { ... }
// and so on
```

These instances are what permit access to the relevant registers:

```rust
// In reality, you'd use write_reg!(gpio, gpioa, MODER, 0x1234)
// and read_reg!(gpio, gpioa, MODER)
let gpioa = gpio::GPIOA::take().unwrap();
gpioa.MODER.write(0x1234);
let _ = gpioa.MODER.read();
```

For convenience in unsafe code, there is also a raw pointer directly to each
`RegisterBlock`:

```rust
pub const GPIOA: *const RegisterBlock = ...;
```

This permits direct use in macros without requiring you to first call `take()`
(see below for macros).

Note that with the `nosync` feature enabled, the `Instance` and
`take()`/`release()` methods are not generated; the only access is via the
raw pointer described above.

As an implementation detail, many structs are actually refactored to live
in the family level, with the original definitions replaced by `pub use`
statements, to reduce duplication and bloat in the crate source files.
The same is also true of duplicated values in the `R`, `W`, and `RW` modules.

### Macros

To simplify using all the constants and registers, four macros are provided.
For full details please check out
[the documentation](https://docs.rs/stm32ral).

In the definitions below:
* `peripheral` is a path to a peripheral module, for example
  `stm32ral::gpio`,
* `instance` is any expression that dereferences to `RegisterBlock`:
  an `Instance`, `&Instance`, `&RegisterBlock`, or `*const RegisterBlock`,
* `INSTANCE` is the path to an instance module, for example
  `stm32ral::gpio::GPIOA`, but anything inside the `peripheral` module will
  be in scope, so you can simply specify `GPIOA`,
* `REGISTER` is an ident and the name of any register in the peripheral,
  for example `MODER`, which must exist as a field in the `RegisterBlock`,
* `value` can be a literal value or any named values from the register
  module.

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

#### `read_reg!(peripheral, instance, REGISTER, FIELD1, FIELD2, ...)`

* Reads and returns the current values of `FIELD1`, `FIELD2`, ... inside
  `instance.REGISTER`

```rust
// Get the value of IDR2 (masked and shifted down to the LSbits)
let idr2 = read_reg!(stm32ral::gpio, gpioa, IDR, IDR2);

// Get the value of IDR2 and IDR3
let (idr2, idr3) = read_reg!(stm32ral::gpio, gpioa, IDR, IDR2, IDR);
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
* Note you have to specify both an `instance` (anything that derefs to
  `RegisterBlock`) and `INSTANCE` (the name of the instance module inside the 
  peripheral module, e.g. `peripheral::INSTANCE::reset` must exist).

```rust
// Reset GPIOA back to reset state, with JTAG/SWD pins on PA13, PA14, PA15.
reset_reg!(stm32ral::gpio, gpioa, GPIOA, MODER);
```

#### `reset_reg!(peripheral, instance, INSTANCE, REGISTER, FIELD1, FIELD2)`

* Writes the reset value to the specified `FIELD`s without changing the
  other fields
* Reads `instance.REGISTER`, masks off the specified `FIELD`s, sets those
  bits to their reset values, and writes back the result
* Note you have to specify both an `instance` (anything that derefs to
  `RegisterBlock`) and `INSTANCE` (the name of the instance module inside the 
  peripheral module, e.g. `peripheral::INSTANCE::reset` must exist).

```rust
// Reset PA13, PA14, PA15 to their reset state.
reset_reg!(stm32ral::gpio, gpioa, GPIOA, MODER, MODER13, MODER14, MODER15);
```

#### Unsafe Macro Use

For convenience, when using the macros in an `unsafe` context,
you do not need to first `take()` the instance and can instead
specify it directly:

```rust
// Unsafely and directly access GPIOE.
unsafe { write_reg!(stm32ral::gpio, GPIOE, 0x01010101) };

// The macro is effectively doing this:
unsafe { (*stm32ral::gpio::GPIOE).MODER.write( 0x01010101 ) };
```

This works because each instance also exists as a `*const RegisterBlock`
in the peripheral module, which the macros bring into scope and dereference.

### Runtime Support & Interrupts

Use the `rt` feature to bring in `cortex-m-rt` support, providing a
suitable `device.x` linker script and interrupt definitions.

You can then specify your own interrupt handler:

```rust
#[interrupt]
fn TIM2() {
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
every peripheral instance has a `take() -> Option<Instance>` function
which returns `Some(Instance)` if the instance is not currently taken, and
`None` if it is. You can therefore use this safe function in your code to
obtain an Instance, and pass it (or a reference to it) on to any other
functions that require it, while ensuring no other threads (or interrupt
routines) can access the peripheral in safe code. When you're done using it,
you can call `release(instance)` to make it available to `take()` again.

However, you will often need to use peripherals in other contexts where it is
awkward or impossible to safely pass the `Instance` around.
This crate provides a `*const RegisterBlock` which can be unsafely dereferenced 
for this purpose, and can be given directly in the macros in an unsafe context.
When using these unsafe features, you must ensure no data races will happen
yourself (for instance, because an interrupt will only fire after you are done
initialising the peripheral and don't access it thereafter, or because you use
your own mutex to ensure exclusive access, etc).

The `nosync` feature removes all the synchronisation methods described above
and leaves only unsafe access, reducing overhead and permitting some higher
level crate to provide its own safe access guarantees without having to take
every peripheral at runtime.

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

## Outstanding Work

There are a few unresolved issues which require some further thought,
but shouldn't present major backwards compatibility issues:

### More Enumerated Values, Bad SVD Files

The work to add all possible enumerated values and fix incorrect SVD files is
always ongoing at [stm32-rs](https://github.com/adamgreig/stm32-rs).

### Aliased Registers

Aliased registers are not as well handled as they could be. This is a classic
problem for Rust register accesses as Rust does not yet have anonymous
unions, which would be the usual solution in C.

At the moment, stm32ral merges aliased registers, attempting to pick a suitable
merged name, and combining all the fields together. This is ergonomic for
many aliased registers (e.g., `CCMR` in timers), but not for others
(such as `OTG_HS_DIEPINT5` and `OTG_HS_DIEPTSIZ7`, yuck).

Once there are anonymous unions there might be a better solution.

### Timers

Most peripherals combine well, for instance `GPIOA` through `GPIOK` are all
instances of `gpio::RegisterBlock`. The same applies to most other peripherals
like `USART`, `SPI`, `I2C`, and so on.

Timers are not well merged because their hierarchy is complicated: we can't
just have a single `TIM` since there are so many different register blocks,
but the solution at the moment (no merging) is not optimal either.

Ideally we might identify the various categories, such as:

* Advanced
* Basic
* General purpose (type 1)
* General purpose (type 2)
* General purpose (32 bit)
* Low power
* High resolution

We could then try to group those together.

### Other Peripherals

A few other peripherals do not merge well yet either, especially
on STM32F373 and STM32F3x8 where some GPIO peripherals do not have the
`LCKR` registers, annoyingly. The best solution might be to just pretend
it does have it.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
