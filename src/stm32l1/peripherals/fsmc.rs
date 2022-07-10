#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Flexible static memory controller
//!
//! Used by: stm32l100, stm32l151, stm32l162

use crate::RWRegister;
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// BCR1
pub mod BCR1 {

    /// CBURSTRW
    pub mod CBURSTRW {
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

            /// 0b0: Write operations are always performed in asynchronous mode
            pub const Disabled: u32 = 0b0;

            /// 0b1: Write operations are performed in synchronous mode
            pub const Enabled: u32 = 0b1;
        }
    }

    /// ASYNCWAIT
    pub mod ASYNCWAIT {
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

            /// 0b0: Wait signal not used in asynchronous mode
            pub const Disabled: u32 = 0b0;

            /// 0b1: Wait signal used even in asynchronous mode
            pub const Enabled: u32 = 0b1;
        }
    }

    /// EXTMOD
    pub mod EXTMOD {
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

            /// 0b0: Values inside the FMC_BWTR are not taken into account
            pub const Disabled: u32 = 0b0;

            /// 0b1: Values inside the FMC_BWTR are taken into account
            pub const Enabled: u32 = 0b1;
        }
    }

    /// WAITEN
    pub mod WAITEN {
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

            /// 0b0: Values inside the FMC_BWTR are taken into account
            pub const Disabled: u32 = 0b0;

            /// 0b1: NWAIT signal enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// WREN
    pub mod WREN {
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

            /// 0b0: Write operations disabled for the bank by the FMC
            pub const Disabled: u32 = 0b0;

            /// 0b1: Write operations enabled for the bank by the FMC
            pub const Enabled: u32 = 0b1;
        }
    }

    /// WAITCFG
    pub mod WAITCFG {
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

            /// 0b0: NWAIT signal is active one data cycle before wait state
            pub const BeforeWaitState: u32 = 0b0;

            /// 0b1: NWAIT signal is active during wait state
            pub const DuringWaitState: u32 = 0b1;
        }
    }

    /// WRAPMOD
    pub mod WRAPMOD {
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

    /// WAITPOL
    pub mod WAITPOL {
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

            /// 0b0: NWAIT active low
            pub const ActiveLow: u32 = 0b0;

            /// 0b1: NWAIT active high
            pub const ActiveHigh: u32 = 0b1;
        }
    }

    /// BURSTEN
    pub mod BURSTEN {
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

            /// 0b0: Burst mode disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Burst mode enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// FACCEN
    pub mod FACCEN {
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

            /// 0b0: Corresponding NOR Flash memory access is disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Corresponding NOR Flash memory access is enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// MWID
    pub mod MWID {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (2 bits: 0b11 << 4)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Memory data bus width 8 bits
            pub const Bits8: u32 = 0b00;

            /// 0b01: Memory data bus width 16 bits
            pub const Bits16: u32 = 0b01;

            /// 0b10: Memory data bus width 32 bits
            pub const Bits32: u32 = 0b10;
        }
    }

    /// MTYP
    pub mod MTYP {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (2 bits: 0b11 << 2)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: SRAM memory type
            pub const SRAM: u32 = 0b00;

            /// 0b01: PSRAM (CRAM) memory type
            pub const PSRAM: u32 = 0b01;

            /// 0b10: NOR Flash/OneNAND Flash
            pub const Flash: u32 = 0b10;
        }
    }

    /// MUXEN
    pub mod MUXEN {
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

            /// 0b0: Address/Data non-multiplexed
            pub const Disabled: u32 = 0b0;

            /// 0b1: Address/Data multiplexed on databus
            pub const Enabled: u32 = 0b1;
        }
    }

    /// MBKEN
    pub mod MBKEN {
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

            /// 0b0: Corresponding memory bank is disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Corresponding memory bank is enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// CRAM page size
    pub mod CPSIZE {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (3 bits: 0b111 << 16)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: No burst split when crossing page boundary
            pub const NoBurstSplit: u32 = 0b000;

            /// 0b001: 128 bytes CRAM page size
            pub const Bytes128: u32 = 0b001;

            /// 0b010: 256 bytes CRAM page size
            pub const Bytes256: u32 = 0b010;

            /// 0b011: 512 bytes CRAM page size
            pub const Bytes512: u32 = 0b011;

            /// 0b100: 1024 bytes CRAM page size
            pub const Bytes1024: u32 = 0b100;
        }
    }
}

