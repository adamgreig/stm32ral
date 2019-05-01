#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Flash

#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;
use {RORegister, RWRegister, WORegister};

/// Access control register
pub mod ACR {

    /// Latency
    pub mod LATENCY {
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

            /// 0b0: Zero wait state is used to read a word in the NVM
            pub const WS0: u32 = 0b0;

            /// 0b1: One wait state is used to read a word in the NVM
            pub const WS1: u32 = 0b1;
        }
    }

    /// Prefetch enable
    pub mod PRFTEN {
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

            /// 0b0: Prefetch is disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Prefetch is enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Flash mode during Sleep
    pub mod SLEEP_PD {
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

            /// 0b0: When the device is in Sleep mode, the NVM is in Idle mode
            pub const NVMIdleMode: u32 = 0b0;

            /// 0b1: When the device is in Sleep mode, the NVM is in power-down mode
            pub const NVMPwrDownMode: u32 = 0b1;
        }
    }

    /// Flash mode during Run
    pub mod RUN_PD {
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

            /// 0b0: When the device is in Run mode, the NVM is in Idle mode
            pub const NVMIdleMode: u32 = 0b0;

            /// 0b1: When the device is in Run mode, the NVM is in power-down mode
            pub const NVMPwrDownMode: u32 = 0b1;
        }
    }

    /// Disable Buffer
    pub mod DISAB_BUF {
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

            /// 0b0: The buffers are enabled
            pub const Enabled: u32 = 0b0;

            /// 0b1: The buffers are disabled
            pub const Disabled: u32 = 0b1;
        }
    }

    /// Pre-read data address
    pub mod PRE_READ {
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

            /// 0b0: The pre-read is disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: The pre-read is enabled
            pub const Enabled: u32 = 0b1;
        }
    }
}

/// Program/erase control register
pub mod PECR {

    /// FLASH_PECR and data EEPROM lock
    pub mod PELOCK {
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

            /// 0b0: The FLASH_PECR register is unlocked
            pub const Unlocked: u32 = 0b0;

