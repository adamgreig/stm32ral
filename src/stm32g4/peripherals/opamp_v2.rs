#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Operational amplifiers
//!
//! Used by: stm32g473, stm32g474, stm32g483, stm32g484

use crate::RWRegister;
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// OPAMP1 control/status register
pub mod OPAMP1_CSR {

    /// Operational amplifier Enable
    pub mod OPAEN {
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

    /// FORCE_VP
    pub mod FORCE_VP {
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

    /// VP_SEL
    pub mod VP_SEL {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (2 bits: 0b11 << 2)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// USERTRIM
    pub mod USERTRIM {
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

    /// VM_SEL
    pub mod VM_SEL {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (2 bits: 0b11 << 5)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// OPAHSM
    pub mod OPAHSM {
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

    /// OPAINTOEN
    pub mod OPAINTOEN {
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

    /// CALON
    pub mod CALON {
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

    /// CALSEL
    pub mod CALSEL {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (2 bits: 0b11 << 12)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PGA_GAIN
    pub mod PGA_GAIN {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (5 bits: 0b11111 << 14)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// TRIMOFFSETP
    pub mod TRIMOFFSETP {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (5 bits: 0b11111 << 19)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// TRIMOFFSETN
    pub mod TRIMOFFSETN {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (5 bits: 0b11111 << 24)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CALOUT
    pub mod CALOUT {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// LOCK
    pub mod LOCK {
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

/// OPAMP2 control/status register
pub mod OPAMP2_CSR {
    pub use super::OPAMP1_CSR::CALON;
    pub use super::OPAMP1_CSR::CALOUT;
    pub use super::OPAMP1_CSR::CALSEL;
    pub use super::OPAMP1_CSR::FORCE_VP;
    pub use super::OPAMP1_CSR::LOCK;
    pub use super::OPAMP1_CSR::OPAEN;
    pub use super::OPAMP1_CSR::OPAHSM;
    pub use super::OPAMP1_CSR::OPAINTOEN;
    pub use super::OPAMP1_CSR::PGA_GAIN;
    pub use super::OPAMP1_CSR::TRIMOFFSETN;
    pub use super::OPAMP1_CSR::TRIMOFFSETP;
    pub use super::OPAMP1_CSR::USERTRIM;
    pub use super::OPAMP1_CSR::VM_SEL;
    pub use super::OPAMP1_CSR::VP_SEL;
}

/// OPAMP3 control/status register
pub mod OPAMP3_CSR {
    pub use super::OPAMP1_CSR::CALON;
    pub use super::OPAMP1_CSR::CALOUT;
    pub use super::OPAMP1_CSR::CALSEL;
    pub use super::OPAMP1_CSR::FORCE_VP;
    pub use super::OPAMP1_CSR::LOCK;
    pub use super::OPAMP1_CSR::OPAEN;
    pub use super::OPAMP1_CSR::OPAHSM;
    pub use super::OPAMP1_CSR::OPAINTOEN;
    pub use super::OPAMP1_CSR::PGA_GAIN;
    pub use super::OPAMP1_CSR::TRIMOFFSETN;
    pub use super::OPAMP1_CSR::TRIMOFFSETP;
    pub use super::OPAMP1_CSR::USERTRIM;
    pub use super::OPAMP1_CSR::VM_SEL;
    pub use super::OPAMP1_CSR::VP_SEL;
}

/// OPAMP4 control/status register
pub mod OPAMP4_CSR {
    pub use super::OPAMP1_CSR::CALON;
    pub use super::OPAMP1_CSR::CALOUT;
    pub use super::OPAMP1_CSR::CALSEL;
    pub use super::OPAMP1_CSR::FORCE_VP;
    pub use super::OPAMP1_CSR::LOCK;
    pub use super::OPAMP1_CSR::OPAEN;
    pub use super::OPAMP1_CSR::OPAHSM;
    pub use super::OPAMP1_CSR::OPAINTOEN;
    pub use super::OPAMP1_CSR::PGA_GAIN;
    pub use super::OPAMP1_CSR::TRIMOFFSETN;
    pub use super::OPAMP1_CSR::TRIMOFFSETP;
    pub use super::OPAMP1_CSR::USERTRIM;
    pub use super::OPAMP1_CSR::VM_SEL;
    pub use super::OPAMP1_CSR::VP_SEL;
}

/// OPAMP5 control/status register
pub mod OPAMP5_CSR {
    pub use super::OPAMP1_CSR::CALON;
    pub use super::OPAMP1_CSR::CALOUT;
    pub use super::OPAMP1_CSR::CALSEL;
    pub use super::OPAMP1_CSR::FORCE_VP;
    pub use super::OPAMP1_CSR::LOCK;
    pub use super::OPAMP1_CSR::OPAEN;
    pub use super::OPAMP1_CSR::OPAHSM;
    pub use super::OPAMP1_CSR::OPAINTOEN;
    pub use super::OPAMP1_CSR::PGA_GAIN;
    pub use super::OPAMP1_CSR::TRIMOFFSETN;
    pub use super::OPAMP1_CSR::TRIMOFFSETP;
    pub use super::OPAMP1_CSR::USERTRIM;
    pub use super::OPAMP1_CSR::VM_SEL;
    pub use super::OPAMP1_CSR::VP_SEL;
}

/// OPAMP6 control/status register
pub mod OPAMP6_CSR {
    pub use super::OPAMP1_CSR::CALON;
    pub use super::OPAMP1_CSR::CALOUT;
    pub use super::OPAMP1_CSR::CALSEL;
    pub use super::OPAMP1_CSR::FORCE_VP;
    pub use super::OPAMP1_CSR::LOCK;
    pub use super::OPAMP1_CSR::OPAEN;
    pub use super::OPAMP1_CSR::OPAHSM;
    pub use super::OPAMP1_CSR::OPAINTOEN;
    pub use super::OPAMP1_CSR::PGA_GAIN;
    pub use super::OPAMP1_CSR::TRIMOFFSETN;
    pub use super::OPAMP1_CSR::TRIMOFFSETP;
    pub use super::OPAMP1_CSR::USERTRIM;
    pub use super::OPAMP1_CSR::VM_SEL;
    pub use super::OPAMP1_CSR::VP_SEL;
}

/// OPAMP1 control/status register
pub mod OPAMP1_TCMR {

