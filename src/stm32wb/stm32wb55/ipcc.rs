#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! IPCC

use crate::{RORegister, RWRegister, WORegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// Control register CPU1
pub mod C1CR {

    /// processor 1 Transmit channel free interrupt enable
    pub mod TXFIE {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// processor 1 Receive channel occupied interrupt enable
    pub mod RXOIE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Mask register CPU1
pub mod C1MR {

    /// processor 1 Transmit channel 6 free interrupt mask
    pub mod CH6FM {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// processor 1 Transmit channel 5 free interrupt mask
    pub mod CH5FM {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// processor 1 Transmit channel 4 free interrupt mask
    pub mod CH4FM {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// processor 1 Transmit channel 3 free interrupt mask
    pub mod CH3FM {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// processor 1 Transmit channel 2 free interrupt mask
    pub mod CH2FM {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// processor 1 Transmit channel 1 free interrupt mask
    pub mod CH1FM {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// processor 1 Receive channel 6 occupied interrupt enable
    pub mod CH6OM {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// processor 1 Receive channel 5 occupied interrupt enable
    pub mod CH5OM {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// processor 1 Receive channel 4 occupied interrupt enable
    pub mod CH4OM {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// processor 1 Receive channel 3 occupied interrupt enable
    pub mod CH3OM {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// processor 1 Receive channel 2 occupied interrupt enable
    pub mod CH2OM {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// processor 1 Receive channel 1 occupied interrupt enable
    pub mod CH1OM {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Status Set or Clear register CPU1
pub mod C1SCR {

    /// processor 1 Transmit channel 6 status set
    pub mod CH6S {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// processor 1 Transmit channel 5 status set
    pub mod CH5S {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// processor 1 Transmit channel 4 status set
    pub mod CH4S {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// processor 1 Transmit channel 3 status set
    pub mod CH3S {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// processor 1 Transmit channel 2 status set
    pub mod CH2S {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// processor 1 Transmit channel 1 status set
    pub mod CH1S {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// processor 1 Receive channel 6 status clear
    pub mod CH6C {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// processor 1 Receive channel 5 status clear
    pub mod CH5C {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// processor 1 Receive channel 4 status clear
    pub mod CH4C {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// processor 1 Receive channel 3 status clear
    pub mod CH3C {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// processor 1 Receive channel 2 status clear
    pub mod CH2C {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// processor 1 Receive channel 1 status clear
    pub mod CH1C {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// CPU1 to CPU2 status register
pub mod C1TOC2SR {

    /// processor 1 transmit to process 2 Receive channel 6 status flag
    pub mod CH6F {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// processor 1 transmit to process 2 Receive channel 5 status flag
    pub mod CH5F {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// processor 1 transmit to process 2 Receive channel 4 status flag
    pub mod CH4F {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// processor 1 transmit to process 2 Receive channel 3 status flag
    pub mod CH3F {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// processor 1 transmit to process 2 Receive channel 2 status flag
    pub mod CH2F {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// processor 1 transmit to process 2 Receive channel 1 status flag
    pub mod CH1F {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Control register CPU2
pub mod C2CR {
    pub use super::C1CR::RXOIE;
    pub use super::C1CR::TXFIE;
}

/// Mask register CPU2
pub mod C2MR {
    pub use super::C1MR::CH1FM;
    pub use super::C1MR::CH1OM;
    pub use super::C1MR::CH2FM;
    pub use super::C1MR::CH2OM;
    pub use super::C1MR::CH3FM;
    pub use super::C1MR::CH3OM;
    pub use super::C1MR::CH4FM;
    pub use super::C1MR::CH4OM;
    pub use super::C1MR::CH5FM;
    pub use super::C1MR::CH5OM;
    pub use super::C1MR::CH6FM;
    pub use super::C1MR::CH6OM;
}

/// Status Set or Clear register CPU2
pub mod C2SCR {
    pub use super::C1SCR::CH1C;
    pub use super::C1SCR::CH1S;
    pub use super::C1SCR::CH2C;
    pub use super::C1SCR::CH2S;
    pub use super::C1SCR::CH3C;
    pub use super::C1SCR::CH3S;
    pub use super::C1SCR::CH4C;
    pub use super::C1SCR::CH4S;
    pub use super::C1SCR::CH5C;
    pub use super::C1SCR::CH5S;
    pub use super::C1SCR::CH6C;
    pub use super::C1SCR::CH6S;
}

/// CPU2 to CPU1 status register
pub mod C2TOC1SR {
    pub use super::C1TOC2SR::CH1F;
    pub use super::C1TOC2SR::CH2F;
    pub use super::C1TOC2SR::CH3F;
    pub use super::C1TOC2SR::CH4F;
    pub use super::C1TOC2SR::CH5F;
    pub use super::C1TOC2SR::CH6F;
}

/// IPCC Hardware configuration register
pub mod HWCFGR {

    /// Number of channels per CPU supported by the IP, range 1 to 16
    pub mod CHANNELS {
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

/// IPCC version register
pub mod VERR {

    /// Major Revision
    pub mod MAJREV {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Minor Revision
    pub mod MINREV {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// IPCC indentification register
pub mod IPIDR {

    /// Identification Code
    pub mod IPID {
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

/// IPCC size indentification register
pub mod SIDR {

    /// Size Identification Code
    pub mod SID {
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
    /// Control register CPU1
    pub C1CR: RWRegister<u32>,

    /// Mask register CPU1
    pub C1MR: RWRegister<u32>,

    /// Status Set or Clear register CPU1
    pub C1SCR: WORegister<u32>,

    /// CPU1 to CPU2 status register
    pub C1TOC2SR: RORegister<u32>,

    /// Control register CPU2
    pub C2CR: RWRegister<u32>,

    /// Mask register CPU2
    pub C2MR: RWRegister<u32>,

    /// Status Set or Clear register CPU2
    pub C2SCR: WORegister<u32>,

    /// CPU2 to CPU1 status register
    pub C2TOC1SR: RORegister<u32>,

    _reserved1: [u32; 244],

    /// IPCC Hardware configuration register
    pub HWCFGR: RORegister<u32>,

    /// IPCC version register
    pub VERR: RORegister<u32>,

    /// IPCC indentification register
    pub IPIDR: RORegister<u32>,

    /// IPCC size indentification register
    pub SIDR: RORegister<u32>,
}
pub struct ResetValues {
    pub C1CR: u32,
    pub C1MR: u32,
    pub C1SCR: u32,
    pub C1TOC2SR: u32,
    pub C2CR: u32,
    pub C2MR: u32,
    pub C2SCR: u32,
    pub C2TOC1SR: u32,
    pub HWCFGR: u32,
    pub VERR: u32,
    pub IPIDR: u32,
    pub SIDR: u32,
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

/// Access functions for the IPCC peripheral instance
pub mod IPCC {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x58000c00,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in IPCC
    pub const reset: ResetValues = ResetValues {
        C1CR: 0x00000000,
        C1MR: 0xFFFFFFFF,
        C1SCR: 0x00000000,
        C1TOC2SR: 0x00000000,
        C2CR: 0x00000000,
        C2MR: 0xFFFFFFFF,
        C2SCR: 0x00000000,
        C2TOC1SR: 0x00000000,
        HWCFGR: 0x00000006,
        VERR: 0x00000010,
        IPIDR: 0x00100071,
        SIDR: 0xA3C5DD01,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut IPCC_TAKEN: bool = false;

    /// Safe access to IPCC
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
            if IPCC_TAKEN {
                None
            } else {
                IPCC_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to IPCC
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if IPCC_TAKEN && inst.addr == INSTANCE.addr {
                IPCC_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal IPCC
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        IPCC_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to IPCC
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const IPCC: *const RegisterBlock = 0x58000c00 as *const _;
