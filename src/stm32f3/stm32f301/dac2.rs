#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Digital-to-analog converter

use crate::{RORegister, RWRegister, WORegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// control register
pub mod CR {

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
        /// Read-write values
        pub mod RW {

            /// 0b0: DAC channel X DMA Underrun Interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: DAC channel X DMA Underrun Interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
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
        /// Read-write values
        pub mod RW {

            /// 0b0: DAC channel X DMA mode disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: DAC channel X DMA mode enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// DAC channel1 mask/amplitude selector
    pub mod MAMP13 {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// MAMP12
    pub mod MAMP12 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// MAMP11
    pub mod MAMP11 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// MAMP10
    pub mod MAMP10 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DAC channel1 noise/triangle wave generation enable
    pub mod WAVE1 {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// WAVE2
    pub mod WAVE2 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DAC channel1 trigger selection
    pub mod TSEL1 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (3 bits: 0b111 << 3)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Timer 6 TRGO event
            pub const TIM6_TRGO: u32 = 0b000;

            /// 0b001: Timer 3 TRGO event
            pub const TIM3_TRGO: u32 = 0b001;

            /// 0b010: Timer 7 TRGO event
            pub const TIM7_TRGO: u32 = 0b010;

            /// 0b011: Timer 15 TRGO event
            pub const TIM15_TRGO: u32 = 0b011;

            /// 0b100: Timer 2 TRGO event
            pub const TIM2_TRGO: u32 = 0b100;

            /// 0b110: EXTI line9
            pub const EXTI9: u32 = 0b110;

            /// 0b111: Software trigger
            pub const SOFTWARE: u32 = 0b111;
        }
    }

    /// DAC channel1 trigger enable
    pub mod TEN1 {
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

            /// 0b0: DAC channel X trigger disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: DAC channel X trigger enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// DAC channel1 output buffer disable
    pub mod BOFF1 {
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

            /// 0b0: DAC channel X output buffer enabled
            pub const Enabled: u32 = 0b0;

            /// 0b1: DAC channel X output buffer disabled
            pub const Disabled: u32 = 0b1;
        }
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
        /// Read-write values
        pub mod RW {

            /// 0b0: DAC channel X disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: DAC channel X enabled
            pub const Enabled: u32 = 0b1;
        }
    }
}

/// software trigger register
pub mod SWTRIGR {

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
        /// Read-write values
        pub mod RW {

            /// 0b0: DAC channel X software trigger disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: DAC channel X software trigger enabled
            pub const Enabled: u32 = 0b1;
        }
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

/// DAC channel1 12-bit left aligned data holding register
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

/// DAC channel1 8-bit right aligned data holding register
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

/// DAC channel1 data output register
pub mod DOR1 {

    /// DAC channel1 data output
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

/// DAC status register
pub mod SR {

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
        /// Read-write values
        pub mod RW {

            /// 0b0: No DMA underrun error condition occurred for DAC channel X
            pub const NoUnderrun: u32 = 0b0;

            /// 0b1: DMA underrun error condition occurred for DAC channel X
            pub const Underrun: u32 = 0b1;
        }
    }
}
pub struct RegisterBlock {
    /// control register
    pub CR: RWRegister<u32>,

    /// software trigger register
    pub SWTRIGR: WORegister<u32>,

    /// channel1 12-bit right-aligned data holding register
    pub DHR12R1: RWRegister<u32>,

    /// DAC channel1 12-bit left aligned data holding register
    pub DHR12L1: RWRegister<u32>,

    /// DAC channel1 8-bit right aligned data holding register
    pub DHR8R1: RWRegister<u32>,

    _reserved1: [u32; 6],

    /// DAC channel1 data output register
    pub DOR1: RORegister<u32>,

    _reserved2: [u32; 1],

    /// DAC status register
    pub SR: RWRegister<u32>,
}
pub struct ResetValues {
    pub CR: u32,
    pub SWTRIGR: u32,
    pub DHR12R1: u32,
    pub DHR12L1: u32,
    pub DHR8R1: u32,
    pub DOR1: u32,
    pub SR: u32,
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
#[cfg(feature = "rtfm")]
unsafe impl Send for Instance {}

/// Access functions for the DAC2 peripheral instance
pub mod DAC2 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40009800,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in DAC2
    pub const reset: ResetValues = ResetValues {
        CR: 0x00000000,
        SWTRIGR: 0x00000000,
        DHR12R1: 0x00000000,
        DHR12L1: 0x00000000,
        DHR8R1: 0x00000000,
        DOR1: 0x00000000,
        SR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut DAC2_TAKEN: bool = false;

    /// Safe access to DAC2
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
            if DAC2_TAKEN {
                None
            } else {
                DAC2_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to DAC2
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if DAC2_TAKEN && inst.addr == INSTANCE.addr {
                DAC2_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal DAC2
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        DAC2_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to DAC2
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const DAC2: *const RegisterBlock = 0x40009800 as *const _;