            /// 0b1: The FLASH_PECR register is locked and no write/erase operation can start
            pub const Locked: u32 = 0b1;
        }
    }

    /// Program memory lock
    pub mod PRGLOCK {
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

            /// 0b0: The write and erase operations in the Flash program memory are disabled
            pub const Unlocked: u32 = 0b0;

            /// 0b1: The write and erase operations in the Flash program memory are enabled
            pub const Locked: u32 = 0b1;
        }
    }

    /// Option bytes block lock
    pub mod OPTLOCK {
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

            /// 0b0: The write and erase operations in the Option bytes area are disabled
            pub const Unlocked: u32 = 0b0;

            /// 0b1: The write and erase operations in the Option bytes area are enabled
            pub const Locked: u32 = 0b1;
        }
    }

    /// Program memory selection
    pub mod PROG {
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

            /// 0b0: The Flash program memory is not selected
            pub const NotSelected: u32 = 0b0;

            /// 0b1: The Flash program memory is selected
            pub const Selected: u32 = 0b1;
        }
    }

    /// Data EEPROM selection
    pub mod DATA {
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

            /// 0b0: Data EEPROM not selected
            pub const NotSelected: u32 = 0b0;

            /// 0b1: Data memory selected
            pub const Selected: u32 = 0b1;
        }
    }

    /// Fixed time data write for Byte, Half Word and Word programming
    pub mod FIX {
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

            /// 0b0: An erase phase is automatically performed
            pub const AutoErase: u32 = 0b0;

            /// 0b1: The program operation is always performed with a preliminary erase
            pub const PrelimErase: u32 = 0b1;
        }
    }

    /// Page or Double Word erase mode
    pub mod ERASE {
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

            /// 0b0: No erase operation requested
            pub const NoErase: u32 = 0b0;

            /// 0b1: Erase operation requested
            pub const Erase: u32 = 0b1;
        }
    }

    /// Half Page/Double Word programming mode
    pub mod FPRG {
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

            /// 0b0: Half Page programming disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Half Page programming enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Parallel bank mode
    pub mod PARALLELBANK {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Parallel bank mode disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Parallel bank mode enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// End of programming interrupt enable
    pub mod EOPIE {
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

            /// 0b0: End of program interrupt disable
            pub const Disabled: u32 = 0b0;

            /// 0b1: End of program interrupt enable
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Error interrupt enable
    pub mod ERRIE {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Error interrupt disable
            pub const Disabled: u32 = 0b0;

            /// 0b1: Error interrupt enable
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Launch the option byte loading
    pub mod OBL_LAUNCH {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Option byte loaded
            pub const Complete: u32 = 0b0;

            /// 0b1: Option byte loading to be done
            pub const NotComplete: u32 = 0b1;
        }
        /// Write-only values
        pub mod W {

            /// 0b1: Reload option byte
            pub const Reload: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Power down key register
pub mod PDKEYR {

    /// RUN_PD in FLASH_ACR key
    pub mod PDKEYR {
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

/// Program/erase key register
pub mod PEKEYR {

    /// FLASH_PEC and data EEPROM key
    pub mod PEKEYR {
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

/// Program memory key register
pub mod PRGKEYR {

    /// Program memory key
    pub mod PRGKEYR {
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

/// Option byte key register
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

/// Status register
pub mod SR {

    /// Write/erase operations in progress
    pub mod BSY {
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

            /// 0b0: No write/erase operation is in progress
            pub const Inactive: u32 = 0b0;

            /// 0b1: No write/erase operation is in progress
            pub const Active: u32 = 0b1;
        }
    }

    /// End of operation
    pub mod EOP {
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

            /// 0b0: No EOP operation occurred
            pub const NoEvent: u32 = 0b0;

            /// 0b1: An EOP event occurred
            pub const Event: u32 = 0b1;
        }
    }

    /// End of high voltage
    pub mod ENDHV {
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

            /// 0b0: High voltage is executing a write/erase operation in the NVM
            pub const Active: u32 = 0b0;

            /// 0b1: High voltage is off, no write/erase operation is ongoing
            pub const Inactive: u32 = 0b1;
        }
    }

    /// Flash memory module ready after low power mode
    pub mod READY {
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

            /// 0b0: The NVM is not ready
            pub const NotReady: u32 = 0b0;

            /// 0b1: The NVM is ready
            pub const Ready: u32 = 0b1;
        }
    }

    /// Write protected error
    pub mod WRPERR {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No protection error happened
            pub const NoError: u32 = 0b0;

            /// 0b1: One protection error happened
            pub const Error: u32 = 0b1;
        }
        /// Write-only values
        pub mod W {

            /// 0b1: Clear the flag
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Programming alignment error
    pub mod PGAERR {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No alignment error happened
            pub const NoError: u32 = 0b0;

            /// 0b1: One alignment error happened
            pub const Error: u32 = 0b1;
        }
        pub use super::WRPERR::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Size error
    pub mod SIZERR {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No size error happened
            pub const NoError: u32 = 0b0;

            /// 0b1: One size error happened
            pub const Error: u32 = 0b1;
        }
        pub use super::WRPERR::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Option validity error
    pub mod OPTVERR {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No error happened during the Option bytes loading
            pub const NoError: u32 = 0b0;

            /// 0b1: One or more errors happened during the Option bytes loading
            pub const Error: u32 = 0b1;
        }
        pub use super::WRPERR::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// RDERR
    pub mod RDERR {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No read protection error happened.
            pub const NoError: u32 = 0b0;

            /// 0b1: One read protection error happened
            pub const Error: u32 = 0b1;
        }
        pub use super::WRPERR::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// NOTZEROERR
    pub mod NOTZEROERR {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: The write operation is done in an erased region or the memory interface can apply an erase before a write
            pub const NoEvent: u32 = 0b0;

            /// 0b1: The write operation is attempting to write to a not-erased region and the memory interface cannot apply an erase before a write
            pub const Event: u32 = 0b1;
        }
        pub use super::WRPERR::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// FWWERR
    pub mod FWWERR {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No write/erase operation aborted to perform a fetch
            pub const NoError: u32 = 0b0;

            /// 0b1: A write/erase operation aborted to perform a fetch
            pub const Error: u32 = 0b1;
        }
        pub use super::WRPERR::W;
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Option byte register
pub mod OPTR {

    /// Read protection
    pub mod RDPROT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (8 bits: 0xff << 0)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b10101010: Level 0
            pub const Level0: u32 = 0b10101010;

            /// 0b00000000: Level 1
            pub const Level1: u32 = 0b00000000;

            /// 0b11001100: Level 2
            pub const Level2: u32 = 0b11001100;
        }
    }

    /// BOR_LEV
    pub mod BOR_LEV {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (4 bits: 0b1111 << 16)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: This is the reset threshold level for the 1.45 V - 1.55 V voltage range (power-down only)
            pub const BOR_Off: u32 = 0b0000;

            /// 0b0001: Reset threshold level for VBOR0 (around 1.8 V)
            pub const BOR_Level1: u32 = 0b0001;

            /// 0b0010: Reset threshold level for VBOR1 (around 2.0 V)
            pub const BOR_Level2: u32 = 0b0010;

            /// 0b0011: Reset threshold level for VBOR2 (around 2.5 V)
            pub const BOR_Level3: u32 = 0b0011;

            /// 0b0100: Reset threshold level for VBOR3 (around 2.7 V)
            pub const BOR_Level4: u32 = 0b0100;

            /// 0b0101: Reset threshold level for VBOR4 (around 3.0 V)
            pub const BOR_Level5: u32 = 0b0101;
        }
    }

    /// Selection of protection mode of WPR bits
    pub mod WPRMOD {
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

            /// 0b0: PCROP disabled. The WRPROT bits are used as a write protection on a sector.
            pub const Disabled: u32 = 0b0;

            /// 0b1: PCROP enabled. The WRPROT bits are used as a read protection on a sector.
            pub const Enabled: u32 = 0b1;
        }
    }
}

/// Write Protection Register 1
pub mod WRPROT1 {

    /// Write Protection
    pub mod WRPROT1 {
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

/// Write Protection Register 2
pub mod WRPROT2 {

    /// Write Protection
    pub mod WRPROT2 {
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
pub struct RegisterBlock {
    /// Access control register
    pub ACR: RWRegister<u32>,

    /// Program/erase control register
    pub PECR: RWRegister<u32>,

    /// Power down key register
    pub PDKEYR: WORegister<u32>,

    /// Program/erase key register
    pub PEKEYR: WORegister<u32>,

    /// Program memory key register
    pub PRGKEYR: WORegister<u32>,

    /// Option byte key register
    pub OPTKEYR: WORegister<u32>,

    /// Status register
    pub SR: RWRegister<u32>,

    /// Option byte register
    pub OPTR: RORegister<u32>,

    /// Write Protection Register 1
    pub WRPROT1: RORegister<u32>,

    _reserved1: [u32; 23],

    /// Write Protection Register 2
    pub WRPROT2: RORegister<u32>,
}
pub struct ResetValues {
    pub ACR: u32,
    pub PECR: u32,
    pub PDKEYR: u32,
    pub PEKEYR: u32,
    pub PRGKEYR: u32,
    pub OPTKEYR: u32,
    pub SR: u32,
    pub OPTR: u32,
    pub WRPROT1: u32,
    pub WRPROT2: u32,
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

/// Access functions for the FLASH peripheral instance
pub mod FLASH {
    #[cfg(not(feature = "nosync"))]
    use external_cortex_m;

    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40022000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in FLASH
    pub const reset: ResetValues = ResetValues {
        ACR: 0x00000000,
        PECR: 0x00000007,
        PDKEYR: 0x00000000,
        PEKEYR: 0x00000000,
        PRGKEYR: 0x00000000,
        OPTKEYR: 0x00000000,
        SR: 0x00000004,
        OPTR: 0x00F80000,
        WRPROT1: 0x00000000,
        WRPROT2: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut FLASH_TAKEN: bool = false;

    /// Safe access to FLASH
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
            if FLASH_TAKEN {
                None
            } else {
                FLASH_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to FLASH
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if FLASH_TAKEN && inst.addr == INSTANCE.addr {
                FLASH_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }
}

/// Raw pointer to FLASH
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const FLASH: *const RegisterBlock = 0x40022000 as *const _;
