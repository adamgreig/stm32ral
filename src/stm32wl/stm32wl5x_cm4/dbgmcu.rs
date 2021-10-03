#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Microcontroller Debug Unit

use crate::{RORegister, RWRegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// DBGMCU Identity Code Register
pub mod IDCODER {

    /// Device ID
    pub mod DEV_ID {
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

    /// Revision
    pub mod REV_ID {
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

/// DBGMCU Configuration Register
pub mod CR {

    /// Allow debug in SLEEP mode
    pub mod DBG_SLEEP {
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

            /// 0b0: Debug Sleep Mode Disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Debug Sleep Mode Enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Allow debug in STOP mode
    pub mod DBG_STOP {
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

            /// 0b0: Debug Stop Mode Disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Debug Stop Mode Enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Allow debug in STANDBY mode
    pub mod DBG_STANDBY {
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

            /// 0b0: Debug Standby Mode Disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Debug Standby Mode Enabled
            pub const Enabled: u32 = 0b1;
        }
    }
}

/// DBGMCU CPU1 APB1 Peripheral Freeze Register 1
pub mod APB1FZR1 {

    /// TIM2 stop in CPU1 debug
    pub mod DBG_TIM2_STOP {
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

            /// 0b0: The counter clock of TIMx is fed even if the core is halted
            pub const Continue: u32 = 0b0;

            /// 0b1: The counter clock of TIMx is stopped when the core is halted
            pub const Stop: u32 = 0b1;
        }
    }

    /// RTC stop in CPU1 debug
    pub mod DBG_RTC_STOP {
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

            /// 0b0: The clock of the RTC counter is fed even if the core is halted
            pub const Continue: u32 = 0b0;

            /// 0b1: The clock of the RTC counter is stopped when the core is halted
            pub const Stop: u32 = 0b1;
        }
    }

    /// WWDG stop in CPU1 debug
    pub mod DBG_WWDG_STOP {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: The window watchdog counter clock continues even if the core is halted
            pub const Continue: u32 = 0b0;

            /// 0b1: The window watchdog counter clock is stopped when the core is halted
            pub const Stop: u32 = 0b1;
        }
    }

    /// IWDG stop in CPU1 debug
    pub mod DBG_IWDG_STOP {
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

            /// 0b0: The independent watchdog counter clock continues even if the core is halted
            pub const Continue: u32 = 0b0;

            /// 0b1: The independent watchdog counter clock is stopped when the core is halted
            pub const Stop: u32 = 0b1;
        }
    }

    /// I2C1 SMBUS timeout stop in CPU1 debug
    pub mod DBG_I2C1_STOP {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Same behavior as in normal mode
            pub const NormalMode: u32 = 0b0;

            /// 0b1: I2C3 SMBUS timeout is frozen
            pub const SMBusTimeoutFrozen: u32 = 0b1;
        }
    }

    /// I2C2 SMBUS timeout stop in CPU1 debug
    pub mod DBG_I2C2_STOP {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DBG_I2C1_STOP::RW;
    }

    /// I2C3 SMBUS timeout stop in CPU1 debug
    pub mod DBG_I2C3_STOP {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DBG_I2C1_STOP::RW;
    }

    /// LPTIM1 stop in CPU1 debug
    pub mod DBG_LPTIM1_STOP {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: LPTIM1 counter clock is fed even if the core is halted
            pub const Continue: u32 = 0b0;

            /// 0b1: LPTIM1 counter clock is stopped when the core is halted
            pub const Stop: u32 = 0b1;
        }
    }
}

/// DBGMCU CPU2 APB1 Peripheral Freeze Register 1 \[dual core device
pub mod C2APB1FZR1 {

    /// DBG_TIM2_STOP
    pub mod DBG_TIM2_STOP {
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

    /// DBG_RTC_STOP
    pub mod DBG_RTC_STOP {
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

    /// DBG_IWDG_STOP
    pub mod DBG_IWDG_STOP {
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

    /// DBG_I2C1_STOP
    pub mod DBG_I2C1_STOP {
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

    /// DBG_I2C2_STOP
    pub mod DBG_I2C2_STOP {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DBG_I2C3_STOP
    pub mod DBG_I2C3_STOP {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DBG_LPTIM1_STOP
    pub mod DBG_LPTIM1_STOP {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DBGMCU CPU1 APB1 Peripheral Freeze Register 2
pub mod APB1FZR2 {

