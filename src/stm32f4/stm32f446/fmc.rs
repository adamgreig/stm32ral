#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Flexible memory controller

use crate::{RORegister, RWRegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// SRAM/NOR-Flash chip-select control register 1
pub mod BCR1 {

    /// CCLKEN
    pub mod CCLKEN {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: The FMC_CLK is generated continuously during asynchronous and synchronous access. The FMC_CLK clock is activated when the CCLKEN is set
            pub const Disabled: u32 = 0b0;

            /// 0b1: The FMC_CLK is only generated during the synchronous memory access (read/write transaction)
            pub const Enabled: u32 = 0b1;
        }
    }

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

    /// Write FIFO disable
    pub mod WFDIS {
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

            /// 0b0: Write FIFO enabled
            pub const Enabled: u32 = 0b0;

            /// 0b1: Write FIFO disabled
            pub const Disabled: u32 = 0b1;
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

/// SRAM/NOR-Flash chip-select timing register 1
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

/// SRAM/NOR-Flash chip-select timing register 1
pub mod BTR2 {
    pub use super::BTR1::ACCMOD;
    pub use super::BTR1::ADDHLD;
    pub use super::BTR1::ADDSET;
    pub use super::BTR1::BUSTURN;
    pub use super::BTR1::CLKDIV;
    pub use super::BTR1::DATAST;
    pub use super::BTR1::DATLAT;
}

/// SRAM/NOR-Flash chip-select timing register 1
pub mod BTR3 {
    pub use super::BTR1::ACCMOD;
    pub use super::BTR1::ADDHLD;
    pub use super::BTR1::ADDSET;
    pub use super::BTR1::BUSTURN;
    pub use super::BTR1::CLKDIV;
    pub use super::BTR1::DATAST;
    pub use super::BTR1::DATLAT;
}

/// SRAM/NOR-Flash chip-select timing register 1
pub mod BTR4 {
    pub use super::BTR1::ACCMOD;
    pub use super::BTR1::ADDHLD;
    pub use super::BTR1::ADDSET;
    pub use super::BTR1::BUSTURN;
    pub use super::BTR1::CLKDIV;
    pub use super::BTR1::DATAST;
    pub use super::BTR1::DATLAT;
}

/// SRAM/NOR-Flash chip-select control register 2
pub mod BCR2 {

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

/// SRAM/NOR-Flash chip-select control register 2
pub mod BCR3 {
    pub use super::BCR2::ASYNCWAIT;
    pub use super::BCR2::BURSTEN;
    pub use super::BCR2::CBURSTRW;
    pub use super::BCR2::CPSIZE;
    pub use super::BCR2::EXTMOD;
    pub use super::BCR2::FACCEN;
    pub use super::BCR2::MBKEN;
    pub use super::BCR2::MTYP;
    pub use super::BCR2::MUXEN;
    pub use super::BCR2::MWID;
    pub use super::BCR2::WAITCFG;
    pub use super::BCR2::WAITEN;
    pub use super::BCR2::WAITPOL;
    pub use super::BCR2::WRAPMOD;
    pub use super::BCR2::WREN;
}

/// SRAM/NOR-Flash chip-select control register 2
pub mod BCR4 {
    pub use super::BCR2::ASYNCWAIT;
    pub use super::BCR2::BURSTEN;
    pub use super::BCR2::CBURSTRW;
    pub use super::BCR2::CPSIZE;
    pub use super::BCR2::EXTMOD;
    pub use super::BCR2::FACCEN;
    pub use super::BCR2::MBKEN;
    pub use super::BCR2::MTYP;
    pub use super::BCR2::MUXEN;
    pub use super::BCR2::MWID;
    pub use super::BCR2::WAITCFG;
    pub use super::BCR2::WAITEN;
    pub use super::BCR2::WAITPOL;
    pub use super::BCR2::WRAPMOD;
    pub use super::BCR2::WREN;
}

/// PC Card/NAND Flash control register 2
pub mod PCR2 {

    /// ECCPS
    pub mod ECCPS {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (3 bits: 0b111 << 17)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: ECC page size 256 bytes
            pub const Bytes256: u32 = 0b000;

            /// 0b001: ECC page size 512 bytes
            pub const Bytes512: u32 = 0b001;

            /// 0b010: ECC page size 1024 bytes
            pub const Bytes1024: u32 = 0b010;

            /// 0b011: ECC page size 2048 bytes
            pub const Bytes2048: u32 = 0b011;

            /// 0b100: ECC page size 4096 bytes
            pub const Bytes4096: u32 = 0b100;

            /// 0b101: ECC page size 8192 bytes
            pub const Bytes8192: u32 = 0b101;
        }
    }

    /// TAR
    pub mod TAR {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (4 bits: 0b1111 << 13)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// TCLR
    pub mod TCLR {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (4 bits: 0b1111 << 9)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// ECCEN
    pub mod ECCEN {
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

            /// 0b0: ECC logic is disabled and reset
            pub const Disabled: u32 = 0b0;

            /// 0b1: ECC logic is enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// PWID
    pub mod PWID {
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

            /// 0b00: External memory device width 8 bits
            pub const Bits8: u32 = 0b00;

            /// 0b01: External memory device width 16 bits
            pub const Bits16: u32 = 0b01;
        }
    }

    /// PTYP
    pub mod PTYP {
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

            /// 0b1: NAND Flash
            pub const NANDFlash: u32 = 0b1;
        }
    }

    /// PBKEN
    pub mod PBKEN {
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

            /// 0b0: Corresponding memory bank is disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Corresponding memory bank is enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// PWAITEN
    pub mod PWAITEN {
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

            /// 0b0: Wait feature disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Wait feature enabled
            pub const Enabled: u32 = 0b1;
        }
    }
}

/// PC Card/NAND Flash control register 2
pub mod PCR3 {
    pub use super::PCR2::ECCEN;
    pub use super::PCR2::ECCPS;
    pub use super::PCR2::PBKEN;
    pub use super::PCR2::PTYP;
    pub use super::PCR2::PWAITEN;
    pub use super::PCR2::PWID;
    pub use super::PCR2::TAR;
    pub use super::PCR2::TCLR;
}

/// PC Card/NAND Flash control register 2
pub mod PCR4 {
    pub use super::PCR2::ECCEN;
    pub use super::PCR2::ECCPS;
    pub use super::PCR2::PBKEN;
    pub use super::PCR2::PTYP;
    pub use super::PCR2::PWAITEN;
    pub use super::PCR2::PWID;
    pub use super::PCR2::TAR;
    pub use super::PCR2::TCLR;
}

/// FIFO status and interrupt register 2
pub mod SR2 {

    /// FEMPT
    pub mod FEMPT {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: FIFO not empty
            pub const NotEmpty: u32 = 0b0;

            /// 0b1: FIFO empty
            pub const Empty: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// IFEN
    pub mod IFEN {
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

            /// 0b0: Interrupt falling edge detection request disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Interrupt falling edge detection request enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// ILEN
    pub mod ILEN {
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

            /// 0b0: Interrupt high-level detection request disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Interrupt high-level detection request enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// IREN
    pub mod IREN {
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

            /// 0b0: Interrupt rising edge detection request disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Interrupt rising edge detection request enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// IFS
    pub mod IFS {
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

            /// 0b0: Interrupt falling edge did not occur
            pub const DidNotOccur: u32 = 0b0;

            /// 0b1: Interrupt falling edge occurred
            pub const Occurred: u32 = 0b1;
        }
    }

    /// ILS
    pub mod ILS {
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

            /// 0b0: Interrupt high-level did not occur
            pub const DidNotOccur: u32 = 0b0;

            /// 0b1: Interrupt high-level occurred
            pub const Occurred: u32 = 0b1;
        }
    }

    /// IRS
    pub mod IRS {
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

            /// 0b0: Interrupt rising edge did not occur
            pub const DidNotOccur: u32 = 0b0;

            /// 0b1: Interrupt rising edge occurred
            pub const Occurred: u32 = 0b1;
        }
    }
}

/// FIFO status and interrupt register 2
pub mod SR3 {
    pub use super::SR2::FEMPT;
    pub use super::SR2::IFEN;
    pub use super::SR2::IFS;
    pub use super::SR2::ILEN;
    pub use super::SR2::ILS;
    pub use super::SR2::IREN;
    pub use super::SR2::IRS;
}

/// FIFO status and interrupt register 2
pub mod SR4 {
    pub use super::SR2::FEMPT;
    pub use super::SR2::IFEN;
    pub use super::SR2::IFS;
    pub use super::SR2::ILEN;
    pub use super::SR2::ILS;
    pub use super::SR2::IREN;
    pub use super::SR2::IRS;
}

/// Common memory space timing register 2
pub mod PMEM2 {

    /// MEMHIZx
    pub mod MEMHIZ {
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

    /// MEMHOLDx
    pub mod MEMHOLD {
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

    /// MEMWAITx
    pub mod MEMWAIT {
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

    /// MEMSETx
    pub mod MEMSET {
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

/// Attribute memory space timing register 2
pub mod PATT2 {

    /// ATTHIZx
    pub mod ATTHIZ {
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

    /// ATTHOLDx
    pub mod ATTHOLD {
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

    /// ATTWAITx
    pub mod ATTWAIT {
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

    /// ATTSETx
    pub mod ATTSET {
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

/// ECC result register 2
pub mod ECCR2 {

    /// ECCx
    pub mod ECC {
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

/// Common memory space timing register 3
pub mod PMEM3 {
    pub use super::PMEM2::MEMHIZ;
    pub use super::PMEM2::MEMHOLD;
    pub use super::PMEM2::MEMSET;
    pub use super::PMEM2::MEMWAIT;
}

/// Attribute memory space timing register 3
pub mod PATT3 {
    pub use super::PATT2::ATTHIZ;
    pub use super::PATT2::ATTHOLD;
    pub use super::PATT2::ATTSET;
    pub use super::PATT2::ATTWAIT;
}

/// ECC result register 3
pub mod ECCR3 {
    pub use super::ECCR2::ECC;
}

/// Common memory space timing register 4
pub mod PMEM4 {
    pub use super::PMEM2::MEMHIZ;
    pub use super::PMEM2::MEMHOLD;
    pub use super::PMEM2::MEMSET;
    pub use super::PMEM2::MEMWAIT;
}

/// Attribute memory space timing register 4
pub mod PATT4 {
    pub use super::PATT2::ATTHIZ;
    pub use super::PATT2::ATTHOLD;
    pub use super::PATT2::ATTSET;
    pub use super::PATT2::ATTWAIT;
}

/// I/O space timing register 4
pub mod PIO4 {

    /// IOHIZx
    pub mod IOHIZx {
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

    /// IOHOLDx
    pub mod IOHOLDx {
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

    /// IOWAITx
    pub mod IOWAITx {
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

    /// IOSETx
    pub mod IOSETx {
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

/// SRAM/NOR-Flash write timing registers 1
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

/// SRAM/NOR-Flash write timing registers 1
pub mod BWTR2 {
    pub use super::BWTR1::ACCMOD;
    pub use super::BWTR1::ADDHLD;
    pub use super::BWTR1::ADDSET;
    pub use super::BWTR1::BUSTURN;
    pub use super::BWTR1::CLKDIV;
    pub use super::BWTR1::DATAST;
    pub use super::BWTR1::DATLAT;
}

/// SRAM/NOR-Flash write timing registers 1
pub mod BWTR3 {
    pub use super::BWTR1::ACCMOD;
    pub use super::BWTR1::ADDHLD;
    pub use super::BWTR1::ADDSET;
    pub use super::BWTR1::BUSTURN;
    pub use super::BWTR1::CLKDIV;
    pub use super::BWTR1::DATAST;
    pub use super::BWTR1::DATLAT;
}

/// SRAM/NOR-Flash write timing registers 1
pub mod BWTR4 {
    pub use super::BWTR1::ACCMOD;
    pub use super::BWTR1::ADDHLD;
    pub use super::BWTR1::ADDSET;
    pub use super::BWTR1::BUSTURN;
    pub use super::BWTR1::CLKDIV;
    pub use super::BWTR1::DATAST;
    pub use super::BWTR1::DATLAT;
}

/// SDRAM Control Register 1
pub mod SDCR1 {

    /// Number of column address bits
    pub mod NC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (2 bits: 0b11 << 0)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: 8 bits
            pub const Bits8: u32 = 0b00;

            /// 0b01: 9 bits
            pub const Bits9: u32 = 0b01;

            /// 0b10: 10 bits
            pub const Bits10: u32 = 0b10;

            /// 0b11: 11 bits
            pub const Bits11: u32 = 0b11;
        }
    }

    /// Number of row address bits
    pub mod NR {
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

            /// 0b00: 11 bits
            pub const Bits11: u32 = 0b00;

            /// 0b01: 12 bits
            pub const Bits12: u32 = 0b01;

            /// 0b10: 13 bits
            pub const Bits13: u32 = 0b10;
        }
    }

    /// Memory data bus width
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

    /// Number of internal banks
    pub mod NB {
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

            /// 0b0: Two internal Banks
            pub const NB2: u32 = 0b0;

            /// 0b1: Four internal Banks
            pub const NB4: u32 = 0b1;
        }
    }

    /// CAS latency
    pub mod CAS {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (2 bits: 0b11 << 7)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b01: 1 cycle
            pub const Clocks1: u32 = 0b01;

            /// 0b10: 2 cycles
            pub const Clocks2: u32 = 0b10;

            /// 0b11: 3 cycles
            pub const Clocks3: u32 = 0b11;
        }
    }

    /// Write protection
    pub mod WP {
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

            /// 0b0: Write accesses allowed
            pub const Disabled: u32 = 0b0;

            /// 0b1: Write accesses ignored
            pub const Enabled: u32 = 0b1;
        }
    }

    /// SDRAM clock configuration
    pub mod SDCLK {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (2 bits: 0b11 << 10)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: SDCLK clock disabled
            pub const Disabled: u32 = 0b00;

            /// 0b10: SDCLK period = 2 x HCLK period
            pub const Div2: u32 = 0b10;

            /// 0b11: SDCLK period = 3 x HCLK period
            pub const Div3: u32 = 0b11;
        }
    }

    /// Burst read
    pub mod RBURST {
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

            /// 0b0: Single read requests are not managed as bursts
            pub const Disabled: u32 = 0b0;

            /// 0b1: Single read requests are always managed as bursts
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Read pipe
    pub mod RPIPE {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (2 bits: 0b11 << 13)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: No clock cycle delay
            pub const NoDelay: u32 = 0b00;

            /// 0b01: One clock cycle delay
            pub const Clocks1: u32 = 0b01;

            /// 0b10: Two clock cycles delay
            pub const Clocks2: u32 = 0b10;
        }
    }
}

/// SDRAM Control Register 1
pub mod SDCR2 {
    pub use super::SDCR1::CAS;
    pub use super::SDCR1::MWID;
    pub use super::SDCR1::NB;
    pub use super::SDCR1::NC;
    pub use super::SDCR1::NR;
    pub use super::SDCR1::RBURST;
    pub use super::SDCR1::RPIPE;
    pub use super::SDCR1::SDCLK;
    pub use super::SDCR1::WP;
}

/// SDRAM Timing register 1
pub mod SDTR1 {

    /// Load Mode Register to Active
    pub mod TMRD {
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

    /// Exit self-refresh delay
    pub mod TXSR {
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

    /// Self refresh time
    pub mod TRAS {
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

    /// Row cycle delay
    pub mod TRC {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (4 bits: 0b1111 << 12)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Recovery delay
    pub mod TWR {
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

    /// Row precharge delay
    pub mod TRP {
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

    /// Row to column delay
    pub mod TRCD {
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
}

/// SDRAM Timing register 1
pub mod SDTR2 {
    pub use super::SDTR1::TMRD;
    pub use super::SDTR1::TRAS;
    pub use super::SDTR1::TRC;
    pub use super::SDTR1::TRCD;
    pub use super::SDTR1::TRP;
    pub use super::SDTR1::TWR;
    pub use super::SDTR1::TXSR;
}

/// SDRAM Command Mode register
pub mod SDCMR {

    /// Command mode
    pub mod MODE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b000: Normal Mode
            pub const Normal: u32 = 0b000;

            /// 0b001: Clock Configuration Enable
            pub const ClockConfigurationEnable: u32 = 0b001;

            /// 0b010: PALL (All Bank Precharge) command
            pub const PALL: u32 = 0b010;

            /// 0b011: Auto-refresh command
            pub const AutoRefreshCommand: u32 = 0b011;

            /// 0b100: Load Mode Resgier
            pub const LoadModeRegister: u32 = 0b100;

            /// 0b101: Self-refresh command
            pub const SelfRefreshCommand: u32 = 0b101;

            /// 0b110: Power-down command
            pub const PowerDownCommand: u32 = 0b110;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Command target bank 2
    pub mod CTB2 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b0: Command not issued to SDRAM Bank 1
            pub const NotIssued: u32 = 0b0;

            /// 0b1: Command issued to SDRAM Bank 1
            pub const Issued: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Command target bank 1
    pub mod CTB1 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::CTB2::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Number of Auto-refresh
    pub mod NRFS {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (4 bits: 0b1111 << 5)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Mode Register definition
    pub mod MRD {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (13 bits: 0x1fff << 9)
        pub const mask: u32 = 0x1fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// SDRAM Refresh Timer register
pub mod SDRTR {

    /// Clear Refresh error flag
    pub mod CRE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Refresh Error Flag is cleared
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Refresh Timer Count
    pub mod COUNT {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (13 bits: 0x1fff << 1)
        pub const mask: u32 = 0x1fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// RES Interrupt Enable
    pub mod REIE {
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

            /// 0b0: Interrupt is disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Interrupt is generated if RE = 1
            pub const Enabled: u32 = 0b1;
        }
    }
}

/// SDRAM Status register
pub mod SDSR {

    /// Refresh error flag
    pub mod RE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No refresh error has been detected
            pub const NoError: u32 = 0b0;

            /// 0b1: A refresh error has been detected
            pub const Error: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Status Mode for Bank 1
    pub mod MODES1 {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (2 bits: 0b11 << 1)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values
        pub mod R {

            /// 0b00: Normal Mode
            pub const Normal: u32 = 0b00;

            /// 0b01: Self-refresh mode
            pub const SelfRefresh: u32 = 0b01;

            /// 0b10: Power-down mode
            pub const PowerDown: u32 = 0b10;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Status Mode for Bank 2
    pub mod MODES2 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (2 bits: 0b11 << 3)
        pub const mask: u32 = 0b11 << offset;
        pub use super::MODES1::R;
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Busy status
    pub mod BUSY {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: SDRAM Controller is ready to accept a new request
            pub const NotBusy: u32 = 0b0;

            /// 0b1: SDRAM Controller is not ready to accept a new request
            pub const Busy: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}
#[repr(C)]
pub struct RegisterBlock {
    /// SRAM/NOR-Flash chip-select control register 1
    pub BCR1: RWRegister<u32>,

    /// SRAM/NOR-Flash chip-select timing register 1
    pub BTR1: RWRegister<u32>,

    /// SRAM/NOR-Flash chip-select control register 2
    pub BCR2: RWRegister<u32>,

    /// SRAM/NOR-Flash chip-select timing register 1
    pub BTR2: RWRegister<u32>,

    /// SRAM/NOR-Flash chip-select control register 2
    pub BCR3: RWRegister<u32>,

    /// SRAM/NOR-Flash chip-select timing register 1
    pub BTR3: RWRegister<u32>,

    /// SRAM/NOR-Flash chip-select control register 2
    pub BCR4: RWRegister<u32>,

    /// SRAM/NOR-Flash chip-select timing register 1
    pub BTR4: RWRegister<u32>,

    _reserved1: [u8; 64],

    /// PC Card/NAND Flash control register 2
    pub PCR2: RWRegister<u32>,

    /// FIFO status and interrupt register 2
    pub SR2: RWRegister<u32>,

    /// Common memory space timing register 2
    pub PMEM2: RWRegister<u32>,

    /// Attribute memory space timing register 2
    pub PATT2: RWRegister<u32>,

    _reserved2: [u8; 4],

    /// ECC result register 2
    pub ECCR2: RORegister<u32>,

    _reserved3: [u8; 8],

    /// PC Card/NAND Flash control register 2
    pub PCR3: RWRegister<u32>,

    /// FIFO status and interrupt register 2
    pub SR3: RWRegister<u32>,

    /// Common memory space timing register 3
    pub PMEM3: RWRegister<u32>,

    /// Attribute memory space timing register 3
    pub PATT3: RWRegister<u32>,

    _reserved4: [u8; 4],

    /// ECC result register 3
    pub ECCR3: RORegister<u32>,

    _reserved5: [u8; 8],

    /// PC Card/NAND Flash control register 2
    pub PCR4: RWRegister<u32>,

    /// FIFO status and interrupt register 2
    pub SR4: RWRegister<u32>,

    /// Common memory space timing register 4
    pub PMEM4: RWRegister<u32>,

    /// Attribute memory space timing register 4
    pub PATT4: RWRegister<u32>,

    /// I/O space timing register 4
    pub PIO4: RWRegister<u32>,

    _reserved6: [u8; 80],

    /// SRAM/NOR-Flash write timing registers 1
    pub BWTR1: RWRegister<u32>,

    _reserved7: [u8; 4],

    /// SRAM/NOR-Flash write timing registers 1
    pub BWTR2: RWRegister<u32>,

    _reserved8: [u8; 4],

    /// SRAM/NOR-Flash write timing registers 1
    pub BWTR3: RWRegister<u32>,

    _reserved9: [u8; 4],

    /// SRAM/NOR-Flash write timing registers 1
    pub BWTR4: RWRegister<u32>,

    _reserved10: [u8; 32],

    /// SDRAM Control Register 1
    pub SDCR1: RWRegister<u32>,

    /// SDRAM Control Register 1
    pub SDCR2: RWRegister<u32>,

    /// SDRAM Timing register 1
    pub SDTR1: RWRegister<u32>,

    /// SDRAM Timing register 1
    pub SDTR2: RWRegister<u32>,

    /// SDRAM Command Mode register
    pub SDCMR: RWRegister<u32>,

    /// SDRAM Refresh Timer register
    pub SDRTR: RWRegister<u32>,

    /// SDRAM Status register
    pub SDSR: RORegister<u32>,
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
    pub PCR2: u32,
    pub SR2: u32,
    pub PMEM2: u32,
    pub PATT2: u32,
    pub ECCR2: u32,
    pub PCR3: u32,
    pub SR3: u32,
    pub PMEM3: u32,
    pub PATT3: u32,
    pub ECCR3: u32,
    pub PCR4: u32,
    pub SR4: u32,
    pub PMEM4: u32,
    pub PATT4: u32,
    pub PIO4: u32,
    pub BWTR1: u32,
    pub BWTR2: u32,
    pub BWTR3: u32,
    pub BWTR4: u32,
    pub SDCR1: u32,
    pub SDCR2: u32,
    pub SDTR1: u32,
    pub SDTR2: u32,
    pub SDCMR: u32,
    pub SDRTR: u32,
    pub SDSR: u32,
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

/// Access functions for the FMC peripheral instance
pub mod FMC {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0xa0000000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in FMC
    pub const reset: ResetValues = ResetValues {
        BCR1: 0x000030D0,
        BTR1: 0xFFFFFFFF,
        BTR2: 0xFFFFFFFF,
        BTR3: 0xFFFFFFFF,
        BTR4: 0xFFFFFFFF,
        BCR2: 0x000030D0,
        BCR3: 0x000030D0,
        BCR4: 0x000030D0,
        PCR2: 0x00000018,
        PCR3: 0x00000018,
        PCR4: 0x00000018,
        SR2: 0x00000040,
        SR3: 0x00000040,
        SR4: 0x00000040,
        PMEM2: 0xFCFCFCFC,
        PATT2: 0xFCFCFCFC,
        ECCR2: 0x00000000,
        PMEM3: 0xFCFCFCFC,
        PATT3: 0xFCFCFCFC,
        ECCR3: 0x00000000,
        PMEM4: 0xFCFCFCFC,
        PATT4: 0xFCFCFCFC,
        PIO4: 0xFCFCFCFC,
        BWTR1: 0x0FFFFFFF,
        BWTR2: 0x0FFFFFFF,
        BWTR3: 0x0FFFFFFF,
        BWTR4: 0x0FFFFFFF,
        SDCR1: 0x000002D0,
        SDCR2: 0x000002D0,
        SDTR1: 0x0FFFFFFF,
        SDTR2: 0x0FFFFFFF,
        SDCMR: 0x00000000,
        SDRTR: 0x00000000,
        SDSR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut FMC_TAKEN: bool = false;

    /// Safe access to FMC
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
            if FMC_TAKEN {
                None
            } else {
                FMC_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to FMC
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if FMC_TAKEN && inst.addr == INSTANCE.addr {
                FMC_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal FMC
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        FMC_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to FMC
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const FMC: *const RegisterBlock = 0xa0000000 as *const _;
