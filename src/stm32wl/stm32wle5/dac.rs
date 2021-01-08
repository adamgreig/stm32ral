#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Digital-to-analog converter

use crate::{RORegister, RWRegister, WORegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// control register
pub mod CR {

    /// DAC Channel 1 calibration enable
    pub mod CEN1 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DAC channel1 DMA Underrun Interrupt enable
    pub mod DMAUDRIE1 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DAC channel1 DMA enable
    pub mod DMAEN1 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DAC channel1 mask/amplitude selector
    pub mod MAMP1 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (4 bits: 0b1111 << 8)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DAC channel1 noise/triangle wave generation enable
    pub mod WAVE1 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (2 bits: 0b11 << 6)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DAC channel1 trigger selection
    pub mod TSEL13 {
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

    /// TSEL12
    pub mod TSEL12 {
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

    /// TSEL11
    pub mod TSEL11 {
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

    /// TSEL10
    pub mod TSEL10 {
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

    /// DAC channel1 trigger enable
    pub mod TEN1 {
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

    /// DAC channel1 enable
    pub mod EN1 {
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

/// software trigger register
pub mod SWTRGR {

    /// DAC channel1 software trigger
    pub mod SWTRIG1 {
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

/// channel1 12-bit right-aligned data holding register
pub mod DHR12R1 {

    /// DAC channel1 12-bit right-aligned data
    pub mod DACC1DHR {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (12 bits: 0xfff << 0)
        pub const mask: u32 = 0xfff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// channel1 12-bit left aligned data holding register
pub mod DHR12L1 {

    /// DAC channel1 12-bit left-aligned data
    pub mod DACC1DHR {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (12 bits: 0xfff << 4)
        pub const mask: u32 = 0xfff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// channel1 8-bit right aligned data holding register
pub mod DHR8R1 {

    /// DAC channel1 8-bit right-aligned data
    pub mod DACC1DHR {
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

/// Dual DAC 12-bit right-aligned data holding register
pub mod DHR12RD {
    pub use super::DHR12R1::DACC1DHR;
}

/// Dual DAC 12-bit left aligned data holding register
pub mod DHR12LD {
    pub use super::DHR12L1::DACC1DHR;
}

/// Dual DAC 8-bit right aligned data holding register
pub mod DHR8RD {
    pub use super::DHR8R1::DACC1DHR;
}

/// DAC channel1 data output register
pub mod DOR1 {

    /// DACC1DOR
    pub mod DACC1DOR {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (12 bits: 0xfff << 0)
        pub const mask: u32 = 0xfff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// status register
pub mod SR {

    /// DAC Channel 1 busy writing sample time flag
    pub mod BWST1 {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DAC Channel 1 calibration offset status
    pub mod CAL_FLAG1 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DAC channel1 DMA underrun flag
    pub mod DMAUDR1 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// calibration control register
pub mod CCR {

    /// DAC Channel 1 offset trimming value
    pub mod OTRIM1 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (5 bits: 0b11111 << 0)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// mode control register
pub mod MCR {

    /// DAC Channel 1 mode
    pub mod MODE1 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Sample and Hold sample time register 1
pub mod SHSR1 {

    /// DAC Channel 1 sample Time (only valid in Sample and Hold mode)
    pub mod TSAMPLE1 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (10 bits: 0x3ff << 0)
        pub const mask: u32 = 0x3ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Sample and Hold hold time register
pub mod SHHR {

    /// DAC Channel 1 hold Time (only valid in Sample and Hold mode)
    pub mod THOLD1 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (10 bits: 0x3ff << 0)
        pub const mask: u32 = 0x3ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Sample and Hold refresh time register
pub mod SHRR {

    /// DAC Channel 1 refresh Time (only valid in Sample and Hold mode)
    pub mod TREFRESH1 {
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
#[repr(C)]
pub struct RegisterBlock {
    /// control register
    pub CR: RWRegister<u32>,

    /// software trigger register
    pub SWTRGR: WORegister<u32>,

    /// channel1 12-bit right-aligned data holding register
    pub DHR12R1: RWRegister<u32>,

    /// channel1 12-bit left aligned data holding register
    pub DHR12L1: RWRegister<u32>,

    /// channel1 8-bit right aligned data holding register
    pub DHR8R1: RWRegister<u32>,

    _reserved1: [u32; 3],

    /// Dual DAC 12-bit right-aligned data holding register
    pub DHR12RD: RWRegister<u32>,

    /// Dual DAC 12-bit left aligned data holding register
    pub DHR12LD: RWRegister<u32>,

    /// Dual DAC 8-bit right aligned data holding register
    pub DHR8RD: RWRegister<u32>,

    /// DAC channel1 data output register
    pub DOR1: RORegister<u32>,

    _reserved2: [u32; 1],

    /// status register
    pub SR: RWRegister<u32>,

    /// calibration control register
    pub CCR: RWRegister<u32>,

    /// mode control register
    pub MCR: RWRegister<u32>,

    /// Sample and Hold sample time register 1
    pub SHSR1: RWRegister<u32>,

    _reserved3: [u32; 1],

    /// Sample and Hold hold time register
    pub SHHR: RWRegister<u32>,

    /// Sample and Hold refresh time register
    pub SHRR: RWRegister<u32>,
}
pub struct ResetValues {
    pub CR: u32,
    pub SWTRGR: u32,
    pub DHR12R1: u32,
    pub DHR12L1: u32,
    pub DHR8R1: u32,
    pub DHR12RD: u32,
    pub DHR12LD: u32,
    pub DHR8RD: u32,
    pub DOR1: u32,
    pub SR: u32,
    pub CCR: u32,
    pub MCR: u32,
    pub SHSR1: u32,
    pub SHHR: u32,
    pub SHRR: u32,
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

/// Access functions for the DAC peripheral instance
pub mod DAC {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40007400,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in DAC
    pub const reset: ResetValues = ResetValues {
        CR: 0x00000000,
        SWTRGR: 0x00000000,
        DHR12R1: 0x00000000,
        DHR12L1: 0x00000000,
        DHR8R1: 0x00000000,
        DHR12RD: 0x00000000,
        DHR12LD: 0x00000000,
        DHR8RD: 0x00000000,
        DOR1: 0x00000000,
        SR: 0x00000000,
        CCR: 0x00000000,
        MCR: 0x00000000,
        SHSR1: 0x00000000,
        SHHR: 0x00010001,
        SHRR: 0x00010001,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut DAC_TAKEN: bool = false;

    /// Safe access to DAC
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
            if DAC_TAKEN {
                None
            } else {
                DAC_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to DAC
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if DAC_TAKEN && inst.addr == INSTANCE.addr {
                DAC_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal DAC
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        DAC_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to DAC
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const DAC: *const RegisterBlock = 0x40007400 as *const _;
