#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! On-The-Fly Decryption engine

use crate::{RORegister, RWRegister, WORegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// OTFDEC control register
pub mod CR {

    /// Encryption mode bit
    pub mod ENC {
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

/// OTFDEC region x configuration register
pub mod R1CFGR {

    /// region on-the-fly decryption enable
    pub mod REG_EN {
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

    /// region config lock
    pub mod CONFIGLOCK {
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

    /// region key lock
    pub mod KEYLOCK {
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

    /// operating mode
    pub mod MODE {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (2 bits: 0b11 << 4)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// region key 8-bit CRC
    pub mod KEYCRC {
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

    /// region firmware version
    pub mod REGx_VERSION {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (16 bits: 0xffff << 16)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// OTFDEC region x configuration register
pub mod R2CFGR {
    pub use super::R1CFGR::REGx_VERSION;
    pub use super::R1CFGR::CONFIGLOCK;
    pub use super::R1CFGR::KEYCRC;
    pub use super::R1CFGR::KEYLOCK;
    pub use super::R1CFGR::MODE;
    pub use super::R1CFGR::REG_EN;
}

/// OTFDEC region x configuration register
pub mod R3CFGR {
    pub use super::R1CFGR::REGx_VERSION;
    pub use super::R1CFGR::CONFIGLOCK;
    pub use super::R1CFGR::KEYCRC;
    pub use super::R1CFGR::KEYLOCK;
    pub use super::R1CFGR::MODE;
    pub use super::R1CFGR::REG_EN;
}

/// OTFDEC region x configuration register
pub mod R4CFGR {
    pub use super::R1CFGR::REGx_VERSION;
    pub use super::R1CFGR::CONFIGLOCK;
    pub use super::R1CFGR::KEYCRC;
    pub use super::R1CFGR::KEYLOCK;
    pub use super::R1CFGR::MODE;
    pub use super::R1CFGR::REG_EN;
}

/// OTFDEC region x start address register
pub mod R1STARTADDR {

    /// Region AXI start address
    pub mod REGx_START_ADDR {
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

/// OTFDEC region x start address register
pub mod R2STARTADDR {
    pub use super::R1STARTADDR::REGx_START_ADDR;
}

/// OTFDEC region x start address register
pub mod R3STARTADDR {
    pub use super::R1STARTADDR::REGx_START_ADDR;
}

/// OTFDEC region x start address register
pub mod R4STARTADDR {
    pub use super::R1STARTADDR::REGx_START_ADDR;
}

/// OTFDEC region x end address register
pub mod R1ENDADDR {

    /// Region AXI end address
    pub mod REGx_END_ADDR {
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

/// OTFDEC region x end address register
pub mod R2ENDADDR {
    pub use super::R1ENDADDR::REGx_END_ADDR;
}

/// OTFDEC region x end address register
pub mod R3ENDADDR {
    pub use super::R1ENDADDR::REGx_END_ADDR;
}

/// OTFDEC region x end address register
pub mod R4ENDADDR {
    pub use super::R1ENDADDR::REGx_END_ADDR;
}

/// OTFDEC region x nonce register 0
pub mod R1NONCER0 {

    /// REGx_NONCE
    pub mod REGx_NONCE {
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

/// OTFDEC region x nonce register 0
pub mod R2NONCER0 {
    pub use super::R1NONCER0::REGx_NONCE;
}

/// OTFDEC region x nonce register 0
pub mod R3NONCER0 {
    pub use super::R1NONCER0::REGx_NONCE;
}

/// OTFDEC region x nonce register 0
pub mod R4NONCER0 {
    pub use super::R1NONCER0::REGx_NONCE;
}

/// OTFDEC region x nonce register 1
pub mod R1NONCER1 {
    pub use super::R1NONCER0::REGx_NONCE;
}

/// OTFDEC region x nonce register 1
pub mod R2NONCER1 {
    pub use super::R1NONCER0::REGx_NONCE;
}

/// OTFDEC region x nonce register 1
pub mod R3NONCER1 {
    pub use super::R1NONCER0::REGx_NONCE;
}

/// OTFDEC region x nonce register 1
pub mod R4NONCER1 {
    pub use super::R1NONCER0::REGx_NONCE;
}

/// OTFDEC region x key register 0
pub mod R1KEYR0 {

    /// REGx_KEY
    pub mod REGx_KEY {
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

/// OTFDEC region x key register 0
pub mod R2KEYR0 {
    pub use super::R1KEYR0::REGx_KEY;
}

/// OTFDEC region x key register 0
pub mod R3KEYR0 {
    pub use super::R1KEYR0::REGx_KEY;
}

/// OTFDEC region x key register 0
pub mod R4KEYR0 {
    pub use super::R1KEYR0::REGx_KEY;
}

/// OTFDEC region x key register 1
pub mod R1KEYR1 {
    pub use super::R1KEYR0::REGx_KEY;
}

/// OTFDEC region x key register 1
pub mod R2KEYR1 {
    pub use super::R1KEYR0::REGx_KEY;
}

/// OTFDEC region x key register 1
pub mod R3KEYR1 {
    pub use super::R1KEYR0::REGx_KEY;
}

/// OTFDEC region x key register 1
pub mod R4KEYR1 {
    pub use super::R1KEYR0::REGx_KEY;
}

/// OTFDEC region x key register 2
pub mod R1KEYR2 {
    pub use super::R1KEYR0::REGx_KEY;
}

/// OTFDEC region x key register 2
pub mod R2KEYR2 {

    /// REGx_KEY
    pub mod REGx_KEY_ {
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

/// OTFDEC region x key register 2
pub mod R3KEYR2 {
    pub use super::R1KEYR0::REGx_KEY;
}

/// OTFDEC region x key register 2
pub mod R4KEYR2 {
    pub use super::R1KEYR0::REGx_KEY;
}

/// OTFDEC region x key register 3
pub mod R1KEYR3 {
    pub use super::R1KEYR0::REGx_KEY;
}

/// OTFDEC region x key register 3
pub mod R2KEYR3 {
    pub use super::R1KEYR0::REGx_KEY;
}

/// OTFDEC region x key register 3
pub mod R3KEYR3 {
    pub use super::R1KEYR0::REGx_KEY;
}

/// OTFDEC region x key register 3
pub mod R4KEYR3 {
    pub use super::R1KEYR0::REGx_KEY;
}

/// OTFDEC interrupt status register
pub mod ISR {

    /// Security Error Interrupt Flag status
    pub mod SEIF {
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

    /// Execute-only execute-Never Error Interrupt Flag status
    pub mod XONEIF {
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

    /// Key Error Interrupt Flag status
    pub mod KEIF {
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
}

/// OTFDEC interrupt clear register
pub mod ICR {

    /// SEIF
    pub mod SEIF {
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

    /// Execute-only execute-Never Error Interrupt Flag clear
    pub mod XONEIF {
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

    /// KEIF
    pub mod KEIF {
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
}

/// OTFDEC interrupt enable register
pub mod IER {

    /// Security Error Interrupt Enable
    pub mod SEIE {
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

    /// XONEIE
    pub mod XONEIE {
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

    /// KEIE
    pub mod KEIE {
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
}
#[repr(C)]
pub struct RegisterBlock {
    /// OTFDEC control register
    pub CR: RWRegister<u32>,

    _reserved1: [u32; 7],

    /// OTFDEC region x configuration register
    pub R1CFGR: RWRegister<u32>,

    /// OTFDEC region x start address register
    pub R1STARTADDR: RWRegister<u32>,

    /// OTFDEC region x end address register
    pub R1ENDADDR: RWRegister<u32>,

    /// OTFDEC region x nonce register 0
    pub R1NONCER0: RWRegister<u32>,

    /// OTFDEC region x nonce register 1
    pub R1NONCER1: RWRegister<u32>,

    /// OTFDEC region x key register 0
    pub R1KEYR0: WORegister<u32>,

    /// OTFDEC region x key register 1
    pub R1KEYR1: WORegister<u32>,

    /// OTFDEC region x key register 2
    pub R1KEYR2: WORegister<u32>,

    /// OTFDEC region x key register 3
    pub R1KEYR3: WORegister<u32>,

    _reserved2: [u32; 3],

    /// OTFDEC region x configuration register
    pub R2CFGR: RWRegister<u32>,

    /// OTFDEC region x start address register
    pub R2STARTADDR: RWRegister<u32>,

    /// OTFDEC region x end address register
    pub R2ENDADDR: RWRegister<u32>,

    /// OTFDEC region x nonce register 0
    pub R2NONCER0: RWRegister<u32>,

    /// OTFDEC region x nonce register 1
    pub R2NONCER1: RWRegister<u32>,

    /// OTFDEC region x key register 0
    pub R2KEYR0: WORegister<u32>,

    /// OTFDEC region x key register 1
    pub R2KEYR1: WORegister<u32>,

    /// OTFDEC region x key register 2
    pub R2KEYR2: WORegister<u32>,

    /// OTFDEC region x key register 3
    pub R2KEYR3: WORegister<u32>,

    _reserved3: [u32; 3],

    /// OTFDEC region x configuration register
    pub R3CFGR: RWRegister<u32>,

    /// OTFDEC region x start address register
    pub R3STARTADDR: RWRegister<u32>,

    /// OTFDEC region x end address register
    pub R3ENDADDR: RWRegister<u32>,

    /// OTFDEC region x nonce register 0
    pub R3NONCER0: RWRegister<u32>,

    /// OTFDEC region x nonce register 1
    pub R3NONCER1: RWRegister<u32>,

    /// OTFDEC region x key register 0
    pub R3KEYR0: WORegister<u32>,

    /// OTFDEC region x key register 1
    pub R3KEYR1: WORegister<u32>,

    /// OTFDEC region x key register 2
    pub R3KEYR2: WORegister<u32>,

    /// OTFDEC region x key register 3
    pub R3KEYR3: WORegister<u32>,

    _reserved4: [u32; 3],

    /// OTFDEC region x configuration register
    pub R4CFGR: RWRegister<u32>,

    /// OTFDEC region x start address register
    pub R4STARTADDR: RWRegister<u32>,

    /// OTFDEC region x end address register
    pub R4ENDADDR: RWRegister<u32>,

    /// OTFDEC region x nonce register 0
    pub R4NONCER0: RWRegister<u32>,

    /// OTFDEC region x nonce register 1
    pub R4NONCER1: RWRegister<u32>,

    /// OTFDEC region x key register 0
    pub R4KEYR0: WORegister<u32>,

    /// OTFDEC region x key register 1
    pub R4KEYR1: WORegister<u32>,

    /// OTFDEC region x key register 2
    pub R4KEYR2: WORegister<u32>,

    /// OTFDEC region x key register 3
    pub R4KEYR3: WORegister<u32>,

    _reserved5: [u32; 139],

    /// OTFDEC interrupt status register
    pub ISR: RORegister<u32>,

    /// OTFDEC interrupt clear register
    pub ICR: WORegister<u32>,

    /// OTFDEC interrupt enable register
    pub IER: RWRegister<u32>,
}
pub struct ResetValues {
    pub CR: u32,
    pub R1CFGR: u32,
    pub R1STARTADDR: u32,
    pub R1ENDADDR: u32,
    pub R1NONCER0: u32,
    pub R1NONCER1: u32,
    pub R1KEYR0: u32,
    pub R1KEYR1: u32,
    pub R1KEYR2: u32,
    pub R1KEYR3: u32,
    pub R2CFGR: u32,
    pub R2STARTADDR: u32,
    pub R2ENDADDR: u32,
    pub R2NONCER0: u32,
    pub R2NONCER1: u32,
    pub R2KEYR0: u32,
    pub R2KEYR1: u32,
    pub R2KEYR2: u32,
    pub R2KEYR3: u32,
    pub R3CFGR: u32,
    pub R3STARTADDR: u32,
    pub R3ENDADDR: u32,
    pub R3NONCER0: u32,
    pub R3NONCER1: u32,
    pub R3KEYR0: u32,
    pub R3KEYR1: u32,
    pub R3KEYR2: u32,
    pub R3KEYR3: u32,
    pub R4CFGR: u32,
    pub R4STARTADDR: u32,
    pub R4ENDADDR: u32,
    pub R4NONCER0: u32,
    pub R4NONCER1: u32,
    pub R4KEYR0: u32,
    pub R4KEYR1: u32,
    pub R4KEYR2: u32,
    pub R4KEYR3: u32,
    pub ISR: u32,
    pub ICR: u32,
    pub IER: u32,
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

/// Access functions for the OTFDEC1 peripheral instance
pub mod OTFDEC1 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x420c5000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in OTFDEC1
    pub const reset: ResetValues = ResetValues {
        CR: 0x00000000,
        R1CFGR: 0x00000000,
        R2CFGR: 0x00000000,
        R3CFGR: 0x00000000,
        R4CFGR: 0x00000000,
        R1STARTADDR: 0x00000000,
        R2STARTADDR: 0x00000000,
        R3STARTADDR: 0x00000000,
        R4STARTADDR: 0x00000000,
        R1ENDADDR: 0x00000FFF,
        R2ENDADDR: 0x00000FFF,
        R3ENDADDR: 0x00000FFF,
        R4ENDADDR: 0x00000FFF,
        R1NONCER0: 0x00000000,
        R2NONCER0: 0x00000000,
        R3NONCER0: 0x00000000,
        R4NONCER0: 0x00000000,
        R1NONCER1: 0x00000000,
        R2NONCER1: 0x00000000,
        R3NONCER1: 0x00000000,
        R4NONCER1: 0x00000000,
        R1KEYR0: 0x00000000,
        R2KEYR0: 0x00000000,
        R3KEYR0: 0x00000000,
        R4KEYR0: 0x00000000,
        R1KEYR1: 0x00000000,
        R2KEYR1: 0x00000000,
        R3KEYR1: 0x00000000,
        R4KEYR1: 0x00000000,
        R1KEYR2: 0x00000000,
        R2KEYR2: 0x00000000,
        R3KEYR2: 0x00000000,
        R4KEYR2: 0x00000000,
        R1KEYR3: 0x00000000,
        R2KEYR3: 0x00000000,
        R3KEYR3: 0x00000000,
        R4KEYR3: 0x00000000,
        ISR: 0x00000000,
        ICR: 0x00000000,
        IER: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut OTFDEC1_TAKEN: bool = false;

    /// Safe access to OTFDEC1
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
            if OTFDEC1_TAKEN {
                None
            } else {
                OTFDEC1_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to OTFDEC1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if OTFDEC1_TAKEN && inst.addr == INSTANCE.addr {
                OTFDEC1_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal OTFDEC1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        OTFDEC1_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to OTFDEC1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const OTFDEC1: *const RegisterBlock = 0x420c5000 as *const _;

/// Access functions for the SEC_OTFDEC1 peripheral instance
pub mod SEC_OTFDEC1 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x520c5000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in SEC_OTFDEC1
    pub const reset: ResetValues = ResetValues {
        CR: 0x00000000,
        R1CFGR: 0x00000000,
        R2CFGR: 0x00000000,
        R3CFGR: 0x00000000,
        R4CFGR: 0x00000000,
        R1STARTADDR: 0x00000000,
        R2STARTADDR: 0x00000000,
        R3STARTADDR: 0x00000000,
        R4STARTADDR: 0x00000000,
        R1ENDADDR: 0x00000FFF,
        R2ENDADDR: 0x00000FFF,
        R3ENDADDR: 0x00000FFF,
        R4ENDADDR: 0x00000FFF,
        R1NONCER0: 0x00000000,
        R2NONCER0: 0x00000000,
        R3NONCER0: 0x00000000,
        R4NONCER0: 0x00000000,
        R1NONCER1: 0x00000000,
        R2NONCER1: 0x00000000,
        R3NONCER1: 0x00000000,
        R4NONCER1: 0x00000000,
        R1KEYR0: 0x00000000,
        R2KEYR0: 0x00000000,
        R3KEYR0: 0x00000000,
        R4KEYR0: 0x00000000,
        R1KEYR1: 0x00000000,
        R2KEYR1: 0x00000000,
        R3KEYR1: 0x00000000,
        R4KEYR1: 0x00000000,
        R1KEYR2: 0x00000000,
        R2KEYR2: 0x00000000,
        R3KEYR2: 0x00000000,
        R4KEYR2: 0x00000000,
        R1KEYR3: 0x00000000,
        R2KEYR3: 0x00000000,
        R3KEYR3: 0x00000000,
        R4KEYR3: 0x00000000,
        ISR: 0x00000000,
        ICR: 0x00000000,
        IER: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut SEC_OTFDEC1_TAKEN: bool = false;

    /// Safe access to SEC_OTFDEC1
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
            if SEC_OTFDEC1_TAKEN {
                None
            } else {
                SEC_OTFDEC1_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to SEC_OTFDEC1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if SEC_OTFDEC1_TAKEN && inst.addr == INSTANCE.addr {
                SEC_OTFDEC1_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal SEC_OTFDEC1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        SEC_OTFDEC1_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to SEC_OTFDEC1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const SEC_OTFDEC1: *const RegisterBlock = 0x520c5000 as *const _;
