#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Flash

use crate::{RWRegister, WORegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// Access control register
pub mod ACR {

    /// Latency
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

            /// 0b001: 1 wait states
            pub const WS1: u32 = 0b001;

            /// 0b010: 2 wait states
            pub const WS2: u32 = 0b010;
        }
    }

    /// Prefetch enable
    pub mod PRFTEN {
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

            /// 0b0: Prefetch is disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Prefetch is enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Instruction cache enable
    pub mod ICEN {
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

            /// 0b0: Instruction cache is disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Instruction cache is enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Data cache enable
    pub mod DCEN {
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

            /// 0b0: Data cache is disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Data cache is enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Instruction cache reset
    pub mod ICRST {
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

            /// 0b0: Instruction cache is not reset
            pub const NotReset: u32 = 0b0;

            /// 0b1: Instruction cache is reset
            pub const Reset: u32 = 0b1;
        }
    }

    /// Data cache reset
    pub mod DCRST {
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

            /// 0b0: Data cache is not reset
            pub const NotReset: u32 = 0b0;

            /// 0b1: Data cache is reset
            pub const Reset: u32 = 0b1;
        }
    }

    /// CPU1 programm erase suspend request
    pub mod PES {
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

            /// 0b0: Flash program and erase operations granted
            pub const Granted: u32 = 0b0;

            /// 0b1: Any new Flash program and erase operation is suspended until this bit is cleared. The PESD bit in FLASH_SR is set when PES bit in FLASH_ACR is set
            pub const Suspended: u32 = 0b1;
        }
    }

    /// Flash User area empty
    pub mod EMPTY {
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

            /// 0b0: User Flash programmend
            pub const Programmed: u32 = 0b0;

            /// 0b1: User Flash empty
            pub const Empty: u32 = 0b1;
        }
    }
}

/// Flash key register
pub mod KEYR {

    /// KEY
    pub mod KEY {
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
    pub mod OPTKEY {
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

    /// End of operation
    pub mod EOP {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No EOP operation occurred
            pub const NoEvent: u32 = 0b0;

            /// 0b1: An EOP event occurred
            pub const Event: u32 = 0b1;
        }
        /// Write-only values
        pub mod W {

            /// 0b1: Clear the flag
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Operation error
    pub mod OPERR {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No memory opreation error happened
            pub const NoError: u32 = 0b0;

            /// 0b1: Memory operation error happened
            pub const Error: u32 = 0b1;
        }
        pub use super::EOP::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Programming error
    pub mod PROGERR {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No size programming error happened
            pub const NoError: u32 = 0b0;

            /// 0b1: Programming error happened
            pub const Error: u32 = 0b1;
        }
        pub use super::EOP::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Write protected error
    pub mod WRPERR {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No write protection error happened
            pub const NoError: u32 = 0b0;

            /// 0b1: Write protection error happened
            pub const Error: u32 = 0b1;
        }
        pub use super::EOP::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Programming alignment error
    pub mod PGAERR {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No programming alignment error happened
            pub const NoError: u32 = 0b0;

            /// 0b1: Programming alignment error happened
            pub const Error: u32 = 0b1;
        }
        pub use super::EOP::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Size error
    pub mod SIZERR {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No size error happened
            pub const NoError: u32 = 0b0;

            /// 0b1: Size error happened
            pub const Error: u32 = 0b1;
        }
        pub use super::EOP::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Programming sequence error
    pub mod PGSERR {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No fast programming sequence error happened
            pub const NoError: u32 = 0b0;

            /// 0b1: Fast programming sequence error happened
            pub const Error: u32 = 0b1;
        }
        pub use super::EOP::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Fast programming data miss error
    pub mod MISSERR {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No fast programming data miss error happened
            pub const NoError: u32 = 0b0;

            /// 0b1: Fast programming data miss error happened
            pub const Error: u32 = 0b1;
        }
        pub use super::EOP::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Fast programming error
    pub mod FASTERR {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No fast programming error happened
            pub const NoError: u32 = 0b0;

            /// 0b1: Fast programming error happened
            pub const Error: u32 = 0b1;
        }
        pub use super::EOP::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// User Option OPTIVAL indication
    pub mod OPTNV {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: The OBL user option OPTVAL indicates "valid"
            pub const Valid: u32 = 0b0;

            /// 0b1: The OBL user option OPTVAL indicates "invalid"
            pub const Invalid: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PCROP read error
    pub mod RDERR {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No read-only error happened
            pub const NoError: u32 = 0b0;

