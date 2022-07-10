#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Analog-to-Digital Converter

use crate::{RORegister, RWRegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// ADC Common status register
pub mod CSR {

    /// ADDRDY_MST
    pub mod ADDRDY_MST {
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

    /// EOSMP_MST
    pub mod EOSMP_MST {
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

            /// 0b0: End of sampling phase no yet reached
            pub const NotEnded: u32 = 0b0;

            /// 0b1: End of sampling phase reached
            pub const Ended: u32 = 0b1;
        }
    }

    /// EOC_MST
    pub mod EOC_MST {
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

            /// 0b0: Regular conversion is not complete
            pub const NotComplete: u32 = 0b0;

            /// 0b1: Regular conversion complete
            pub const Complete: u32 = 0b1;
        }
    }

    /// EOS_MST
    pub mod EOS_MST {
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

            /// 0b0: Regular sequence is not complete
            pub const NotComplete: u32 = 0b0;

            /// 0b1: Regular sequence complete
            pub const Complete: u32 = 0b1;
        }
    }

    /// OVR_MST
    pub mod OVR_MST {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No overrun occurred
            pub const NoOverrun: u32 = 0b0;

            /// 0b1: Overrun occurred
            pub const Overrun: u32 = 0b1;
        }
    }

    /// JEOC_MST
    pub mod JEOC_MST {
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

            /// 0b0: Injected conversion is not complete
            pub const NotComplete: u32 = 0b0;

            /// 0b1: Injected conversion complete
            pub const Complete: u32 = 0b1;
        }
    }

    /// JEOS_MST
    pub mod JEOS_MST {
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

            /// 0b0: Injected sequence is not complete
            pub const NotComplete: u32 = 0b0;

            /// 0b1: Injected sequence complete
            pub const Complete: u32 = 0b1;
        }
    }

    /// AWD1_MST
    pub mod AWD1_MST {
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

            /// 0b0: No analog watchdog event occurred
            pub const NoEvent: u32 = 0b0;

            /// 0b1: Analog watchdog event occurred
            pub const Event: u32 = 0b1;
        }
    }

    /// AWD2_MST
    pub mod AWD2_MST {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AWD1_MST::RW;
    }

    /// AWD3_MST
    pub mod AWD3_MST {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AWD1_MST::RW;
    }

    /// JQOVF_MST
    pub mod JQOVF_MST {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No injected context queue overflow has occurred
            pub const NoOverflow: u32 = 0b0;

            /// 0b1: Injected context queue overflow has occurred
            pub const Overflow: u32 = 0b1;
        }
    }

    /// ADRDY_SLV
    pub mod ADRDY_SLV {
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

            /// 0b0: ADC is not ready to start conversion
            pub const NotReady: u32 = 0b0;

            /// 0b1: ADC is ready to start conversion
            pub const Ready: u32 = 0b1;
        }
    }

    /// EOSMP_SLV
    pub mod EOSMP_SLV {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EOSMP_MST::RW;
    }

    /// End of regular conversion of the slave ADC
    pub mod EOC_SLV {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EOC_MST::RW;
    }

    /// End of regular sequence flag of the slave ADC
    pub mod EOS_SLV {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::EOS_MST::RW;
    }

    /// Overrun flag of the slave ADC
    pub mod OVR_SLV {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OVR_MST::RW;
    }

    /// End of injected conversion flag of the slave ADC
    pub mod JEOC_SLV {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::JEOC_MST::RW;
    }

    /// End of injected sequence flag of the slave ADC
    pub mod JEOS_SLV {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::JEOS_MST::RW;
    }

    /// Analog watchdog 1 flag of the slave ADC
    pub mod AWD1_SLV {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AWD1_MST::RW;
    }

    /// Analog watchdog 2 flag of the slave ADC
    pub mod AWD2_SLV {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AWD1_MST::RW;
    }

    /// Analog watchdog 3 flag of the slave ADC
    pub mod AWD3_SLV {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AWD1_MST::RW;
    }

    /// Injected Context Queue Overflow flag of the slave ADC
    pub mod JQOVF_SLV {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::JQOVF_MST::RW;
    }
}

/// ADC common control register
pub mod CCR {

    /// Dual ADC mode selection
    pub mod DUAL {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (5 bits: 0b11111 << 0)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00000: Independent mode
            pub const Independent: u32 = 0b00000;

            /// 0b00001: Dual, combined regular simultaneous + injected simultaneous mode
            pub const DualRJ: u32 = 0b00001;

            /// 0b00010: Dual, combined regular simultaneous + alternate trigger mode
            pub const DualRA: u32 = 0b00010;

