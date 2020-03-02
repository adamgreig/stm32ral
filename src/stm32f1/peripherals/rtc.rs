#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Real time clock
//!
//! Used by: stm32f101, stm32f102, stm32f103, stm32f107

use crate::{RORegister, RWRegister, WORegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// RTC Control Register High
pub mod CRH {

    /// Second interrupt Enable
    pub mod SECIE {
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

            /// 0b0: Second interrupt is masked
            pub const Disabled: u32 = 0b0;

            /// 0b1: Second interrupt is enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Alarm interrupt Enable
    pub mod ALRIE {
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

            /// 0b0: Alarm interrupt is masked
            pub const Disabled: u32 = 0b0;

            /// 0b1: Alarm interrupt is enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Overflow interrupt Enable
    pub mod OWIE {
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

            /// 0b0: Overflow interrupt is masked
            pub const Disabled: u32 = 0b0;

            /// 0b1: Overflow interrupt is enabled
            pub const Enabled: u32 = 0b1;
        }
    }
}

/// RTC Control Register Low
pub mod CRL {

    /// Second Flag
    pub mod SECF {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Second flag condition not met
            pub const NoPrescalerOverflow: u32 = 0b0;

            /// 0b1: Second flag condition met
            pub const PrescalerOverflow: u32 = 0b1;
        }
        /// Write-only values
        pub mod W {

            /// 0b0: Clear flag
            pub const Clear: u32 = 0b0;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Alarm Flag
    pub mod ALRF {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Alarm not detected
            pub const NoAlarm: u32 = 0b0;

            /// 0b1: Alarm detected
            pub const Alarm: u32 = 0b1;
        }
        pub use super::SECF::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Overflow Flag
    pub mod OWF {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Overflow not detected
            pub const NoOverflow: u32 = 0b0;

            /// 0b1: 32-bit programmable counter overflow occurred
            pub const Overflow: u32 = 0b1;
        }
        pub use super::SECF::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Registers Synchronized Flag
    pub mod RSF {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Registers not yet synchronized
            pub const NotSynchronized: u32 = 0b0;

            /// 0b1: Registers synchronized
            pub const Synchronized: u32 = 0b1;
        }
        pub use super::SECF::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Configuration Flag
    pub mod CNF {
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

            /// 0b0: Exit configuration mode (start update of RTC registers)
            pub const Exit: u32 = 0b0;

            /// 0b1: Enter configuration mode
            pub const Enter: u32 = 0b1;
        }
    }

    /// RTC operation OFF
    pub mod RTOFF {
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

            /// 0b0: Last write operation on RTC registers is still ongoing
            pub const Enabled: u32 = 0b0;

            /// 0b1: Last write operation on RTC registers terminated
            pub const Disabled: u32 = 0b1;
        }
    }
}

/// RTC Prescaler Load Register High
pub mod PRLH {

    /// RTC Prescaler Load Register High
    pub mod PRLH {
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

/// RTC Prescaler Load Register Low
pub mod PRLL {

    /// RTC Prescaler Divider Register Low
    pub mod PRLL {
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

/// RTC Prescaler Divider Register High
pub mod DIVH {

    /// RTC prescaler divider register high
    pub mod DIVH {
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

/// RTC Prescaler Divider Register Low
pub mod DIVL {

    /// RTC prescaler divider register Low
    pub mod DIVL {
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

/// RTC Counter Register High
pub mod CNTH {

    /// RTC counter register high
    pub mod CNTH {
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

/// RTC Counter Register Low
pub mod CNTL {

    /// RTC counter register Low
    pub mod CNTL {
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

/// RTC Alarm Register High
pub mod ALRH {

    /// RTC alarm register high
    pub mod ALRH {
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

/// RTC Alarm Register Low
pub mod ALRL {

    /// RTC alarm register low
    pub mod ALRL {
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
    /// RTC Control Register High
    pub CRH: RWRegister<u32>,

    /// RTC Control Register Low
    pub CRL: RWRegister<u32>,

    /// RTC Prescaler Load Register High
    pub PRLH: WORegister<u32>,

    /// RTC Prescaler Load Register Low
    pub PRLL: WORegister<u32>,

    /// RTC Prescaler Divider Register High
    pub DIVH: RORegister<u32>,

    /// RTC Prescaler Divider Register Low
    pub DIVL: RORegister<u32>,

    /// RTC Counter Register High
    pub CNTH: RWRegister<u32>,

    /// RTC Counter Register Low
    pub CNTL: RWRegister<u32>,

    /// RTC Alarm Register High
    pub ALRH: WORegister<u32>,

    /// RTC Alarm Register Low
    pub ALRL: WORegister<u32>,
}
pub struct ResetValues {
    pub CRH: u32,
    pub CRL: u32,
    pub PRLH: u32,
    pub PRLL: u32,
    pub DIVH: u32,
    pub DIVL: u32,
    pub CNTH: u32,
    pub CNTL: u32,
    pub ALRH: u32,
    pub ALRL: u32,
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