            /// 0b1: Read-only error happened
            pub const Error: u32 = 0b1;
        }
        pub use super::EOP::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Option validity error
    pub mod OPTVERR {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No error in option and engineering bits
            pub const NoError: u32 = 0b0;

            /// 0b1: Error in option and engineering bits
            pub const Error: u32 = 0b1;
        }
        pub use super::EOP::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Busy
    pub mod BSY {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
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

    /// Programming or erase configuration busy
    pub mod CFGBSY {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: PG, PNB, PER, MER bits available for writing
            pub const Free: u32 = 0b0;

            /// 0b1: PG, PNB, PER, MER bits not available for writing (operation ongoing)
            pub const Busy: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Programming / erase operation suspended
    pub mod PESD {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Flash program and erase operations granted
            pub const Granted: u32 = 0b0;

            /// 0b1: Any new Flash program and erase operation is suspended until this bit is cleared. This bit is set when the PES bit in FLASH_ACR is set
            pub const Suspended: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Flash control register
pub mod CR {

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

            /// 0b0: Flash programming disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Flash programming enabled
            pub const Enabled: u32 = 0b1;
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

            /// 0b0: Page erase disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Page erase enabled
            pub const Enabled: u32 = 0b1;
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

            /// 0b0: No mass erase
            pub const NoErase: u32 = 0b0;

            /// 0b1: Trigger mass erase
            pub const MassErase: u32 = 0b1;
        }
    }

    /// Page number
    pub mod PNB {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (7 bits: 0x7f << 3)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Start
    pub mod STRT {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Options modification completed or idle
            pub const Done: u32 = 0b0;
        }
        /// Write-only values
        pub mod W {

            /// 0b1: Trigger options programming operation
            pub const Start: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Options modification start
    pub mod OPTSTRT {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        pub use super::STRT::R;
        pub use super::STRT::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Fast programming
    pub mod FSTPG {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Fast programming disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Fast programming enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// End of operation interrupt enable
    pub mod EOPIE {
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

            /// 0b0: End of program interrupt disable
            pub const Disabled: u32 = 0b0;

            /// 0b1: End of program interrupt enable
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Error interrupt enable
    pub mod ERRIE {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: OPERR Error interrupt disable
            pub const Disabled: u32 = 0b0;

            /// 0b1: OPERR Error interrupt enable
            pub const Enabled: u32 = 0b1;
        }
    }

    /// PCROP read error interrupt enable
    pub mod RDERRIE {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: PCROP read error interrupt disable
            pub const Disabled: u32 = 0b0;

            /// 0b1: PCROP read error interrupt enable
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Force the option byte loading
    pub mod OBL_LAUNCH {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
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

    /// Options Lock
    pub mod OPTLOCK {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: FLASH_CR options are unlocked
            pub const Unlocked: u32 = 0b0;
        }
        /// Write-only values
        pub mod W {

            /// 0b1: FLASH_CR options are locked
            pub const Locked: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// FLASH_CR Lock
    pub mod LOCK {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: FLASH_CR is unlocked
            pub const Unlocked: u32 = 0b0;
        }
        /// Write-only values
        pub mod W {

            /// 0b1: FLASH_CR is locked
            pub const Locked: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Flash ECC register
pub mod ECCR {

    /// ECC fail address
    pub mod ADDR_ECC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (17 bits: 0x1ffff << 0)
        pub const mask: u32 = 0x1ffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// System Flash ECC fail
    pub mod SYSF_ECC {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No System Flash memory ECC fail
            pub const NotInFlash: u32 = 0b0;

            /// 0b1: System Flash memory ECC fail
            pub const InFlash: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// ECC correction interrupt enable
    pub mod ECCCIE {
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

            /// 0b0: ECCC interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: ECCC interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// ECC correction
    pub mod ECCC {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: ECC error corrected
            pub const NoEvent: u32 = 0b0;

            /// 0b1: No ECC error corrected
            pub const Event: u32 = 0b1;
        }
        /// Write-only values
        pub mod W {

            /// 0b1: Clear the flag
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// ECC detection
    pub mod ECCD {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Two ECC errors detected
            pub const NoEvent: u32 = 0b0;

            /// 0b1: No two ECC errors detected
            pub const Event: u32 = 0b1;
        }
        pub use super::ECCC::W;
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Flash option register
pub mod OPTR {

