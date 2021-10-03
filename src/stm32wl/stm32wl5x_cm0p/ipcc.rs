#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Inter Processor communication controller

use crate::{RORegister, RWRegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// IPCC Processor 1 control register
pub mod C1CR {

    /// RXOIE
    pub mod RXOIE {
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

            /// 0b1: Enable an unmasked processor receive channel occupied to generate an RX occupied interrupt
            pub const Enabled: u32 = 0b1;

            /// 0b0: Processor RX occupied interrupt disabled
            pub const Disabled: u32 = 0b0;
        }
    }

    /// TXFIE
    pub mod TXFIE {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b1: Enable an unmasked processor transmit channel free to generate a TX free interrupt
            pub const Enabled: u32 = 0b1;

            /// 0b0: Processor TX free interrupt disabled
            pub const Disabled: u32 = 0b0;
        }
    }
}

/// IPCC Processor 1 mask register
pub mod C1MR {

    /// CH1OM
    pub mod CH1OM {
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

            /// 0b1: Receive channel n occupied interrupt masked
            pub const Masked: u32 = 0b1;

            /// 0b0: Receive channel n occupied interrupt not masked
            pub const Unmasked: u32 = 0b0;
        }
    }

    /// CH2OM
    pub mod CH2OM {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CH1OM::RW;
    }

    /// CH3OM
    pub mod CH3OM {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CH1OM::RW;
    }

    /// CH4OM
    pub mod CH4OM {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CH1OM::RW;
    }

    /// CH5OM
    pub mod CH5OM {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CH1OM::RW;
    }

    /// CH6OM
    pub mod CH6OM {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CH1OM::RW;
    }

    /// CH1FM
    pub mod CH1FM {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b1: Transmit channel n free interrupt masked
            pub const Masked: u32 = 0b1;

            /// 0b0: Transmit channel n free interrupt not masked
            pub const Unmasked: u32 = 0b0;
        }
    }

    /// CH2FM
    pub mod CH2FM {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CH1FM::RW;
    }

    /// CH3FM
    pub mod CH3FM {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CH1FM::RW;
    }

    /// CH4FM
    pub mod CH4FM {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CH1FM::RW;
    }

    /// CH5FM
    pub mod CH5FM {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CH1FM::RW;
    }

    /// CH6FM
    pub mod CH6FM {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CH1FM::RW;
    }
}

/// Reading this register will always return 0x0000 0000.
pub mod C1SCR {

    /// CH1C
    pub mod CH1C {
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

            /// 0b1: Processor receive channel n status bit clear
            pub const Clear: u32 = 0b1;

            /// 0b0: No action
            pub const NoAction: u32 = 0b0;
        }
    }

    /// CH2C
    pub mod CH2C {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CH1C::RW;
    }

    /// CH3C
    pub mod CH3C {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CH1C::RW;
    }

    /// CH4C
    pub mod CH4C {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CH1C::RW;
    }

    /// CH5C
    pub mod CH5C {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CH1C::RW;
    }

    /// CH6C
    pub mod CH6C {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CH1C::RW;
    }

    /// CH1S
    pub mod CH1S {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b1: Processor transmit channel n status bit set
            pub const Set: u32 = 0b1;

            /// 0b0: No action
            pub const NoAction: u32 = 0b0;
        }
    }

    /// CH2S
    pub mod CH2S {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CH1S::RW;
    }

    /// CH3S
    pub mod CH3S {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CH1S::RW;
    }

    /// CH4S
    pub mod CH4S {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CH1S::RW;
    }

    /// CH5S
    pub mod CH5S {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CH1S::RW;
    }

    /// CH6S
    pub mod CH6S {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CH1S::RW;
    }
}

/// IPCC processor 1 to processor 2 status register
pub mod C1TOC2SR {

    /// CH1F
    pub mod CH1F {
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

            /// 0b1: Channel occupied, data can be read by the receiving processor. Generates a channel RX occupied interrupt to the other processor, when unmasked
            pub const Occupied: u32 = 0b1;

            /// 0b0: Channel free, data can be written by the sending processor. Generates a channel TX free interrupt to the current processor, when unmasked
            pub const Free: u32 = 0b0;
        }
    }

    /// CH2F
    pub mod CH2F {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CH1F::RW;
    }

    /// CH3F
    pub mod CH3F {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CH1F::RW;
    }

    /// CH4F
    pub mod CH4F {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CH1F::RW;
    }

    /// CH5F
    pub mod CH5F {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CH1F::RW;
    }

    /// CH6F
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
}

/// IPCC Processor 2 control register
pub mod C2CR {
    pub use super::C1CR::RXOIE;
    pub use super::C1CR::TXFIE;
}

/// IPCC Processor 2 mask register
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

/// Reading this register will always return 0x0000 0000.
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

/// IPCC processor 2 to processor 1 status register
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

    /// CHANNELS
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

/// IPCC IP Version register
pub mod VERR {

    /// MINREV
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

    /// MAJREV
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
}

/// IPCC IP Identification register
pub mod IPIDR {

    /// ID
    pub mod ID {
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

/// IPCC Size ID register
pub mod SIDR {

    /// SID
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
    /// IPCC Processor 1 control register
    pub C1CR: RWRegister<u32>,

    /// IPCC Processor 1 mask register
    pub C1MR: RWRegister<u32>,

    /// Reading this register will always return 0x0000 0000.
    pub C1SCR: RWRegister<u32>,

    /// IPCC processor 1 to processor 2 status register
    pub C1TOC2SR: RORegister<u32>,

    /// IPCC Processor 2 control register
    pub C2CR: RWRegister<u32>,

    /// IPCC Processor 2 mask register
    pub C2MR: RWRegister<u32>,

    /// Reading this register will always return 0x0000 0000.
    pub C2SCR: RWRegister<u32>,

    /// IPCC processor 2 to processor 1 status register
    pub C2TOC1SR: RORegister<u32>,

    _reserved1: [u32; 244],

    /// IPCC Hardware configuration register
    pub HWCFGR: RORegister<u32>,

    /// IPCC IP Version register
    pub VERR: RORegister<u32>,

    /// IPCC IP Identification register
    pub IPIDR: RORegister<u32>,

    /// IPCC Size ID register
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