    /// VMS_SEL
    pub mod VMS_SEL {
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

    /// VPS_SEL
    pub mod VPS_SEL {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (2 bits: 0b11 << 1)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// T1CM_EN
    pub mod T1CM_EN {
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

    /// T8CM_EN
    pub mod T8CM_EN {
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

    /// T20CM_EN
    pub mod T20CM_EN {
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

    /// LOCK
    pub mod LOCK {
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

/// OPAMP2 control/status register
pub mod OPAMP2_TCMR {
    pub use super::OPAMP1_TCMR::LOCK;
    pub use super::OPAMP1_TCMR::T1CM_EN;
    pub use super::OPAMP1_TCMR::T20CM_EN;
    pub use super::OPAMP1_TCMR::T8CM_EN;
    pub use super::OPAMP1_TCMR::VMS_SEL;
    pub use super::OPAMP1_TCMR::VPS_SEL;
}

/// OPAMP3 control/status register
pub mod OPAMP3_TCMR {
    pub use super::OPAMP1_TCMR::LOCK;
    pub use super::OPAMP1_TCMR::T1CM_EN;
    pub use super::OPAMP1_TCMR::T20CM_EN;
    pub use super::OPAMP1_TCMR::T8CM_EN;
    pub use super::OPAMP1_TCMR::VMS_SEL;
    pub use super::OPAMP1_TCMR::VPS_SEL;
}

/// OPAMP4 control/status register
pub mod OPAMP4_TCMR {
    pub use super::OPAMP1_TCMR::LOCK;
    pub use super::OPAMP1_TCMR::T1CM_EN;
    pub use super::OPAMP1_TCMR::T20CM_EN;
    pub use super::OPAMP1_TCMR::T8CM_EN;
    pub use super::OPAMP1_TCMR::VMS_SEL;
    pub use super::OPAMP1_TCMR::VPS_SEL;
}

/// OPAMP5 control/status register
pub mod OPAMP5_TCMR {
    pub use super::OPAMP1_TCMR::LOCK;
    pub use super::OPAMP1_TCMR::T1CM_EN;
    pub use super::OPAMP1_TCMR::T20CM_EN;
    pub use super::OPAMP1_TCMR::T8CM_EN;
    pub use super::OPAMP1_TCMR::VMS_SEL;
    pub use super::OPAMP1_TCMR::VPS_SEL;
}

/// OPAMP6 control/status register
pub mod OPAMP6_TCMR {
    pub use super::OPAMP1_TCMR::LOCK;
    pub use super::OPAMP1_TCMR::T1CM_EN;
    pub use super::OPAMP1_TCMR::T20CM_EN;
    pub use super::OPAMP1_TCMR::T8CM_EN;
    pub use super::OPAMP1_TCMR::VMS_SEL;
    pub use super::OPAMP1_TCMR::VPS_SEL;
}
pub struct RegisterBlock {
    /// OPAMP1 control/status register
    pub OPAMP1_CSR: RWRegister<u32>,

    /// OPAMP2 control/status register
    pub OPAMP2_CSR: RWRegister<u32>,

    /// OPAMP3 control/status register
    pub OPAMP3_CSR: RWRegister<u32>,

    /// OPAMP4 control/status register
    pub OPAMP4_CSR: RWRegister<u32>,

    /// OPAMP5 control/status register
    pub OPAMP5_CSR: RWRegister<u32>,

    /// OPAMP6 control/status register
    pub OPAMP6_CSR: RWRegister<u32>,

    /// OPAMP1 control/status register
    pub OPAMP1_TCMR: RWRegister<u32>,

    /// OPAMP2 control/status register
    pub OPAMP2_TCMR: RWRegister<u32>,

    /// OPAMP3 control/status register
    pub OPAMP3_TCMR: RWRegister<u32>,

    /// OPAMP4 control/status register
    pub OPAMP4_TCMR: RWRegister<u32>,

    /// OPAMP5 control/status register
    pub OPAMP5_TCMR: RWRegister<u32>,

    /// OPAMP6 control/status register
    pub OPAMP6_TCMR: RWRegister<u32>,
}
pub struct ResetValues {
    pub OPAMP1_CSR: u32,
    pub OPAMP2_CSR: u32,
    pub OPAMP3_CSR: u32,
    pub OPAMP4_CSR: u32,
    pub OPAMP5_CSR: u32,
    pub OPAMP6_CSR: u32,
    pub OPAMP1_TCMR: u32,
    pub OPAMP2_TCMR: u32,
    pub OPAMP3_TCMR: u32,
    pub OPAMP4_TCMR: u32,
    pub OPAMP5_TCMR: u32,
    pub OPAMP6_TCMR: u32,
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
