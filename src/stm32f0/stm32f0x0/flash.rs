#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Flash

use crate::{RORegister, RWRegister, WORegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// Flash access control register
pub mod ACR {

    /// LATENCY
    pub mod LATENCY {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: 0 wait states
            pub const WS0: u32 = 0b000;

            /// 0b001: 1 wait state
            pub const WS1: u32 = 0b001;
        }
    }

    /// PRFTBE
    pub mod PRFTBE {
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

            /// 0b0: Prefetch is disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Prefetch is enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// PRFTBS
    pub mod PRFTBS {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Prefetch buffer is disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Prefetch buffer is enabled
            pub const Enabled: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Flash key register
pub mod KEYR {

    /// Flash Key
    pub mod FKEYR {
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

/// Flash option key register
pub mod OPTKEYR {

    /// Option byte key
    pub mod OPTKEYR {
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

/// Flash status register
pub mod SR {

    /// End of operation
    pub mod EOP {
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

            /// 0b0: No EOP operation occurred
            pub const NoEvent: u32 = 0b0;

            /// 0b1: An EOP event occurred
            pub const Event: u32 = 0b1;
        }
    }

    /// Write protection error
    pub mod WRPRT {
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

            /// 0b0: No write protection error occurred
            pub const NoError: u32 = 0b0;

            /// 0b1: A write protection error occurred
            pub const Error: u32 = 0b1;
        }
    }

    /// Programming error
    pub mod PGERR {
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

            /// 0b0: No programming error occurred
            pub const NoError: u32 = 0b0;

            /// 0b1: A programming error occurred
            pub const Error: u32 = 0b1;
        }
    }

    /// Busy
    pub mod BSY {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No write/erase operation is in progress
            pub const Inactive: u32 = 0b0;

            /// 0b1: No write/erase operation is in progress
            pub const Active: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Flash control register
pub mod CR {

    /// Force option byte loading
    pub mod FORCE_OPTLOAD {
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

            /// 0b0: Force option byte loading inactive
            pub const Inactive: u32 = 0b0;

            /// 0b1: Force option byte loading active
            pub const Active: u32 = 0b1;
        }
    }

    /// End of operation interrupt enable
    pub mod EOPIE {
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

            /// 0b0: End of operation interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: End of operation interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Error interrupt enable
    pub mod ERRIE {
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

            /// 0b0: Error interrupt generation disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Error interrupt generation enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Option bytes write enable
    pub mod OPTWRE {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Option byte write enabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Option byte write disabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Lock
    pub mod LOCK {
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

            /// 0b0: FLASH_CR register is unlocked
            pub const Unlocked: u32 = 0b0;

            /// 0b1: FLASH_CR register is locked
            pub const Locked: u32 = 0b1;
        }
    }

    /// Start
    pub mod STRT {
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

            /// 0b1: Trigger an erase operation
            pub const Start: u32 = 0b1;
        }
    }

    /// Option byte erase
    pub mod OPTER {
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

            /// 0b1: Erase option byte activated
            pub const OptionByteErase: u32 = 0b1;
        }
    }

    /// Option byte programming
    pub mod OPTPG {
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

            /// 0b1: Program option byte activated
            pub const OptionByteProgramming: u32 = 0b1;
        }
    }

    /// Mass erase
    pub mod MER {
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

            /// 0b1: Erase activated for all user sectors
            pub const MassErase: u32 = 0b1;
        }
    }

    /// Page erase
    pub mod PER {
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

            /// 0b1: Erase activated for selected page
            pub const PageErase: u32 = 0b1;
        }
    }

    /// Programming
    pub mod PG {
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

            /// 0b1: Flash programming activated
            pub const Program: u32 = 0b1;
        }
    }
}

/// Flash address register
pub mod AR {

    /// Flash address
    pub mod FAR {
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

/// Option byte register
pub mod OBR {

    /// Option byte error
    pub mod OPTERR {
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

            /// 0b1: The loaded option byte and its complement do not match
            pub const OptionByteError: u32 = 0b1;
        }
    }

    /// Read protection level status
    pub mod RDPRT {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (2 bits: 0b11 << 1)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Level 0
            pub const Level0: u32 = 0b00;

            /// 0b01: Level 1
            pub const Level1: u32 = 0b01;

            /// 0b11: Level 2
            pub const Level2: u32 = 0b11;
        }
    }

    /// WDG_SW
    pub mod WDG_SW {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Hardware watchdog
            pub const Hardware: u32 = 0b0;

            /// 0b1: Software watchdog
            pub const Software: u32 = 0b1;
        }
    }

    /// nRST_STOP
    pub mod nRST_STOP {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Reset generated when entering Stop mode
            pub const Reset: u32 = 0b0;

            /// 0b1: No reset generated
            pub const NoReset: u32 = 0b1;
        }
    }

    /// nRST_STDBY
    pub mod nRST_STDBY {
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

            /// 0b0: Reset generated when entering Standby mode
            pub const Reset: u32 = 0b0;

            /// 0b1: No reset generated
            pub const NoReset: u32 = 0b1;
        }
    }

    /// BOOT1
    pub mod nBOOT1 {
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

            /// 0b0: Together with BOOT0, select the device boot mode
            pub const Disabled: u32 = 0b0;

            /// 0b1: Together with BOOT0, select the device boot mode
            pub const Enabled: u32 = 0b1;
        }
    }

    /// VDDA_MONITOR
    pub mod VDDA_MONITOR {
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

            /// 0b0: VDDA power supply supervisor disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: VDDA power supply supervisor enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// RAM_PARITY_CHECK
    pub mod RAM_PARITY_CHECK {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b1: RAM parity check disabled
            pub const Disabled: u32 = 0b1;

            /// 0b0: RAM parity check enabled
            pub const Enabled: u32 = 0b0;
        }
    }

    /// Data0
    pub mod Data0 {
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

    /// Data1
    pub mod Data1 {
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

/// Write protection register
pub mod WRPR {

    /// Write protect
    pub mod WRP {
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
    /// Flash access control register
    pub ACR: RWRegister<u32>,

    /// Flash key register
    pub KEYR: WORegister<u32>,

    /// Flash option key register
    pub OPTKEYR: WORegister<u32>,

    /// Flash status register
    pub SR: RWRegister<u32>,

    /// Flash control register
    pub CR: RWRegister<u32>,

    /// Flash address register
    pub AR: WORegister<u32>,

    _reserved1: [u32; 1],

    /// Option byte register
    pub OBR: RORegister<u32>,

    /// Write protection register
    pub WRPR: RORegister<u32>,
}
pub struct ResetValues {
    pub ACR: u32,
    pub KEYR: u32,
    pub OPTKEYR: u32,
    pub SR: u32,
    pub CR: u32,
    pub AR: u32,
    pub OBR: u32,
    pub WRPR: u32,
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

/// Access functions for the Flash peripheral instance
pub mod Flash {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40022000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in Flash
    pub const reset: ResetValues = ResetValues {
        ACR: 0x00000030,
        KEYR: 0x00000000,
        OPTKEYR: 0x00000000,
        SR: 0x00000000,
        CR: 0x00000080,
        AR: 0x00000000,
        OBR: 0x03FFFFF2,
        WRPR: 0xFFFFFFFF,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut Flash_TAKEN: bool = false;

    /// Safe access to Flash
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
            if Flash_TAKEN {
                None
            } else {
                Flash_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to Flash
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if Flash_TAKEN && inst.addr == INSTANCE.addr {
                Flash_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal Flash
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        Flash_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to Flash
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const Flash: *const RegisterBlock = 0x40022000 as *const _;
