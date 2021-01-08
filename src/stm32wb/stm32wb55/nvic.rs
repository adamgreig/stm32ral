#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Nested Vectored Interrupt Controller

use crate::{RORegister, RWRegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// Interrupt Set-Enable Register
pub mod ISER0 {

    /// SETENA
    pub mod SETENA {
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

/// Interrupt Set-Enable Register
pub mod ISER1 {
    pub use super::ISER0::SETENA;
}

/// Interrupt Clear-Enable Register
pub mod ICER0 {

    /// CLRENA
    pub mod CLRENA {
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

/// Interrupt Clear-Enable Register
pub mod ICER1 {
    pub use super::ICER0::CLRENA;
}

/// Interrupt Set-Pending Register
pub mod ISPR0 {

    /// SETPEND
    pub mod SETPEND {
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

/// Interrupt Set-Pending Register
pub mod ISPR1 {
    pub use super::ISPR0::SETPEND;
}

/// Interrupt Clear-Pending Register
pub mod ICPR0 {

    /// CLRPEND
    pub mod CLRPEND {
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

/// Interrupt Clear-Pending Register
pub mod ICPR1 {
    pub use super::ICPR0::CLRPEND;
}

/// Interrupt Active Bit Register
pub mod IABR0 {

    /// ACTIVE
    pub mod ACTIVE {
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

/// Interrupt Active Bit Register
pub mod IABR1 {
    pub use super::IABR0::ACTIVE;
}

/// Interrupt Priority Register
pub mod IPR0 {

    /// IPR_N0
    pub mod IPR_N0 {
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

    /// IPR_N1
    pub mod IPR_N1 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (8 bits: 0xff << 8)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// IPR_N2
    pub mod IPR_N2 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (8 bits: 0xff << 16)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// IPR_N3
    pub mod IPR_N3 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (8 bits: 0xff << 24)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Priority Register
pub mod IPR1 {
    pub use super::IPR0::IPR_N0;
    pub use super::IPR0::IPR_N1;
    pub use super::IPR0::IPR_N2;
    pub use super::IPR0::IPR_N3;
}

/// Interrupt Priority Register
pub mod IPR2 {
    pub use super::IPR0::IPR_N0;
    pub use super::IPR0::IPR_N1;
    pub use super::IPR0::IPR_N2;
    pub use super::IPR0::IPR_N3;
}

/// Interrupt Priority Register
pub mod IPR3 {
    pub use super::IPR0::IPR_N0;
    pub use super::IPR0::IPR_N1;
    pub use super::IPR0::IPR_N2;
    pub use super::IPR0::IPR_N3;
}

/// Interrupt Priority Register
pub mod IPR4 {
    pub use super::IPR0::IPR_N0;
    pub use super::IPR0::IPR_N1;
    pub use super::IPR0::IPR_N2;
    pub use super::IPR0::IPR_N3;
}

/// Interrupt Priority Register
pub mod IPR5 {
    pub use super::IPR0::IPR_N0;
    pub use super::IPR0::IPR_N1;
    pub use super::IPR0::IPR_N2;
    pub use super::IPR0::IPR_N3;
}

/// Interrupt Priority Register
pub mod IPR6 {
    pub use super::IPR0::IPR_N0;
    pub use super::IPR0::IPR_N1;
    pub use super::IPR0::IPR_N2;
    pub use super::IPR0::IPR_N3;
}

/// Interrupt Priority Register
pub mod IPR7 {
    pub use super::IPR0::IPR_N0;
    pub use super::IPR0::IPR_N1;
    pub use super::IPR0::IPR_N2;
    pub use super::IPR0::IPR_N3;
}

/// Interrupt Priority Register
pub mod IPR8 {
    pub use super::IPR0::IPR_N0;
    pub use super::IPR0::IPR_N1;
    pub use super::IPR0::IPR_N2;
    pub use super::IPR0::IPR_N3;
}

/// Interrupt Priority Register
pub mod IPR9 {
    pub use super::IPR0::IPR_N0;
    pub use super::IPR0::IPR_N1;
    pub use super::IPR0::IPR_N2;
    pub use super::IPR0::IPR_N3;
}

/// Interrupt Priority Register
pub mod IPR10 {
    pub use super::IPR0::IPR_N0;
    pub use super::IPR0::IPR_N1;
    pub use super::IPR0::IPR_N2;
    pub use super::IPR0::IPR_N3;
}

/// Interrupt Priority Register
pub mod IPR11 {
    pub use super::IPR0::IPR_N0;
    pub use super::IPR0::IPR_N1;
    pub use super::IPR0::IPR_N2;
    pub use super::IPR0::IPR_N3;
}

/// Interrupt Priority Register
pub mod IPR12 {
    pub use super::IPR0::IPR_N0;
    pub use super::IPR0::IPR_N1;
    pub use super::IPR0::IPR_N2;
    pub use super::IPR0::IPR_N3;
}

/// Interrupt Priority Register
pub mod IPR13 {
    pub use super::IPR0::IPR_N0;
    pub use super::IPR0::IPR_N1;
    pub use super::IPR0::IPR_N2;
    pub use super::IPR0::IPR_N3;
}

/// Interrupt Priority Register
pub mod IPR14 {
    pub use super::IPR0::IPR_N0;
    pub use super::IPR0::IPR_N1;
    pub use super::IPR0::IPR_N2;
    pub use super::IPR0::IPR_N3;
}

/// Interrupt Priority Register
pub mod IPR15 {
    pub use super::IPR0::IPR_N0;
    pub use super::IPR0::IPR_N1;
    pub use super::IPR0::IPR_N2;
    pub use super::IPR0::IPR_N3;
}

/// Interrupt Priority Register
pub mod IPR16 {
    pub use super::IPR0::IPR_N0;
    pub use super::IPR0::IPR_N1;
    pub use super::IPR0::IPR_N2;
    pub use super::IPR0::IPR_N3;
}

/// Interrupt Priority Register
pub mod IPR17 {
    pub use super::IPR0::IPR_N0;
    pub use super::IPR0::IPR_N1;
    pub use super::IPR0::IPR_N2;
    pub use super::IPR0::IPR_N3;
}
#[repr(C)]
pub struct RegisterBlock {
    /// Interrupt Set-Enable Register
    pub ISER0: RWRegister<u32>,

    /// Interrupt Set-Enable Register
    pub ISER1: RWRegister<u32>,

    _reserved1: [u32; 30],

    /// Interrupt Clear-Enable Register
    pub ICER0: RWRegister<u32>,

    /// Interrupt Clear-Enable Register
    pub ICER1: RWRegister<u32>,

    _reserved2: [u32; 30],

    /// Interrupt Set-Pending Register
    pub ISPR0: RWRegister<u32>,

    /// Interrupt Set-Pending Register
    pub ISPR1: RWRegister<u32>,

    _reserved3: [u32; 30],

    /// Interrupt Clear-Pending Register
    pub ICPR0: RWRegister<u32>,

    /// Interrupt Clear-Pending Register
    pub ICPR1: RWRegister<u32>,

    _reserved4: [u32; 30],

    /// Interrupt Active Bit Register
    pub IABR0: RORegister<u32>,

    /// Interrupt Active Bit Register
    pub IABR1: RORegister<u32>,

    _reserved5: [u32; 62],

    /// Interrupt Priority Register
    pub IPR0: RWRegister<u32>,

    /// Interrupt Priority Register
    pub IPR1: RWRegister<u32>,

    /// Interrupt Priority Register
    pub IPR2: RWRegister<u32>,

    /// Interrupt Priority Register
    pub IPR3: RWRegister<u32>,

    /// Interrupt Priority Register
    pub IPR4: RWRegister<u32>,

    /// Interrupt Priority Register
    pub IPR5: RWRegister<u32>,

    /// Interrupt Priority Register
    pub IPR6: RWRegister<u32>,

    /// Interrupt Priority Register
    pub IPR7: RWRegister<u32>,

    /// Interrupt Priority Register
    pub IPR8: RWRegister<u32>,

    /// Interrupt Priority Register
    pub IPR9: RWRegister<u32>,

    /// Interrupt Priority Register
    pub IPR10: RWRegister<u32>,

    /// Interrupt Priority Register
    pub IPR11: RWRegister<u32>,

    /// Interrupt Priority Register
    pub IPR12: RWRegister<u32>,

    /// Interrupt Priority Register
    pub IPR13: RWRegister<u32>,

    /// Interrupt Priority Register
    pub IPR14: RWRegister<u32>,

    /// Interrupt Priority Register
    pub IPR15: RWRegister<u32>,

    /// Interrupt Priority Register
    pub IPR16: RWRegister<u32>,

    /// Interrupt Priority Register
    pub IPR17: RWRegister<u32>,
}
pub struct ResetValues {
    pub ISER0: u32,
    pub ISER1: u32,
    pub ICER0: u32,
    pub ICER1: u32,
    pub ISPR0: u32,
    pub ISPR1: u32,
    pub ICPR0: u32,
    pub ICPR1: u32,
    pub IABR0: u32,
    pub IABR1: u32,
    pub IPR0: u32,
    pub IPR1: u32,
    pub IPR2: u32,
    pub IPR3: u32,
    pub IPR4: u32,
    pub IPR5: u32,
    pub IPR6: u32,
    pub IPR7: u32,
    pub IPR8: u32,
    pub IPR9: u32,
    pub IPR10: u32,
    pub IPR11: u32,
    pub IPR12: u32,
    pub IPR13: u32,
    pub IPR14: u32,
    pub IPR15: u32,
    pub IPR16: u32,
    pub IPR17: u32,
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

/// Access functions for the NVIC peripheral instance
pub mod NVIC {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0xe000e100,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in NVIC
    pub const reset: ResetValues = ResetValues {
        ISER0: 0x00000000,
        ISER1: 0x00000000,
        ICER0: 0x00000000,
        ICER1: 0x00000000,
        ISPR0: 0x00000000,
        ISPR1: 0x00000000,
        ICPR0: 0x00000000,
        ICPR1: 0x00000000,
        IABR0: 0x00000000,
        IABR1: 0x00000000,
        IPR0: 0x00000000,
        IPR1: 0x00000000,
        IPR2: 0x00000000,
        IPR3: 0x00000000,
        IPR4: 0x00000000,
        IPR5: 0x00000000,
        IPR6: 0x00000000,
        IPR7: 0x00000000,
        IPR8: 0x00000000,
        IPR9: 0x00000000,
        IPR10: 0x00000000,
        IPR11: 0x00000000,
        IPR12: 0x00000000,
        IPR13: 0x00000000,
        IPR14: 0x00000000,
        IPR15: 0x00000000,
        IPR16: 0x00000000,
        IPR17: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut NVIC_TAKEN: bool = false;

    /// Safe access to NVIC
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
            if NVIC_TAKEN {
                None
            } else {
                NVIC_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to NVIC
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if NVIC_TAKEN && inst.addr == INSTANCE.addr {
                NVIC_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal NVIC
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        NVIC_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to NVIC
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const NVIC: *const RegisterBlock = 0xe000e100 as *const _;
