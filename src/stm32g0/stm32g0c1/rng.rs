#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Random number generator

use crate::{RORegister, RWRegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// control register
pub mod RNG_CR {

    /// True random number generator enable
    pub mod RNGEN {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: True random number generator is disabled. Analog noise sources are powered off and logic clocked by the RNG clock is gated.
            pub const B_0x0: u32 = 0b0;

            /// 0b1: True random number generator is enabled.
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Interrupt Enable
    pub mod IE {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: RNG Interrupt is disabled
            pub const B_0x0: u32 = 0b0;

            /// 0b1: RNG Interrupt is enabled. An interrupt is pending as soon as DRDY='1', SEIS='1' or CEIS=1 in the RNG_SR register.
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Clock error detection The clock error detection cannot be enabled nor disabled on-the-fly when the RNG is enabled, i.e. to enable or disable CED the RNG must be disabled.
    pub mod CED {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Clock error detection is enable
            pub const B_0x0: u32 = 0b0;

            /// 0b1: Clock error detection is disable
            pub const B_0x1: u32 = 0b1;
        }
    }
}

/// status register
pub mod RNG_SR {

    /// Data Ready Once the output buffer becomes empty (after reading the RNG_DR register), this bit returns to 0 until a new random value is generated. Note: The DRDY bit can rise when the peripheral is disabled (RNGEN=0 in the RNG_CR register). If IE=1 in the RNG_CR register, an interrupt is generated when DRDY=1.
    pub mod DRDY {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: The RNG_DR register is not yet valid, no random data is available.
            pub const B_0x0: u32 = 0b0;

            /// 0b1: The RNG_DR register contains valid random data.
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Clock error current status Note: CECS bit is valid only if the CED bit in the RNG_CR register is set to 0.
    pub mod CECS {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: The RNG clock is correct (fRNGCLK> fHCLK/32). If the CEIS bit is set, this means that a slow clock was detected and the situation has been recovered.
            pub const B_0x0: u32 = 0b0;

            /// 0b1: The RNG clock is too slow (fRNGCLK< fHCLK/32).
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Seed error current status One of the noise source has provided more than 64 consecutive bits at a constant value (â0â or â1â), or more than 32 consecutive occurrence of two bit patterns (â01â or â10â) Both noise sources have delivered more than 32 consecutive bits at a constant value (â0â or â1â), or more than 16 consecutive occurrence of two bit patterns (â01â or â10â)
    pub mod SECS {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No faulty sequence has currently been detected. If the SEIS bit is set, this means that a faulty sequence was detected and the situation has been recovered.
            pub const B_0x0: u32 = 0b0;

            /// 0b1: At least one of the following faulty sequence has been detected:
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Clock error interrupt status This bit is set at the same time as CECS. It is cleared by writing 0. Writing 1 has no effect. An interrupt is pending if IE = 1 in the RNG_CR register.
    pub mod CEIS {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: The RNG clock is correct (fRNGCLK> fHCLK/32)
            pub const B_0x0: u32 = 0b0;

            /// 0b1: The RNG has been detected too slow (fRNGCLK< fHCLK/32)
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Seed error interrupt status This bit is set at the same time as SECS. It is cleared by writing 0. Writing 1 has no effect. An interrupt is pending if IE = 1 in the RNG_CR register.
    pub mod SEIS {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No faulty sequence detected
            pub const B_0x0: u32 = 0b0;

            /// 0b1: At least one faulty sequence has been detected. See SECS bit description for details.
            pub const B_0x1: u32 = 0b1;
        }
    }
}

/// data register
pub mod RNG_DR {

    /// Random data
    pub mod RNDATA {
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
    /// control register
    pub RNG_CR: RWRegister<u32>,

    /// status register
    pub RNG_SR: RWRegister<u32>,

    /// data register
    pub RNG_DR: RORegister<u32>,
}
pub struct ResetValues {
    pub RNG_CR: u32,
    pub RNG_SR: u32,
    pub RNG_DR: u32,
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

/// Access functions for the RNG peripheral instance
pub mod RNG {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40025000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in RNG
    pub const reset: ResetValues = ResetValues {
        RNG_CR: 0x00000000,
        RNG_SR: 0x00000000,
        RNG_DR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut RNG_TAKEN: bool = false;

    /// Safe access to RNG
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
            if RNG_TAKEN {
                None
            } else {
                RNG_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to RNG
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if RNG_TAKEN && inst.addr == INSTANCE.addr {
                RNG_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal RNG
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        RNG_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to RNG
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const RNG: *const RegisterBlock = 0x40025000 as *const _;