    /// Read protection level
    pub mod RDP {
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

            /// 0b10001000: Level 1, memories readout protection active (writes 0x88)
            pub const Level1: u32 = 0b10001000;

            /// 0b10101010: Level 0, readout protection not active
            pub const Level0: u32 = 0b10101010;

            /// 0b11001100: Level 2, chip readout protection active
            pub const Level2: u32 = 0b11001100;
        }
    }

    /// System security enabled flag
    pub mod ESE {
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

            /// 0b0: Security disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Security enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// BOR reset Level
    pub mod BOR_LEV {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (3 bits: 0b111 << 9)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: BOR level 0. Reset level threshold is around 1.7 V
            pub const Level0: u32 = 0b000;

            /// 0b001: BOR level 1. Reset level threshold is around 2.0 V
            pub const Level1: u32 = 0b001;

            /// 0b010: BOR level 2. Reset level threshold is around 2.2 V
            pub const Level2: u32 = 0b010;

            /// 0b011: BOR level 3. Reset level threshold is around 2.5 V
            pub const Level3: u32 = 0b011;

            /// 0b100: BOR level 4. Reset level threshold is around 2.8 V
            pub const Level4: u32 = 0b100;
        }
    }

    /// nRST_STOP
    pub mod nRST_STOP {
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

            /// 0b0: Reset generated when entering the Standby mode
            pub const Enabled: u32 = 0b0;

            /// 0b1: No reset generated when entering the Standby mode
            pub const Disabled: u32 = 0b1;
        }
    }

    /// nRST_STDBY
    pub mod nRST_STDBY {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::nRST_STOP::RW;
    }

    /// nRSTSHDW
    pub mod nRST_SHDW {
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

            /// 0b0: Reset generated when entering the Shutdown mode
            pub const Enabled: u32 = 0b0;

            /// 0b1: No reset generated when entering the Shutdown mode
            pub const Disabled: u32 = 0b1;
        }
    }

    /// Independent watchdog selection
    pub mod IWDG_SW {
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

            /// 0b0: Hardware independent watchdog
            pub const Hardware: u32 = 0b0;

            /// 0b1: Software independent watchdog
            pub const Software: u32 = 0b1;
        }
    }

    /// Independent watchdog counter freeze in Stop mode
    pub mod IWDG_STOP {
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

            /// 0b0: Independent watchdog counter frozen in Stop mode
            pub const Frozen: u32 = 0b0;

            /// 0b1: Independent watchdog counter running in Stop mode
            pub const Running: u32 = 0b1;
        }
    }

    /// Independent watchdog counter freeze in Standby mode
    pub mod IWDG_STDBY {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Independent watchdog counter frozen in Standby mode
            pub const Frozen: u32 = 0b0;

            /// 0b1: Independent watchdog counter running in Standby mode
            pub const Running: u32 = 0b1;
        }
    }

    /// Window watchdog selection
    pub mod WWDG_SW {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Hardware window watchdog
            pub const Hardware: u32 = 0b0;

            /// 0b1: Software window watchdog
            pub const Software: u32 = 0b1;
        }
    }

    /// Boot configuration
    pub mod nBOOT1 {
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

            /// 0b0: When nSWBOOT0 is cleared, select boot mode together with nBOOT0
            pub const Clear: u32 = 0b0;

            /// 0b1: When nSWBOOT0 is cleared, select boot mode together with nBOOT0
            pub const Set: u32 = 0b1;
        }
    }

    /// SRAM2 parity check enable
    pub mod SRAM2_PE {
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

            /// 0b0: SRAM2 Parity check enabled
            pub const Enabled: u32 = 0b0;

            /// 0b1: SRAM2 Parity check disabled
            pub const Disabled: u32 = 0b1;
        }
    }

    /// SRAM2 Erase when system reset
    pub mod SRAM_RST {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: SRAM1 and SRAM2 erased when a system reset occurs
            pub const Reset: u32 = 0b0;

            /// 0b1: SRAM1 and SRAM2 not erased when a system reset occurs
            pub const NotReset: u32 = 0b1;
        }
    }

    /// Software BOOT0 selection
    pub mod nSWBOOT0 {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: BOOT0 taken from nBOOT0 in this register
            pub const Bit: u32 = 0b0;

            /// 0b1: BOOT0 taken from GPIO PH3/BOOT0
            pub const Pin: u32 = 0b1;
        }
    }

