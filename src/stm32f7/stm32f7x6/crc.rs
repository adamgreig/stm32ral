#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Cryptographic processor

use crate::{RWRegister, WORegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// DR and DR16
/// DR: DR and DR8
/// DR: Data register
/// DR8: Data register - byte sized
/// DR16: Data register - half-word sized
pub mod DR {

    /// Data Register
    pub mod DR {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Data register bits
    pub mod DR8 {
        /// Offset (0 bits)
        pub const offset: u8 = 0;
        /// Mask (8 bits: 0xff << 0)
        pub const mask: u8 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Data register bits
    pub mod DR16 {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u16 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Independent Data register
pub mod IDR {

    /// Independent Data register
    pub mod IDR {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (8 bits: 0xff << 0)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Control register
pub mod CR {

    /// RESET bit
    pub mod RESET {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Resets the CRC calculation unit and sets the data register to 0xFFFF FFFF
            pub const Reset: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Reverse output data
    pub mod REV_OUT {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Bit order not affected
            pub const Normal: u32 = 0b0;

            /// 0b1: Bit reversed output
            pub const Reversed: u32 = 0b1;
        }
    }

    /// Reverse input data
    pub mod REV_IN {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (2 bits: 0b11 << 5)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Bit order not affected
            pub const Normal: u32 = 0b00;

            /// 0b01: Bit reversal done by byte
            pub const Byte: u32 = 0b01;

            /// 0b10: Bit reversal done by half-word
            pub const HalfWord: u32 = 0b10;

            /// 0b11: Bit reversal done by word
            pub const Word: u32 = 0b11;
        }
    }

    /// Polynomial size
    pub mod POLYSIZE {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (2 bits: 0b11 << 3)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: 32-bit polynomial
            pub const Polysize32: u32 = 0b00;

            /// 0b01: 16-bit polynomial
            pub const Polysize16: u32 = 0b01;

            /// 0b10: 8-bit polynomial
            pub const Polysize8: u32 = 0b10;

            /// 0b11: 7-bit polynomial
            pub const Polysize7: u32 = 0b11;
        }
    }
}

/// Initial CRC value
pub mod INIT {

    /// Programmable initial CRC value
    pub mod INIT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// CRC polynomial
pub mod POL {

    /// Programmable polynomial
    pub mod POL {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}
#[repr(C)]
pub struct RegisterBlock {
    /// DR and DR16
    /// DR: DR and DR8
    /// DR: Data register
    /// DR8: Data register - byte sized
    /// DR16: Data register - half-word sized
    pub DR: RWRegister<u32>,

    /// Independent Data register
    pub IDR: RWRegister<u32>,

    /// Control register
    pub CR: WORegister<u32>,

    _reserved1: [u32; 1],

    /// Initial CRC value
    pub INIT: RWRegister<u32>,

    /// CRC polynomial
    pub POL: RWRegister<u32>,
}
pub struct ResetValues {
    pub DR: u32,
    pub IDR: u32,
    pub CR: u32,
    pub INIT: u32,
    pub POL: u32,
}
#[cfg(not(feature = "nosync"))]
pub struct Instance {
    pub(crate) addr: u32,
    pub(crate) _marker: PhantomData<*const RegisterBlock>,
}
#[cfg(not(feature = "nosync"))]
impl ::core::ops::Deref for Instance {
    type Target = RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &RegisterBlock {
        unsafe { &*(self.addr as *const _) }
    }
}
#[cfg(feature = "rtic")]
unsafe impl Send for Instance {}

/// Access functions for the CRC peripheral instance
pub mod CRC {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40023000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in CRC
    pub const reset: ResetValues = ResetValues {
        DR: 0x0000FFFF,
        IDR: 0x00000000,
        CR: 0x00000000,
        INIT: 0x00000000,
        POL: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut CRC_TAKEN: bool = false;

    /// Safe access to CRC
    ///
    /// This function returns `Some(Instance)` if this instance is not
    /// currently taken, and `None` if it is. This ensures that if you
    /// do get `Some(Instance)`, you are ensured unique access to
    /// the peripheral and there cannot be data races (unless other
    /// code uses `unsafe`, of course). You can then pass the
    /// `Instance` around to other functions as required. When you're
    /// done with it, you can call `release(instance)` to return it.
    ///
    /// `Instance` itself dereferences to a `RegisterBlock`, which
    /// provides access to the peripheral's registers.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn take() -> Option<Instance> {
        external_cortex_m::interrupt::free(|_| unsafe {
            if CRC_TAKEN {
                None
            } else {
                CRC_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to CRC
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if CRC_TAKEN && inst.addr == INSTANCE.addr {
                CRC_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal CRC
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        CRC_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to CRC
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const CRC: *const RegisterBlock = 0x40023000 as *const _;