    /// DBG_LPTIM2_STOP
    pub mod DBG_LPTIM2_STOP {
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

            /// 0b0: LPTIM1 counter clock is fed even if the core is halted
            pub const Continue: u32 = 0b0;

            /// 0b1: LPTIM1 counter clock is stopped when the core is halted
            pub const Stop: u32 = 0b1;
        }
    }

    /// DBG_LPTIM3_STOP
    pub mod DBG_LPTIM3_STOP {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DBG_LPTIM2_STOP::RW;
    }
}

/// DBGMCU CPU2 APB1 Peripheral Freeze Register 2 \[dual core device
pub mod C2APB1FZR2 {

    /// DBG_LPTIM2_STOP
    pub mod DBG_LPTIM2_STOP {
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

    /// DBG_LPTIM3_STOP
    pub mod DBG_LPTIM3_STOP {
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
}

/// DBGMCU CPU1 APB2 Peripheral Freeze Register
pub mod APB2FZR {

    /// DBG_TIM1_STOP
    pub mod DBG_TIM1_STOP {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: The counter clock of TIMx is fed even if the core is halted
            pub const Continue: u32 = 0b0;

            /// 0b1: The counter clock of TIMx is stopped when the core is halted
            pub const Stop: u32 = 0b1;
        }
    }

    /// DBG_TIM16_STOP
    pub mod DBG_TIM16_STOP {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DBG_TIM1_STOP::RW;
    }

    /// DBG_TIM17_STOP
    pub mod DBG_TIM17_STOP {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::DBG_TIM1_STOP::RW;
    }
}

/// DBGMCU CPU2 APB2 Peripheral Freeze Register \[dual core device
pub mod C2APB2FZR {

    /// DBG_TIM1_STOP
    pub mod DBG_TIM1_STOP {
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

    /// DBG_TIM16_STOP
    pub mod DBG_TIM16_STOP {
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

    /// DBG_TIM17_STOP
    pub mod DBG_TIM17_STOP {
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
}
#[repr(C)]
pub struct RegisterBlock {
    /// DBGMCU Identity Code Register
    pub IDCODER: RORegister<u32>,

    /// DBGMCU Configuration Register
    pub CR: RWRegister<u32>,

    _reserved1: [u32; 13],

    /// DBGMCU CPU1 APB1 Peripheral Freeze Register 1
    pub APB1FZR1: RWRegister<u32>,

    /// DBGMCU CPU2 APB1 Peripheral Freeze Register 1 \[dual core device
    pub C2APB1FZR1: RWRegister<u32>,

    /// DBGMCU CPU1 APB1 Peripheral Freeze Register 2
    pub APB1FZR2: RWRegister<u32>,

    /// DBGMCU CPU2 APB1 Peripheral Freeze Register 2 \[dual core device
    pub C2APB1FZR2: RWRegister<u32>,

    /// DBGMCU CPU1 APB2 Peripheral Freeze Register
    pub APB2FZR: RWRegister<u32>,

    /// DBGMCU CPU2 APB2 Peripheral Freeze Register \[dual core device
    pub C2APB2FZR: RWRegister<u32>,
}
pub struct ResetValues {
    pub IDCODER: u32,
    pub CR: u32,
    pub APB1FZR1: u32,
    pub C2APB1FZR1: u32,
    pub APB1FZR2: u32,
    pub C2APB1FZR2: u32,
    pub APB2FZR: u32,
    pub C2APB2FZR: u32,
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

/// Access functions for the DBGMCU peripheral instance
pub mod DBGMCU {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0xe0042000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in DBGMCU
    pub const reset: ResetValues = ResetValues {
        IDCODER: 0x10006497,
        CR: 0x00000000,
        APB1FZR1: 0x00000000,
        C2APB1FZR1: 0x00000000,
        APB1FZR2: 0x00000000,
        C2APB1FZR2: 0x00000000,
        APB2FZR: 0x00000000,
        C2APB2FZR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut DBGMCU_TAKEN: bool = false;

    /// Safe access to DBGMCU
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
            if DBGMCU_TAKEN {
                None
            } else {
                DBGMCU_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to DBGMCU
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if DBGMCU_TAKEN && inst.addr == INSTANCE.addr {
                DBGMCU_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal DBGMCU
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        DBGMCU_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to DBGMCU
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const DBGMCU: *const RegisterBlock = 0xe0042000 as *const _;