    /// nBOOT0 option bit
    pub mod nBOOT0 {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: When nSWBOOT0 is cleared, select boot mode together with nBOOT1
            pub const Clear: u32 = 0b0;

            /// 0b1: When nSWBOOT0 is cleared, select boot mode together with nBOOT1
            pub const Set: u32 = 0b1;
        }
    }

    /// CPU1 CM4 Unique Boot entry enable option bit
    pub mod BOOT_LOCK {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Boot lock is disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Boot lock is enabled
            pub const Enabled: u32 = 0b1;
        }
    }
}

/// Flash PCROP zone A Start address register
pub mod PCROP1ASR {

    /// PCROP1A area start offset
    pub mod PCROP1A_STRT {
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

/// Flash PCROP zone A End address register
pub mod PCROP1AER {

    /// PCROP area end offset
    pub mod PCROP1A_END {
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

    /// PCROP area preserved when RDP level decreased
    pub mod PCROP_RDP {
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

/// Flash WRP area A address register
pub mod WRP1AR {

    /// Bank 1 WRP first area start offset
    pub mod WRP1A_STRT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (7 bits: 0x7f << 0)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Bank 1 WRP first area A end offset
    pub mod WRP1A_END {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (7 bits: 0x7f << 16)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Flash WRP area B address register
pub mod WRP1BR {

    /// Bank 1 WRP second area B end offset
    pub mod WRP1B_STRT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (7 bits: 0x7f << 0)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Bank 1 WRP second area B start offset
    pub mod WRP1B_END {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (7 bits: 0x7f << 16)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Flash PCROP zone B Start address register
pub mod PCROP1BSR {

    /// Bank 1 WRP second area B end offset
    pub mod PCROP1B_STRT {
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

/// Flash PCROP zone B End address register
pub mod PCROP1BER {

    /// PCROP1B area end offset
    pub mod PCROP1B_END {
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
    /// Access control register
    pub ACR: RWRegister<u32>,

    _reserved1: [u8; 4],

    /// Flash key register
    pub KEYR: WORegister<u32>,

    /// Option byte key register
    pub OPTKEYR: WORegister<u32>,

    /// Status register
    pub SR: RWRegister<u32>,

    /// Flash control register
    pub CR: RWRegister<u32>,

    /// Flash ECC register
    pub ECCR: RWRegister<u32>,

    _reserved2: [u8; 4],

    /// Flash option register
    pub OPTR: RWRegister<u32>,

    /// Flash PCROP zone A Start address register
    pub PCROP1ASR: RWRegister<u32>,

    /// Flash PCROP zone A End address register
    pub PCROP1AER: RWRegister<u32>,

    /// Flash WRP area A address register
    pub WRP1AR: RWRegister<u32>,

    /// Flash WRP area B address register
    pub WRP1BR: RWRegister<u32>,

    /// Flash PCROP zone B Start address register
    pub PCROP1BSR: RWRegister<u32>,

    /// Flash PCROP zone B End address register
    pub PCROP1BER: RWRegister<u32>,
}
pub struct ResetValues {
    pub ACR: u32,
    pub KEYR: u32,
    pub OPTKEYR: u32,
    pub SR: u32,
    pub CR: u32,
    pub ECCR: u32,
    pub OPTR: u32,
    pub PCROP1ASR: u32,
    pub PCROP1AER: u32,
    pub WRP1AR: u32,
    pub WRP1BR: u32,
    pub PCROP1BSR: u32,
    pub PCROP1BER: u32,
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

/// Access functions for the FLASH peripheral instance
pub mod FLASH {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x58004000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in FLASH
    pub const reset: ResetValues = ResetValues {
        ACR: 0x00000600,
        KEYR: 0x00000000,
        OPTKEYR: 0x00000000,
        SR: 0x00000000,
        CR: 0xC0000000,
        ECCR: 0x00000000,
        OPTR: 0x3FFFF0AA,
        PCROP1ASR: 0xFFFFFFFF,
        PCROP1AER: 0xFFFFFF00,
        WRP1AR: 0xFF80FFFF,
        WRP1BR: 0xFF80FFFF,
        PCROP1BSR: 0xFFFFFFFF,
        PCROP1BER: 0xFFFFFF00,
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

    /// Unsafely steal FLASH
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        FLASH_TAKEN = true;
        INSTANCE
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
pub const FLASH: *const RegisterBlock = 0x58004000 as *const _;