/// BTR1
pub mod BTR1 {

    /// ACCMOD
    pub mod ACCMOD {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (2 bits: 0b11 << 28)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Access mode A
            pub const A: u32 = 0b00;

            /// 0b01: Access mode B
            pub const B: u32 = 0b01;

            /// 0b10: Access mode C
            pub const C: u32 = 0b10;

            /// 0b11: Access mode D
            pub const D: u32 = 0b11;
        }
    }

    /// DATLAT
    pub mod DATLAT {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (4 bits: 0b1111 << 24)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CLKDIV
    pub mod CLKDIV {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (4 bits: 0b1111 << 20)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// BUSTURN
    pub mod BUSTURN {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (4 bits: 0b1111 << 16)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DATAST
    pub mod DATAST {
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

    /// ADDHLD
    pub mod ADDHLD {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// ADDSET
    pub mod ADDSET {
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

/// BTR1
pub mod BTR2 {
    pub use super::BTR1::ACCMOD;
    pub use super::BTR1::ADDHLD;
    pub use super::BTR1::ADDSET;
    pub use super::BTR1::BUSTURN;
    pub use super::BTR1::CLKDIV;
    pub use super::BTR1::DATAST;
    pub use super::BTR1::DATLAT;
}

/// BTR1
pub mod BTR3 {
    pub use super::BTR1::ACCMOD;
    pub use super::BTR1::ADDHLD;
    pub use super::BTR1::ADDSET;
    pub use super::BTR1::BUSTURN;
    pub use super::BTR1::CLKDIV;
    pub use super::BTR1::DATAST;
    pub use super::BTR1::DATLAT;
}

/// BTR1
pub mod BTR4 {
    pub use super::BTR1::ACCMOD;
    pub use super::BTR1::ADDHLD;
    pub use super::BTR1::ADDSET;
    pub use super::BTR1::BUSTURN;
    pub use super::BTR1::CLKDIV;
    pub use super::BTR1::DATAST;
    pub use super::BTR1::DATLAT;
}

/// BCR2
pub mod BCR2 {
    pub use super::BCR1::ASYNCWAIT;
    pub use super::BCR1::BURSTEN;
    pub use super::BCR1::CBURSTRW;
    pub use super::BCR1::CPSIZE;
    pub use super::BCR1::EXTMOD;
    pub use super::BCR1::FACCEN;
    pub use super::BCR1::MBKEN;
    pub use super::BCR1::MTYP;
    pub use super::BCR1::MUXEN;
    pub use super::BCR1::MWID;
    pub use super::BCR1::WAITCFG;
    pub use super::BCR1::WAITEN;
    pub use super::BCR1::WAITPOL;
    pub use super::BCR1::WRAPMOD;
    pub use super::BCR1::WREN;
}

/// BCR2
pub mod BCR3 {
    pub use super::BCR1::ASYNCWAIT;
    pub use super::BCR1::BURSTEN;
    pub use super::BCR1::CBURSTRW;
    pub use super::BCR1::CPSIZE;
    pub use super::BCR1::EXTMOD;
    pub use super::BCR1::FACCEN;
    pub use super::BCR1::MBKEN;
    pub use super::BCR1::MTYP;
    pub use super::BCR1::MUXEN;
    pub use super::BCR1::MWID;
    pub use super::BCR1::WAITCFG;
    pub use super::BCR1::WAITEN;
    pub use super::BCR1::WAITPOL;
    pub use super::BCR1::WRAPMOD;
    pub use super::BCR1::WREN;
}

/// BCR2
pub mod BCR4 {
    pub use super::BCR1::ASYNCWAIT;
    pub use super::BCR1::BURSTEN;
    pub use super::BCR1::CBURSTRW;
    pub use super::BCR1::CPSIZE;
    pub use super::BCR1::EXTMOD;
    pub use super::BCR1::FACCEN;
    pub use super::BCR1::MBKEN;
    pub use super::BCR1::MTYP;
    pub use super::BCR1::MUXEN;
    pub use super::BCR1::MWID;
    pub use super::BCR1::WAITCFG;
    pub use super::BCR1::WAITEN;
    pub use super::BCR1::WAITPOL;
    pub use super::BCR1::WRAPMOD;
    pub use super::BCR1::WREN;
}

/// BWTR1
pub mod BWTR1 {

    /// ACCMOD
    pub mod ACCMOD {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (2 bits: 0b11 << 28)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Access mode A
            pub const A: u32 = 0b00;

            /// 0b01: Access mode B
            pub const B: u32 = 0b01;

            /// 0b10: Access mode C
            pub const C: u32 = 0b10;

            /// 0b11: Access mode D
            pub const D: u32 = 0b11;
        }
    }

    /// DATLAT
    pub mod DATLAT {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (4 bits: 0b1111 << 24)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CLKDIV
    pub mod CLKDIV {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (4 bits: 0b1111 << 20)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DATAST
    pub mod DATAST {
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

    /// ADDHLD
    pub mod ADDHLD {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// ADDSET
    pub mod ADDSET {
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

    /// Bus turnaround phase duration
    pub mod BUSTURN {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (4 bits: 0b1111 << 16)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// BWTR1
pub mod BWTR2 {
    pub use super::BWTR1::ACCMOD;
    pub use super::BWTR1::ADDHLD;
    pub use super::BWTR1::ADDSET;
    pub use super::BWTR1::BUSTURN;
    pub use super::BWTR1::CLKDIV;
    pub use super::BWTR1::DATAST;
    pub use super::BWTR1::DATLAT;
}

/// BWTR1
pub mod BWTR3 {
    pub use super::BWTR1::ACCMOD;
    pub use super::BWTR1::ADDHLD;
    pub use super::BWTR1::ADDSET;
    pub use super::BWTR1::BUSTURN;
    pub use super::BWTR1::CLKDIV;
    pub use super::BWTR1::DATAST;
    pub use super::BWTR1::DATLAT;
}

/// BWTR1
pub mod BWTR4 {
    pub use super::BWTR1::ACCMOD;
    pub use super::BWTR1::ADDHLD;
    pub use super::BWTR1::ADDSET;
    pub use super::BWTR1::BUSTURN;
    pub use super::BWTR1::CLKDIV;
    pub use super::BWTR1::DATAST;
    pub use super::BWTR1::DATLAT;
}
#[repr(C)]
pub struct RegisterBlock {
    /// BCR1
    pub BCR1: RWRegister<u32>,

    /// BTR1
    pub BTR1: RWRegister<u32>,

    /// BCR2
    pub BCR2: RWRegister<u32>,

    /// BTR1
    pub BTR2: RWRegister<u32>,

    /// BCR2
    pub BCR3: RWRegister<u32>,

    /// BTR1
    pub BTR3: RWRegister<u32>,

    /// BCR2
    pub BCR4: RWRegister<u32>,

    /// BTR1
    pub BTR4: RWRegister<u32>,

    _reserved1: [u8; 228],

    /// BWTR1
    pub BWTR1: RWRegister<u32>,

    _reserved2: [u8; 4],

    /// BWTR1
    pub BWTR2: RWRegister<u32>,

    _reserved3: [u8; 4],

    /// BWTR1
    pub BWTR3: RWRegister<u32>,

    _reserved4: [u8; 4],

    /// BWTR1
    pub BWTR4: RWRegister<u32>,
}
pub struct ResetValues {
    pub BCR1: u32,
    pub BTR1: u32,
    pub BCR2: u32,
    pub BTR2: u32,
    pub BCR3: u32,
    pub BTR3: u32,
    pub BCR4: u32,
    pub BTR4: u32,
    pub BWTR1: u32,
    pub BWTR2: u32,
    pub BWTR3: u32,
    pub BWTR4: u32,
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