            /// 0b00011: Dual, combined interleaved mode + injected simultaneous mode
            pub const DualIJ: u32 = 0b00011;

            /// 0b00101: Dual, injected simultaneous mode only
            pub const DualJ: u32 = 0b00101;

            /// 0b00110: Dual, regular simultaneous mode only
            pub const DualR: u32 = 0b00110;

            /// 0b00111: Dual, interleaved mode only
            pub const DualI: u32 = 0b00111;

            /// 0b01001: Dual, alternate trigger mode only
            pub const DualA: u32 = 0b01001;
        }
    }

    /// Delay between 2 sampling phases
    pub mod DELAY {
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

    /// DMA configuration (for multi-ADC mode)
    pub mod DMACFG {
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

            /// 0b0: DMA one shot mode selected
            pub const OneShot: u32 = 0b0;

            /// 0b1: DMA circular mode selected
            pub const Circulator: u32 = 0b1;
        }
    }

    /// Direct memory access mode for multi ADC mode
    pub mod MDMA {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (2 bits: 0b11 << 14)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: MDMA mode disabled
            pub const Disabled: u32 = 0b00;

            /// 0b10: MDMA mode enabled for 12 and 10-bit resolution
            pub const Bits12_10: u32 = 0b10;

            /// 0b11: MDMA mode enabled for 8 and 6-bit resolution
            pub const Bits8_6: u32 = 0b11;
        }
    }

    /// ADC clock mode
    pub mod CKMODE {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (2 bits: 0b11 << 16)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Use Kernel Clock adc_ker_ck_input divided by PRESC. Asynchronous to AHB clock
            pub const Asynchronous: u32 = 0b00;

            /// 0b01: Use AHB clock rcc_hclk3. In this case rcc_hclk must equal sys_d1cpre_ck
            pub const SyncDiv1: u32 = 0b01;

            /// 0b10: Use AHB clock rcc_hclk3 divided by 2
            pub const SyncDiv2: u32 = 0b10;

            /// 0b11: Use AHB clock rcc_hclk3 divided by 4
            pub const SyncDiv4: u32 = 0b11;
        }
    }

    /// VREFINT enable
    pub mod VREFEN {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: V_REFINT channel disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: V_REFINT channel enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Temperature sensor enable
    pub mod TSEN {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Temperature sensor channel disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Temperature sensor channel enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// VBAT enable
    pub mod VBATEN {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: V_BAT channel disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: V_BAT channel enabled
            pub const Enabled: u32 = 0b1;
        }
    }
}

/// ADC common regular data register for dual and triple modes
pub mod CDR {

    /// Regular data of the slave ADC
    pub mod RDATA_SLV {
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

    /// Regular data of the master ADC
    pub mod RDATA_MST {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u32 = 0xffff << offset;
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
    /// ADC Common status register
    pub CSR: RORegister<u32>,

    _reserved1: [u8; 4],

    /// ADC common control register
    pub CCR: RWRegister<u32>,

    /// ADC common regular data register for dual and triple modes
    pub CDR: RORegister<u32>,
}
pub struct ResetValues {
    pub CSR: u32,
    pub CCR: u32,
    pub CDR: u32,
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

/// Access functions for the ADC1_2 peripheral instance
pub mod ADC1_2 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x50000300,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in ADC1_2
    pub const reset: ResetValues = ResetValues {
        CSR: 0x00000000,
        CCR: 0x00000000,
        CDR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut ADC1_2_TAKEN: bool = false;

    /// Safe access to ADC1_2
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
            if ADC1_2_TAKEN {
                None
            } else {
                ADC1_2_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to ADC1_2
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if ADC1_2_TAKEN && inst.addr == INSTANCE.addr {
                ADC1_2_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal ADC1_2
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        ADC1_2_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to ADC1_2
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const ADC1_2: *const RegisterBlock = 0x50000300 as *const _;

/// Access functions for the ADC3_4 peripheral instance
pub mod ADC3_4 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x50000700,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in ADC3_4
    pub const reset: ResetValues = ResetValues {
        CSR: 0x00000000,
        CCR: 0x00000000,
        CDR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut ADC3_4_TAKEN: bool = false;

    /// Safe access to ADC3_4
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
            if ADC3_4_TAKEN {
                None
            } else {
                ADC3_4_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to ADC3_4
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if ADC3_4_TAKEN && inst.addr == INSTANCE.addr {
                ADC3_4_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal ADC3_4
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        ADC3_4_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to ADC3_4
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const ADC3_4: *const RegisterBlock = 0x50000700 as *const _;
