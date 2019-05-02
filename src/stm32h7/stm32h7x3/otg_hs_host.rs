#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! USB 1 on the go high speed

#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;
use {RORegister, RWRegister, UnsafeRWRegister};

/// OTG_HS host configuration register
pub mod HCFG {

    /// FS/LS PHY clock select
    pub mod FSLSPCS {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (2 bits: 0b11 << 0)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// FS- and LS-only support
    pub mod FSLSS {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// OTG_HS Host frame interval register
pub mod HFIR {

    /// Frame interval
    pub mod FRIVL {
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

/// OTG_HS host frame number/frame time remaining register
pub mod HFNUM {

    /// Frame number
    pub mod FRNUM {
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

    /// Frame time remaining
    pub mod FTREM {
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

/// OTG_HS_Host periodic transmit FIFO/queue status register
pub mod HPTXSTS {

    /// Periodic transmit data FIFO space available
    pub mod PTXFSAVL {
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

    /// Periodic transmit request queue space available
    pub mod PTXQSAV {
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

    /// Top of the periodic transmit request queue
    pub mod PTXQTOP {
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

/// OTG_HS Host all channels interrupt register
pub mod HAINT {

    /// Channel interrupts
    pub mod HAINT {
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

/// OTG_HS host all channels interrupt mask register
pub mod HAINTMSK {

    /// Channel interrupt mask
    pub mod HAINTM {
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

/// OTG_HS host port control and status register
pub mod HPRT {

    /// Port connect status
    pub mod PCSTS {
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

    /// Port connect detected
    pub mod PCDET {
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

    /// Port enable
    pub mod PENA {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Port enable/disable change
    pub mod PENCHNG {
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

    /// Port overcurrent active
    pub mod POCA {
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

    /// Port overcurrent change
    pub mod POCCHNG {
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

    /// Port resume
    pub mod PRES {
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

    /// Port suspend
    pub mod PSUSP {
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

    /// Port reset
    pub mod PRST {
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

    /// Port line status
    pub mod PLSTS {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (2 bits: 0b11 << 10)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Port power
    pub mod PPWR {
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

    /// Port test control
    pub mod PTCTL {
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

    /// Port speed
    pub mod PSPD {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (2 bits: 0b11 << 17)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// OTG_HS host channel-0 characteristics register
pub mod HCCHAR0 {

    /// Maximum packet size
    pub mod MPSIZ {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (11 bits: 0x7ff << 0)
        pub const mask: u32 = 0x7ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Endpoint number
    pub mod EPNUM {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (4 bits: 0b1111 << 11)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Endpoint direction
    pub mod EPDIR {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Low-speed device
    pub mod LSDEV {
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

    /// Endpoint type
    pub mod EPTYP {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (2 bits: 0b11 << 18)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Multi Count (MC) / Error Count (EC)
    pub mod MC {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (2 bits: 0b11 << 20)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Device address
    pub mod DAD {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (7 bits: 0x7f << 22)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Odd frame
    pub mod ODDFRM {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Channel disable
    pub mod CHDIS {
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

    /// Channel enable
    pub mod CHENA {
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

/// OTG_HS host channel-1 characteristics register
pub mod HCCHAR1 {
    pub use super::HCCHAR0::CHDIS;
    pub use super::HCCHAR0::CHENA;
    pub use super::HCCHAR0::DAD;
    pub use super::HCCHAR0::EPDIR;
    pub use super::HCCHAR0::EPNUM;
    pub use super::HCCHAR0::EPTYP;
    pub use super::HCCHAR0::LSDEV;
    pub use super::HCCHAR0::MC;
    pub use super::HCCHAR0::MPSIZ;
    pub use super::HCCHAR0::ODDFRM;
}

/// OTG_HS host channel-2 characteristics register
pub mod HCCHAR2 {
    pub use super::HCCHAR0::CHDIS;
    pub use super::HCCHAR0::CHENA;
    pub use super::HCCHAR0::DAD;
    pub use super::HCCHAR0::EPDIR;
    pub use super::HCCHAR0::EPNUM;
    pub use super::HCCHAR0::EPTYP;
    pub use super::HCCHAR0::LSDEV;
    pub use super::HCCHAR0::MC;
    pub use super::HCCHAR0::MPSIZ;
    pub use super::HCCHAR0::ODDFRM;
}

/// OTG_HS host channel-3 characteristics register
pub mod HCCHAR3 {
    pub use super::HCCHAR0::CHDIS;
    pub use super::HCCHAR0::CHENA;
    pub use super::HCCHAR0::DAD;
    pub use super::HCCHAR0::EPDIR;
    pub use super::HCCHAR0::EPNUM;
    pub use super::HCCHAR0::EPTYP;
    pub use super::HCCHAR0::LSDEV;
    pub use super::HCCHAR0::MC;
    pub use super::HCCHAR0::MPSIZ;
    pub use super::HCCHAR0::ODDFRM;
}

/// OTG_HS host channel-4 characteristics register
pub mod HCCHAR4 {
    pub use super::HCCHAR0::CHDIS;
    pub use super::HCCHAR0::CHENA;
    pub use super::HCCHAR0::DAD;
    pub use super::HCCHAR0::EPDIR;
    pub use super::HCCHAR0::EPNUM;
    pub use super::HCCHAR0::EPTYP;
    pub use super::HCCHAR0::LSDEV;
    pub use super::HCCHAR0::MC;
    pub use super::HCCHAR0::MPSIZ;
    pub use super::HCCHAR0::ODDFRM;
}

/// OTG_HS host channel-5 characteristics register
pub mod HCCHAR5 {
    pub use super::HCCHAR0::CHDIS;
    pub use super::HCCHAR0::CHENA;
    pub use super::HCCHAR0::DAD;
    pub use super::HCCHAR0::EPDIR;
    pub use super::HCCHAR0::EPNUM;
    pub use super::HCCHAR0::EPTYP;
    pub use super::HCCHAR0::LSDEV;
    pub use super::HCCHAR0::MC;
    pub use super::HCCHAR0::MPSIZ;
    pub use super::HCCHAR0::ODDFRM;
}

/// OTG_HS host channel-6 characteristics register
pub mod HCCHAR6 {
    pub use super::HCCHAR0::CHDIS;
    pub use super::HCCHAR0::CHENA;
    pub use super::HCCHAR0::DAD;
    pub use super::HCCHAR0::EPDIR;
    pub use super::HCCHAR0::EPNUM;
    pub use super::HCCHAR0::EPTYP;
    pub use super::HCCHAR0::LSDEV;
    pub use super::HCCHAR0::MC;
    pub use super::HCCHAR0::MPSIZ;
    pub use super::HCCHAR0::ODDFRM;
}

/// OTG_HS host channel-7 characteristics register
pub mod HCCHAR7 {
    pub use super::HCCHAR0::CHDIS;
    pub use super::HCCHAR0::CHENA;
    pub use super::HCCHAR0::DAD;
    pub use super::HCCHAR0::EPDIR;
    pub use super::HCCHAR0::EPNUM;
    pub use super::HCCHAR0::EPTYP;
    pub use super::HCCHAR0::LSDEV;
    pub use super::HCCHAR0::MC;
    pub use super::HCCHAR0::MPSIZ;
    pub use super::HCCHAR0::ODDFRM;
}

/// OTG_HS host channel-8 characteristics register
pub mod HCCHAR8 {
    pub use super::HCCHAR0::CHDIS;
    pub use super::HCCHAR0::CHENA;
    pub use super::HCCHAR0::DAD;
    pub use super::HCCHAR0::EPDIR;
    pub use super::HCCHAR0::EPNUM;
    pub use super::HCCHAR0::EPTYP;
    pub use super::HCCHAR0::LSDEV;
    pub use super::HCCHAR0::MC;
    pub use super::HCCHAR0::MPSIZ;
    pub use super::HCCHAR0::ODDFRM;
}

/// OTG_HS host channel-9 characteristics register
pub mod HCCHAR9 {
    pub use super::HCCHAR0::CHDIS;
    pub use super::HCCHAR0::CHENA;
    pub use super::HCCHAR0::DAD;
    pub use super::HCCHAR0::EPDIR;
    pub use super::HCCHAR0::EPNUM;
    pub use super::HCCHAR0::EPTYP;
    pub use super::HCCHAR0::LSDEV;
    pub use super::HCCHAR0::MC;
    pub use super::HCCHAR0::MPSIZ;
    pub use super::HCCHAR0::ODDFRM;
}

/// OTG_HS host channel-10 characteristics register
pub mod HCCHAR10 {
    pub use super::HCCHAR0::CHDIS;
    pub use super::HCCHAR0::CHENA;
    pub use super::HCCHAR0::DAD;
    pub use super::HCCHAR0::EPDIR;
    pub use super::HCCHAR0::EPNUM;
    pub use super::HCCHAR0::EPTYP;
    pub use super::HCCHAR0::LSDEV;
    pub use super::HCCHAR0::MC;
    pub use super::HCCHAR0::MPSIZ;
    pub use super::HCCHAR0::ODDFRM;
}

/// OTG_HS host channel-11 characteristics register
pub mod HCCHAR11 {
    pub use super::HCCHAR0::CHDIS;
    pub use super::HCCHAR0::CHENA;
    pub use super::HCCHAR0::DAD;
    pub use super::HCCHAR0::EPDIR;
    pub use super::HCCHAR0::EPNUM;
    pub use super::HCCHAR0::EPTYP;
    pub use super::HCCHAR0::LSDEV;
    pub use super::HCCHAR0::MC;
    pub use super::HCCHAR0::MPSIZ;
    pub use super::HCCHAR0::ODDFRM;
}

/// OTG_HS host channel-0 split control register
pub mod HCSPLT0 {

    /// Port address
    pub mod PRTADDR {
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

    /// Hub address
    pub mod HUBADDR {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (7 bits: 0x7f << 7)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// XACTPOS
    pub mod XACTPOS {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (2 bits: 0b11 << 14)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Do complete split
    pub mod COMPLSPLT {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Split enable
    pub mod SPLITEN {
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

/// OTG_HS host channel-1 split control register
pub mod HCSPLT1 {
    pub use super::HCSPLT0::COMPLSPLT;
    pub use super::HCSPLT0::HUBADDR;
    pub use super::HCSPLT0::PRTADDR;
    pub use super::HCSPLT0::SPLITEN;
    pub use super::HCSPLT0::XACTPOS;
}

/// OTG_HS host channel-2 split control register
pub mod HCSPLT2 {
    pub use super::HCSPLT0::COMPLSPLT;
    pub use super::HCSPLT0::HUBADDR;
    pub use super::HCSPLT0::PRTADDR;
    pub use super::HCSPLT0::SPLITEN;
    pub use super::HCSPLT0::XACTPOS;
}

/// OTG_HS host channel-3 split control register
pub mod HCSPLT3 {
    pub use super::HCSPLT0::COMPLSPLT;
    pub use super::HCSPLT0::HUBADDR;
    pub use super::HCSPLT0::PRTADDR;
    pub use super::HCSPLT0::SPLITEN;
    pub use super::HCSPLT0::XACTPOS;
}

/// OTG_HS host channel-4 split control register
pub mod HCSPLT4 {
    pub use super::HCSPLT0::COMPLSPLT;
    pub use super::HCSPLT0::HUBADDR;
    pub use super::HCSPLT0::PRTADDR;
    pub use super::HCSPLT0::SPLITEN;
    pub use super::HCSPLT0::XACTPOS;
}

/// OTG_HS host channel-5 split control register
pub mod HCSPLT5 {
    pub use super::HCSPLT0::COMPLSPLT;
    pub use super::HCSPLT0::HUBADDR;
    pub use super::HCSPLT0::PRTADDR;
    pub use super::HCSPLT0::SPLITEN;
    pub use super::HCSPLT0::XACTPOS;
}

/// OTG_HS host channel-6 split control register
pub mod HCSPLT6 {
    pub use super::HCSPLT0::COMPLSPLT;
    pub use super::HCSPLT0::HUBADDR;
    pub use super::HCSPLT0::PRTADDR;
    pub use super::HCSPLT0::SPLITEN;
    pub use super::HCSPLT0::XACTPOS;
}

/// OTG_HS host channel-7 split control register
pub mod HCSPLT7 {
    pub use super::HCSPLT0::COMPLSPLT;
    pub use super::HCSPLT0::HUBADDR;
    pub use super::HCSPLT0::PRTADDR;
    pub use super::HCSPLT0::SPLITEN;
    pub use super::HCSPLT0::XACTPOS;
}

/// OTG_HS host channel-8 split control register
pub mod HCSPLT8 {
    pub use super::HCSPLT0::COMPLSPLT;
    pub use super::HCSPLT0::HUBADDR;
    pub use super::HCSPLT0::PRTADDR;
    pub use super::HCSPLT0::SPLITEN;
    pub use super::HCSPLT0::XACTPOS;
}

/// OTG_HS host channel-9 split control register
pub mod HCSPLT9 {
    pub use super::HCSPLT0::COMPLSPLT;
    pub use super::HCSPLT0::HUBADDR;
    pub use super::HCSPLT0::PRTADDR;
    pub use super::HCSPLT0::SPLITEN;
    pub use super::HCSPLT0::XACTPOS;
}

/// OTG_HS host channel-10 split control register
pub mod HCSPLT10 {
    pub use super::HCSPLT0::COMPLSPLT;
    pub use super::HCSPLT0::HUBADDR;
    pub use super::HCSPLT0::PRTADDR;
    pub use super::HCSPLT0::SPLITEN;
    pub use super::HCSPLT0::XACTPOS;
}

/// OTG_HS host channel-11 split control register
pub mod HCSPLT11 {
    pub use super::HCSPLT0::COMPLSPLT;
    pub use super::HCSPLT0::HUBADDR;
    pub use super::HCSPLT0::PRTADDR;
    pub use super::HCSPLT0::SPLITEN;
    pub use super::HCSPLT0::XACTPOS;
}

/// OTG_HS host channel-11 interrupt register
pub mod HCINT0 {

    /// Transfer completed
    pub mod XFRC {
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

    /// Channel halted
    pub mod CHH {
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

    /// AHB error
    pub mod AHBERR {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// STALL response received interrupt
    pub mod STALL {
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

    /// NAK response received interrupt
    pub mod NAK {
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

    /// ACK response received/transmitted interrupt
    pub mod ACK {
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

    /// Response received interrupt
    pub mod NYET {
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

    /// Transaction error
    pub mod TXERR {
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

    /// Babble error
    pub mod BBERR {
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

    /// Frame overrun
    pub mod FRMOR {
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

    /// Data toggle error
    pub mod DTERR {
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
}

/// OTG_HS host channel-1 interrupt register
pub mod HCINT1 {
    pub use super::HCINT0::ACK;
    pub use super::HCINT0::AHBERR;
    pub use super::HCINT0::BBERR;
    pub use super::HCINT0::CHH;
    pub use super::HCINT0::DTERR;
    pub use super::HCINT0::FRMOR;
    pub use super::HCINT0::NAK;
    pub use super::HCINT0::NYET;
    pub use super::HCINT0::STALL;
    pub use super::HCINT0::TXERR;
    pub use super::HCINT0::XFRC;
}

/// OTG_HS host channel-2 interrupt register
pub mod HCINT2 {
    pub use super::HCINT0::ACK;
    pub use super::HCINT0::AHBERR;
    pub use super::HCINT0::BBERR;
    pub use super::HCINT0::CHH;
    pub use super::HCINT0::DTERR;
    pub use super::HCINT0::FRMOR;
    pub use super::HCINT0::NAK;
    pub use super::HCINT0::NYET;
    pub use super::HCINT0::STALL;
    pub use super::HCINT0::TXERR;
    pub use super::HCINT0::XFRC;
}

/// OTG_HS host channel-3 interrupt register
pub mod HCINT3 {
    pub use super::HCINT0::ACK;
    pub use super::HCINT0::AHBERR;
    pub use super::HCINT0::BBERR;
    pub use super::HCINT0::CHH;
    pub use super::HCINT0::DTERR;
    pub use super::HCINT0::FRMOR;
    pub use super::HCINT0::NAK;
    pub use super::HCINT0::NYET;
    pub use super::HCINT0::STALL;
    pub use super::HCINT0::TXERR;
    pub use super::HCINT0::XFRC;
}

/// OTG_HS host channel-4 interrupt register
pub mod HCINT4 {
    pub use super::HCINT0::ACK;
    pub use super::HCINT0::AHBERR;
    pub use super::HCINT0::BBERR;
    pub use super::HCINT0::CHH;
    pub use super::HCINT0::DTERR;
    pub use super::HCINT0::FRMOR;
    pub use super::HCINT0::NAK;
    pub use super::HCINT0::NYET;
    pub use super::HCINT0::STALL;
    pub use super::HCINT0::TXERR;
    pub use super::HCINT0::XFRC;
}

/// OTG_HS host channel-5 interrupt register
pub mod HCINT5 {
    pub use super::HCINT0::ACK;
    pub use super::HCINT0::AHBERR;
    pub use super::HCINT0::BBERR;
    pub use super::HCINT0::CHH;
    pub use super::HCINT0::DTERR;
    pub use super::HCINT0::FRMOR;
    pub use super::HCINT0::NAK;
    pub use super::HCINT0::NYET;
    pub use super::HCINT0::STALL;
    pub use super::HCINT0::TXERR;
    pub use super::HCINT0::XFRC;
}

/// OTG_HS host channel-6 interrupt register
pub mod HCINT6 {
    pub use super::HCINT0::ACK;
    pub use super::HCINT0::AHBERR;
    pub use super::HCINT0::BBERR;
    pub use super::HCINT0::CHH;
    pub use super::HCINT0::DTERR;
    pub use super::HCINT0::FRMOR;
    pub use super::HCINT0::NAK;
    pub use super::HCINT0::NYET;
    pub use super::HCINT0::STALL;
    pub use super::HCINT0::TXERR;
    pub use super::HCINT0::XFRC;
}

/// OTG_HS host channel-7 interrupt register
pub mod HCINT7 {
    pub use super::HCINT0::ACK;
    pub use super::HCINT0::AHBERR;
    pub use super::HCINT0::BBERR;
    pub use super::HCINT0::CHH;
    pub use super::HCINT0::DTERR;
    pub use super::HCINT0::FRMOR;
    pub use super::HCINT0::NAK;
    pub use super::HCINT0::NYET;
    pub use super::HCINT0::STALL;
    pub use super::HCINT0::TXERR;
    pub use super::HCINT0::XFRC;
}

/// OTG_HS host channel-8 interrupt register
pub mod HCINT8 {
    pub use super::HCINT0::ACK;
    pub use super::HCINT0::AHBERR;
    pub use super::HCINT0::BBERR;
    pub use super::HCINT0::CHH;
    pub use super::HCINT0::DTERR;
    pub use super::HCINT0::FRMOR;
    pub use super::HCINT0::NAK;
    pub use super::HCINT0::NYET;
    pub use super::HCINT0::STALL;
    pub use super::HCINT0::TXERR;
    pub use super::HCINT0::XFRC;
}

/// OTG_HS host channel-9 interrupt register
pub mod HCINT9 {
    pub use super::HCINT0::ACK;
    pub use super::HCINT0::AHBERR;
    pub use super::HCINT0::BBERR;
    pub use super::HCINT0::CHH;
    pub use super::HCINT0::DTERR;
    pub use super::HCINT0::FRMOR;
    pub use super::HCINT0::NAK;
    pub use super::HCINT0::NYET;
    pub use super::HCINT0::STALL;
    pub use super::HCINT0::TXERR;
    pub use super::HCINT0::XFRC;
}

/// OTG_HS host channel-10 interrupt register
pub mod HCINT10 {
    pub use super::HCINT0::ACK;
    pub use super::HCINT0::AHBERR;
    pub use super::HCINT0::BBERR;
    pub use super::HCINT0::CHH;
    pub use super::HCINT0::DTERR;
    pub use super::HCINT0::FRMOR;
    pub use super::HCINT0::NAK;
    pub use super::HCINT0::NYET;
    pub use super::HCINT0::STALL;
    pub use super::HCINT0::TXERR;
    pub use super::HCINT0::XFRC;
}

/// OTG_HS host channel-11 interrupt register
pub mod HCINT11 {
    pub use super::HCINT0::ACK;
    pub use super::HCINT0::AHBERR;
    pub use super::HCINT0::BBERR;
    pub use super::HCINT0::CHH;
    pub use super::HCINT0::DTERR;
    pub use super::HCINT0::FRMOR;
    pub use super::HCINT0::NAK;
    pub use super::HCINT0::NYET;
    pub use super::HCINT0::STALL;
    pub use super::HCINT0::TXERR;
    pub use super::HCINT0::XFRC;
}

/// OTG_HS host channel-11 interrupt mask register
pub mod HCINTMSK0 {

    /// Transfer completed mask
    pub mod XFRCM {
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

    /// Channel halted mask
    pub mod CHHM {
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

    /// AHB error
    pub mod AHBERR {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// STALL response received interrupt mask
    pub mod STALLM {
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

    /// NAK response received interrupt mask
    pub mod NAKM {
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

    /// ACK response received/transmitted interrupt mask
    pub mod ACKM {
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

    /// response received interrupt mask
    pub mod NYET {
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

    /// Transaction error mask
    pub mod TXERRM {
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

    /// Babble error mask
    pub mod BBERRM {
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

    /// Frame overrun mask
    pub mod FRMORM {
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

    /// Data toggle error mask
    pub mod DTERRM {
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
}

/// OTG_HS host channel-1 interrupt mask register
pub mod HCINTMSK1 {
    pub use super::HCINTMSK0::ACKM;
    pub use super::HCINTMSK0::AHBERR;
    pub use super::HCINTMSK0::BBERRM;
    pub use super::HCINTMSK0::CHHM;
    pub use super::HCINTMSK0::DTERRM;
    pub use super::HCINTMSK0::FRMORM;
    pub use super::HCINTMSK0::NAKM;
    pub use super::HCINTMSK0::NYET;
    pub use super::HCINTMSK0::STALLM;
    pub use super::HCINTMSK0::TXERRM;
    pub use super::HCINTMSK0::XFRCM;
}

/// OTG_HS host channel-2 interrupt mask register
pub mod HCINTMSK2 {
    pub use super::HCINTMSK0::ACKM;
    pub use super::HCINTMSK0::AHBERR;
    pub use super::HCINTMSK0::BBERRM;
    pub use super::HCINTMSK0::CHHM;
    pub use super::HCINTMSK0::DTERRM;
    pub use super::HCINTMSK0::FRMORM;
    pub use super::HCINTMSK0::NAKM;
    pub use super::HCINTMSK0::NYET;
    pub use super::HCINTMSK0::STALLM;
    pub use super::HCINTMSK0::TXERRM;
    pub use super::HCINTMSK0::XFRCM;
}

/// OTG_HS host channel-3 interrupt mask register
pub mod HCINTMSK3 {
    pub use super::HCINTMSK0::ACKM;
    pub use super::HCINTMSK0::AHBERR;
    pub use super::HCINTMSK0::BBERRM;
    pub use super::HCINTMSK0::CHHM;
    pub use super::HCINTMSK0::DTERRM;
    pub use super::HCINTMSK0::FRMORM;
    pub use super::HCINTMSK0::NAKM;
    pub use super::HCINTMSK0::NYET;
    pub use super::HCINTMSK0::STALLM;
    pub use super::HCINTMSK0::TXERRM;
    pub use super::HCINTMSK0::XFRCM;
}

/// OTG_HS host channel-4 interrupt mask register
pub mod HCINTMSK4 {
    pub use super::HCINTMSK0::ACKM;
    pub use super::HCINTMSK0::AHBERR;
    pub use super::HCINTMSK0::BBERRM;
    pub use super::HCINTMSK0::CHHM;
    pub use super::HCINTMSK0::DTERRM;
    pub use super::HCINTMSK0::FRMORM;
    pub use super::HCINTMSK0::NAKM;
    pub use super::HCINTMSK0::NYET;
    pub use super::HCINTMSK0::STALLM;
    pub use super::HCINTMSK0::TXERRM;
    pub use super::HCINTMSK0::XFRCM;
}

/// OTG_HS host channel-5 interrupt mask register
pub mod HCINTMSK5 {
    pub use super::HCINTMSK0::ACKM;
    pub use super::HCINTMSK0::AHBERR;
    pub use super::HCINTMSK0::BBERRM;
    pub use super::HCINTMSK0::CHHM;
    pub use super::HCINTMSK0::DTERRM;
    pub use super::HCINTMSK0::FRMORM;
    pub use super::HCINTMSK0::NAKM;
    pub use super::HCINTMSK0::NYET;
    pub use super::HCINTMSK0::STALLM;
    pub use super::HCINTMSK0::TXERRM;
    pub use super::HCINTMSK0::XFRCM;
}

/// OTG_HS host channel-6 interrupt mask register
pub mod HCINTMSK6 {
    pub use super::HCINTMSK0::ACKM;
    pub use super::HCINTMSK0::AHBERR;
    pub use super::HCINTMSK0::BBERRM;
    pub use super::HCINTMSK0::CHHM;
    pub use super::HCINTMSK0::DTERRM;
    pub use super::HCINTMSK0::FRMORM;
    pub use super::HCINTMSK0::NAKM;
    pub use super::HCINTMSK0::NYET;
    pub use super::HCINTMSK0::STALLM;
    pub use super::HCINTMSK0::TXERRM;
    pub use super::HCINTMSK0::XFRCM;
}

/// OTG_HS host channel-7 interrupt mask register
pub mod HCINTMSK7 {
    pub use super::HCINTMSK0::ACKM;
    pub use super::HCINTMSK0::AHBERR;
    pub use super::HCINTMSK0::BBERRM;
    pub use super::HCINTMSK0::CHHM;
    pub use super::HCINTMSK0::DTERRM;
    pub use super::HCINTMSK0::FRMORM;
    pub use super::HCINTMSK0::NAKM;
    pub use super::HCINTMSK0::NYET;
    pub use super::HCINTMSK0::STALLM;
    pub use super::HCINTMSK0::TXERRM;
    pub use super::HCINTMSK0::XFRCM;
}

/// OTG_HS host channel-8 interrupt mask register
pub mod HCINTMSK8 {
    pub use super::HCINTMSK0::ACKM;
    pub use super::HCINTMSK0::AHBERR;
    pub use super::HCINTMSK0::BBERRM;
    pub use super::HCINTMSK0::CHHM;
    pub use super::HCINTMSK0::DTERRM;
    pub use super::HCINTMSK0::FRMORM;
    pub use super::HCINTMSK0::NAKM;
    pub use super::HCINTMSK0::NYET;
    pub use super::HCINTMSK0::STALLM;
    pub use super::HCINTMSK0::TXERRM;
    pub use super::HCINTMSK0::XFRCM;
}

/// OTG_HS host channel-9 interrupt mask register
pub mod HCINTMSK9 {
    pub use super::HCINTMSK0::ACKM;
    pub use super::HCINTMSK0::AHBERR;
    pub use super::HCINTMSK0::BBERRM;
    pub use super::HCINTMSK0::CHHM;
    pub use super::HCINTMSK0::DTERRM;
    pub use super::HCINTMSK0::FRMORM;
    pub use super::HCINTMSK0::NAKM;
    pub use super::HCINTMSK0::NYET;
    pub use super::HCINTMSK0::STALLM;
    pub use super::HCINTMSK0::TXERRM;
    pub use super::HCINTMSK0::XFRCM;
}

/// OTG_HS host channel-10 interrupt mask register
pub mod HCINTMSK10 {
    pub use super::HCINTMSK0::ACKM;
    pub use super::HCINTMSK0::AHBERR;
    pub use super::HCINTMSK0::BBERRM;
    pub use super::HCINTMSK0::CHHM;
    pub use super::HCINTMSK0::DTERRM;
    pub use super::HCINTMSK0::FRMORM;
    pub use super::HCINTMSK0::NAKM;
    pub use super::HCINTMSK0::NYET;
    pub use super::HCINTMSK0::STALLM;
    pub use super::HCINTMSK0::TXERRM;
    pub use super::HCINTMSK0::XFRCM;
}

/// OTG_HS host channel-11 interrupt mask register
pub mod HCINTMSK11 {
    pub use super::HCINTMSK0::ACKM;
    pub use super::HCINTMSK0::AHBERR;
    pub use super::HCINTMSK0::BBERRM;
    pub use super::HCINTMSK0::CHHM;
    pub use super::HCINTMSK0::DTERRM;
    pub use super::HCINTMSK0::FRMORM;
    pub use super::HCINTMSK0::NAKM;
    pub use super::HCINTMSK0::NYET;
    pub use super::HCINTMSK0::STALLM;
    pub use super::HCINTMSK0::TXERRM;
    pub use super::HCINTMSK0::XFRCM;
}

/// OTG_HS host channel-11 transfer size register
pub mod HCTSIZ0 {

    /// Transfer size
    pub mod XFRSIZ {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (19 bits: 0x7ffff << 0)
        pub const mask: u32 = 0x7ffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Packet count
    pub mod PKTCNT {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (10 bits: 0x3ff << 19)
        pub const mask: u32 = 0x3ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Data PID
    pub mod DPID {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (2 bits: 0b11 << 29)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// OTG_HS host channel-1 transfer size register
pub mod HCTSIZ1 {
    pub use super::HCTSIZ0::DPID;
    pub use super::HCTSIZ0::PKTCNT;
    pub use super::HCTSIZ0::XFRSIZ;
}

/// OTG_HS host channel-2 transfer size register
pub mod HCTSIZ2 {
    pub use super::HCTSIZ0::DPID;
    pub use super::HCTSIZ0::PKTCNT;
    pub use super::HCTSIZ0::XFRSIZ;
}

/// OTG_HS host channel-3 transfer size register
pub mod HCTSIZ3 {
    pub use super::HCTSIZ0::DPID;
    pub use super::HCTSIZ0::PKTCNT;
    pub use super::HCTSIZ0::XFRSIZ;
}

/// OTG_HS host channel-4 transfer size register
pub mod HCTSIZ4 {
    pub use super::HCTSIZ0::DPID;
    pub use super::HCTSIZ0::PKTCNT;
    pub use super::HCTSIZ0::XFRSIZ;
}

/// OTG_HS host channel-5 transfer size register
pub mod HCTSIZ5 {
    pub use super::HCTSIZ0::DPID;
    pub use super::HCTSIZ0::PKTCNT;
    pub use super::HCTSIZ0::XFRSIZ;
}

/// OTG_HS host channel-6 transfer size register
pub mod HCTSIZ6 {
    pub use super::HCTSIZ0::DPID;
    pub use super::HCTSIZ0::PKTCNT;
    pub use super::HCTSIZ0::XFRSIZ;
}

/// OTG_HS host channel-7 transfer size register
pub mod HCTSIZ7 {
    pub use super::HCTSIZ0::DPID;
    pub use super::HCTSIZ0::PKTCNT;
    pub use super::HCTSIZ0::XFRSIZ;
}

/// OTG_HS host channel-8 transfer size register
pub mod HCTSIZ8 {
    pub use super::HCTSIZ0::DPID;
    pub use super::HCTSIZ0::PKTCNT;
    pub use super::HCTSIZ0::XFRSIZ;
}

/// OTG_HS host channel-9 transfer size register
pub mod HCTSIZ9 {
    pub use super::HCTSIZ0::DPID;
    pub use super::HCTSIZ0::PKTCNT;
    pub use super::HCTSIZ0::XFRSIZ;
}

/// OTG_HS host channel-10 transfer size register
pub mod HCTSIZ10 {
    pub use super::HCTSIZ0::DPID;
    pub use super::HCTSIZ0::PKTCNT;
    pub use super::HCTSIZ0::XFRSIZ;
}

/// OTG_HS host channel-11 transfer size register
pub mod HCTSIZ11 {
    pub use super::HCTSIZ0::DPID;
    pub use super::HCTSIZ0::PKTCNT;
    pub use super::HCTSIZ0::XFRSIZ;
}

/// OTG_HS host channel-0 DMA address register
pub mod HCDMA0 {

    /// DMA address
    pub mod DMAADDR {
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

/// OTG_HS host channel-1 DMA address register
pub mod HCDMA1 {
    pub use super::HCDMA0::DMAADDR;
}

/// OTG_HS host channel-2 DMA address register
pub mod HCDMA2 {
    pub use super::HCDMA0::DMAADDR;
}

/// OTG_HS host channel-3 DMA address register
pub mod HCDMA3 {
    pub use super::HCDMA0::DMAADDR;
}

/// OTG_HS host channel-4 DMA address register
pub mod HCDMA4 {
    pub use super::HCDMA0::DMAADDR;
}

/// OTG_HS host channel-5 DMA address register
pub mod HCDMA5 {
    pub use super::HCDMA0::DMAADDR;
}

/// OTG_HS host channel-6 DMA address register
pub mod HCDMA6 {
    pub use super::HCDMA0::DMAADDR;
}

/// OTG_HS host channel-7 DMA address register
pub mod HCDMA7 {
    pub use super::HCDMA0::DMAADDR;
}

/// OTG_HS host channel-8 DMA address register
pub mod HCDMA8 {
    pub use super::HCDMA0::DMAADDR;
}

/// OTG_HS host channel-9 DMA address register
pub mod HCDMA9 {
    pub use super::HCDMA0::DMAADDR;
}

/// OTG_HS host channel-10 DMA address register
pub mod HCDMA10 {
    pub use super::HCDMA0::DMAADDR;
}

/// OTG_HS host channel-11 DMA address register
pub mod HCDMA11 {
    pub use super::HCDMA0::DMAADDR;
}

/// OTG_HS host channel-12 characteristics register
pub mod HCCHAR12 {
    pub use super::HCCHAR0::CHDIS;
    pub use super::HCCHAR0::CHENA;
    pub use super::HCCHAR0::DAD;
    pub use super::HCCHAR0::EPDIR;
    pub use super::HCCHAR0::EPNUM;
    pub use super::HCCHAR0::EPTYP;
    pub use super::HCCHAR0::LSDEV;
    pub use super::HCCHAR0::MC;
    pub use super::HCCHAR0::MPSIZ;
    pub use super::HCCHAR0::ODDFRM;
}

/// OTG_HS host channel-12 split control register
pub mod HCSPLT12 {
    pub use super::HCSPLT0::COMPLSPLT;
    pub use super::HCSPLT0::HUBADDR;
    pub use super::HCSPLT0::PRTADDR;
    pub use super::HCSPLT0::SPLITEN;
    pub use super::HCSPLT0::XACTPOS;
}

/// OTG_HS host channel-12 interrupt register
pub mod HCINT12 {
    pub use super::HCINT0::ACK;
    pub use super::HCINT0::AHBERR;
    pub use super::HCINT0::BBERR;
    pub use super::HCINT0::CHH;
    pub use super::HCINT0::DTERR;
    pub use super::HCINT0::FRMOR;
    pub use super::HCINT0::NAK;
    pub use super::HCINT0::NYET;
    pub use super::HCINT0::STALL;
    pub use super::HCINT0::TXERR;
    pub use super::HCINT0::XFRC;
}

/// OTG_HS host channel-12 interrupt mask register
pub mod HCINTMSK12 {
    pub use super::HCINTMSK0::ACKM;
    pub use super::HCINTMSK0::AHBERR;
    pub use super::HCINTMSK0::BBERRM;
    pub use super::HCINTMSK0::CHHM;
    pub use super::HCINTMSK0::DTERRM;
    pub use super::HCINTMSK0::FRMORM;
    pub use super::HCINTMSK0::NAKM;
    pub use super::HCINTMSK0::NYET;
    pub use super::HCINTMSK0::STALLM;
    pub use super::HCINTMSK0::TXERRM;
    pub use super::HCINTMSK0::XFRCM;
}

/// OTG_HS host channel-12 transfer size register
pub mod HCTSIZ12 {
    pub use super::HCTSIZ0::DPID;
    pub use super::HCTSIZ0::PKTCNT;
    pub use super::HCTSIZ0::XFRSIZ;
}

/// OTG_HS host channel-12 DMA address register
pub mod HCDMA12 {
    pub use super::HCDMA0::DMAADDR;
}

/// OTG_HS host channel-13 characteristics register
pub mod HCCHAR13 {
    pub use super::HCCHAR0::CHDIS;
    pub use super::HCCHAR0::CHENA;
    pub use super::HCCHAR0::DAD;
    pub use super::HCCHAR0::EPDIR;
    pub use super::HCCHAR0::EPNUM;
    pub use super::HCCHAR0::EPTYP;
    pub use super::HCCHAR0::LSDEV;
    pub use super::HCCHAR0::MC;
    pub use super::HCCHAR0::MPSIZ;
    pub use super::HCCHAR0::ODDFRM;
}

/// OTG_HS host channel-13 split control register
pub mod HCSPLT13 {
    pub use super::HCSPLT0::COMPLSPLT;
    pub use super::HCSPLT0::HUBADDR;
    pub use super::HCSPLT0::PRTADDR;
    pub use super::HCSPLT0::SPLITEN;
    pub use super::HCSPLT0::XACTPOS;
}

/// OTG_HS host channel-13 interrupt register
pub mod HCINT13 {
    pub use super::HCINT0::ACK;
    pub use super::HCINT0::AHBERR;
    pub use super::HCINT0::BBERR;
    pub use super::HCINT0::CHH;
    pub use super::HCINT0::DTERR;
    pub use super::HCINT0::FRMOR;
    pub use super::HCINT0::NAK;
    pub use super::HCINT0::NYET;
    pub use super::HCINT0::STALL;
    pub use super::HCINT0::TXERR;
    pub use super::HCINT0::XFRC;
}

/// OTG_HS host channel-13 interrupt mask register
pub mod HCINTMSK13 {
    pub use super::HCINTMSK0::ACKM;
    pub use super::HCINTMSK0::AHBERR;
    pub use super::HCINTMSK0::BBERRM;
    pub use super::HCINTMSK0::CHHM;
    pub use super::HCINTMSK0::DTERRM;
    pub use super::HCINTMSK0::FRMORM;
    pub use super::HCINTMSK0::NAKM;
    pub use super::HCINTMSK0::NYET;
    pub use super::HCINTMSK0::STALLM;
    pub use super::HCINTMSK0::TXERRM;
    pub use super::HCINTMSK0::XFRCM;
}

/// OTG_HS host channel-13 transfer size register
pub mod HCTSIZ13 {
    pub use super::HCTSIZ0::DPID;
    pub use super::HCTSIZ0::PKTCNT;
    pub use super::HCTSIZ0::XFRSIZ;
}

/// OTG_HS host channel-13 DMA address register
pub mod HCDMA13 {
    pub use super::HCDMA0::DMAADDR;
}

/// OTG_HS host channel-14 characteristics register
pub mod HCCHAR14 {
    pub use super::HCCHAR0::CHDIS;
    pub use super::HCCHAR0::CHENA;
    pub use super::HCCHAR0::DAD;
    pub use super::HCCHAR0::EPDIR;
    pub use super::HCCHAR0::EPNUM;
    pub use super::HCCHAR0::EPTYP;
    pub use super::HCCHAR0::LSDEV;
    pub use super::HCCHAR0::MC;
    pub use super::HCCHAR0::MPSIZ;
    pub use super::HCCHAR0::ODDFRM;
}

/// OTG_HS host channel-14 split control register
pub mod HCSPLT14 {
    pub use super::HCSPLT0::COMPLSPLT;
    pub use super::HCSPLT0::HUBADDR;
    pub use super::HCSPLT0::PRTADDR;
    pub use super::HCSPLT0::SPLITEN;
    pub use super::HCSPLT0::XACTPOS;
}

/// OTG_HS host channel-14 interrupt register
pub mod HCINT14 {
    pub use super::HCINT0::ACK;
    pub use super::HCINT0::AHBERR;
    pub use super::HCINT0::BBERR;
    pub use super::HCINT0::CHH;
    pub use super::HCINT0::DTERR;
    pub use super::HCINT0::FRMOR;
    pub use super::HCINT0::NAK;
    pub use super::HCINT0::NYET;
    pub use super::HCINT0::STALL;
    pub use super::HCINT0::TXERR;
    pub use super::HCINT0::XFRC;
}

/// OTG_HS host channel-14 interrupt mask register
pub mod HCINTMSK14 {
    pub use super::HCINTMSK0::ACKM;
    pub use super::HCINTMSK0::AHBERR;
    pub use super::HCINTMSK0::BBERRM;
    pub use super::HCINTMSK0::CHHM;
    pub use super::HCINTMSK0::DTERRM;
    pub use super::HCINTMSK0::FRMORM;
    pub use super::HCINTMSK0::NAKM;
    pub use super::HCINTMSK0::NYET;
    pub use super::HCINTMSK0::STALLM;
    pub use super::HCINTMSK0::TXERRM;
    pub use super::HCINTMSK0::XFRCM;
}

/// OTG_HS host channel-14 transfer size register
pub mod HCTSIZ14 {
    pub use super::HCTSIZ0::DPID;
    pub use super::HCTSIZ0::PKTCNT;
    pub use super::HCTSIZ0::XFRSIZ;
}

/// OTG_HS host channel-14 DMA address register
pub mod HCDMA14 {
    pub use super::HCDMA0::DMAADDR;
}

/// OTG_HS host channel-15 characteristics register
pub mod HCCHAR15 {
    pub use super::HCCHAR0::CHDIS;
    pub use super::HCCHAR0::CHENA;
    pub use super::HCCHAR0::DAD;
    pub use super::HCCHAR0::EPDIR;
    pub use super::HCCHAR0::EPNUM;
    pub use super::HCCHAR0::EPTYP;
    pub use super::HCCHAR0::LSDEV;
    pub use super::HCCHAR0::MC;
    pub use super::HCCHAR0::MPSIZ;
    pub use super::HCCHAR0::ODDFRM;
}

/// OTG_HS host channel-15 split control register
pub mod HCSPLT15 {
    pub use super::HCSPLT0::COMPLSPLT;
    pub use super::HCSPLT0::HUBADDR;
    pub use super::HCSPLT0::PRTADDR;
    pub use super::HCSPLT0::SPLITEN;
    pub use super::HCSPLT0::XACTPOS;
}

/// OTG_HS host channel-15 interrupt register
pub mod HCINT15 {
    pub use super::HCINT0::ACK;
    pub use super::HCINT0::AHBERR;
    pub use super::HCINT0::BBERR;
    pub use super::HCINT0::CHH;
    pub use super::HCINT0::DTERR;
    pub use super::HCINT0::FRMOR;
    pub use super::HCINT0::NAK;
    pub use super::HCINT0::NYET;
    pub use super::HCINT0::STALL;
    pub use super::HCINT0::TXERR;
    pub use super::HCINT0::XFRC;
}

/// OTG_HS host channel-15 interrupt mask register
pub mod HCINTMSK15 {

    /// Transfer completed mask
    pub mod XFRCM {
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

    /// Channel halted mask
    pub mod CHHM {
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

    /// AHB error
    pub mod AHBERR {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// STALL response received interrupt mask
    pub mod STALL {
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

    /// NAK response received interrupt mask
    pub mod NAKM {
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

    /// ACK response received/transmitted interrupt mask
    pub mod ACKM {
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

    /// Response received interrupt
    pub mod NYET {
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

    /// Transaction error
    pub mod TXERRM {
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

    /// Babble error
    pub mod BBERRM {
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

    /// Frame overrun mask
    pub mod FRMORM {
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

    /// Data toggle error mask
    pub mod DTERRM {
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
}

/// OTG_HS host channel-15 transfer size register
pub mod HCTSIZ15 {
    pub use super::HCTSIZ0::DPID;
    pub use super::HCTSIZ0::PKTCNT;
    pub use super::HCTSIZ0::XFRSIZ;
}

/// OTG_HS host channel-15 DMA address register
pub mod HCDMA15 {
    pub use super::HCDMA0::DMAADDR;
}
pub struct RegisterBlock {
    /// OTG_HS host configuration register
    pub HCFG: RWRegister<u32>,

    /// OTG_HS Host frame interval register
    pub HFIR: RWRegister<u32>,

    /// OTG_HS host frame number/frame time remaining register
    pub HFNUM: RORegister<u32>,

    _reserved1: [u32; 1],

    /// OTG_HS_Host periodic transmit FIFO/queue status register
    pub HPTXSTS: RWRegister<u32>,

    /// OTG_HS Host all channels interrupt register
    pub HAINT: RORegister<u32>,

    /// OTG_HS host all channels interrupt mask register
    pub HAINTMSK: RWRegister<u32>,

    _reserved2: [u32; 9],

    /// OTG_HS host port control and status register
    pub HPRT: RWRegister<u32>,

    _reserved3: [u32; 47],

    /// OTG_HS host channel-0 characteristics register
    pub HCCHAR0: RWRegister<u32>,

    /// OTG_HS host channel-0 split control register
    pub HCSPLT0: RWRegister<u32>,

    /// OTG_HS host channel-11 interrupt register
    pub HCINT0: RWRegister<u32>,

    /// OTG_HS host channel-11 interrupt mask register
    pub HCINTMSK0: RWRegister<u32>,

    /// OTG_HS host channel-11 transfer size register
    pub HCTSIZ0: RWRegister<u32>,

    /// OTG_HS host channel-0 DMA address register
    pub HCDMA0: UnsafeRWRegister<u32>,

    _reserved4: [u32; 2],

    /// OTG_HS host channel-1 characteristics register
    pub HCCHAR1: RWRegister<u32>,

    /// OTG_HS host channel-1 split control register
    pub HCSPLT1: RWRegister<u32>,

    /// OTG_HS host channel-1 interrupt register
    pub HCINT1: RWRegister<u32>,

    /// OTG_HS host channel-1 interrupt mask register
    pub HCINTMSK1: RWRegister<u32>,

    /// OTG_HS host channel-1 transfer size register
    pub HCTSIZ1: RWRegister<u32>,

    /// OTG_HS host channel-1 DMA address register
    pub HCDMA1: UnsafeRWRegister<u32>,

    _reserved5: [u32; 2],

    /// OTG_HS host channel-2 characteristics register
    pub HCCHAR2: RWRegister<u32>,

    /// OTG_HS host channel-2 split control register
    pub HCSPLT2: RWRegister<u32>,

    /// OTG_HS host channel-2 interrupt register
    pub HCINT2: RWRegister<u32>,

    /// OTG_HS host channel-2 interrupt mask register
    pub HCINTMSK2: RWRegister<u32>,

    /// OTG_HS host channel-2 transfer size register
    pub HCTSIZ2: RWRegister<u32>,

    /// OTG_HS host channel-2 DMA address register
    pub HCDMA2: UnsafeRWRegister<u32>,

    _reserved6: [u32; 2],

    /// OTG_HS host channel-3 characteristics register
    pub HCCHAR3: RWRegister<u32>,

    /// OTG_HS host channel-3 split control register
    pub HCSPLT3: RWRegister<u32>,

    /// OTG_HS host channel-3 interrupt register
    pub HCINT3: RWRegister<u32>,

    /// OTG_HS host channel-3 interrupt mask register
    pub HCINTMSK3: RWRegister<u32>,

    /// OTG_HS host channel-3 transfer size register
    pub HCTSIZ3: RWRegister<u32>,

    /// OTG_HS host channel-3 DMA address register
    pub HCDMA3: UnsafeRWRegister<u32>,

    _reserved7: [u32; 2],

    /// OTG_HS host channel-4 characteristics register
    pub HCCHAR4: RWRegister<u32>,

    /// OTG_HS host channel-4 split control register
    pub HCSPLT4: RWRegister<u32>,

    /// OTG_HS host channel-4 interrupt register
    pub HCINT4: RWRegister<u32>,

    /// OTG_HS host channel-4 interrupt mask register
    pub HCINTMSK4: RWRegister<u32>,

    /// OTG_HS host channel-4 transfer size register
    pub HCTSIZ4: RWRegister<u32>,

    /// OTG_HS host channel-4 DMA address register
    pub HCDMA4: UnsafeRWRegister<u32>,

    _reserved8: [u32; 2],

    /// OTG_HS host channel-5 characteristics register
    pub HCCHAR5: RWRegister<u32>,

    /// OTG_HS host channel-5 split control register
    pub HCSPLT5: RWRegister<u32>,

    /// OTG_HS host channel-5 interrupt register
    pub HCINT5: RWRegister<u32>,

    /// OTG_HS host channel-5 interrupt mask register
    pub HCINTMSK5: RWRegister<u32>,

    /// OTG_HS host channel-5 transfer size register
    pub HCTSIZ5: RWRegister<u32>,

    /// OTG_HS host channel-5 DMA address register
    pub HCDMA5: UnsafeRWRegister<u32>,

    _reserved9: [u32; 2],

    /// OTG_HS host channel-6 characteristics register
    pub HCCHAR6: RWRegister<u32>,

    /// OTG_HS host channel-6 split control register
    pub HCSPLT6: RWRegister<u32>,

    /// OTG_HS host channel-6 interrupt register
    pub HCINT6: RWRegister<u32>,

    /// OTG_HS host channel-6 interrupt mask register
    pub HCINTMSK6: RWRegister<u32>,

    /// OTG_HS host channel-6 transfer size register
    pub HCTSIZ6: RWRegister<u32>,

    /// OTG_HS host channel-6 DMA address register
    pub HCDMA6: UnsafeRWRegister<u32>,

    _reserved10: [u32; 2],

    /// OTG_HS host channel-7 characteristics register
    pub HCCHAR7: RWRegister<u32>,

    /// OTG_HS host channel-7 split control register
    pub HCSPLT7: RWRegister<u32>,

    /// OTG_HS host channel-7 interrupt register
    pub HCINT7: RWRegister<u32>,

    /// OTG_HS host channel-7 interrupt mask register
    pub HCINTMSK7: RWRegister<u32>,

    /// OTG_HS host channel-7 transfer size register
    pub HCTSIZ7: RWRegister<u32>,

    /// OTG_HS host channel-7 DMA address register
    pub HCDMA7: UnsafeRWRegister<u32>,

    _reserved11: [u32; 2],

    /// OTG_HS host channel-8 characteristics register
    pub HCCHAR8: RWRegister<u32>,

    /// OTG_HS host channel-8 split control register
    pub HCSPLT8: RWRegister<u32>,

    /// OTG_HS host channel-8 interrupt register
    pub HCINT8: RWRegister<u32>,

    /// OTG_HS host channel-8 interrupt mask register
    pub HCINTMSK8: RWRegister<u32>,

    /// OTG_HS host channel-8 transfer size register
    pub HCTSIZ8: RWRegister<u32>,

    /// OTG_HS host channel-8 DMA address register
    pub HCDMA8: UnsafeRWRegister<u32>,

    _reserved12: [u32; 2],

    /// OTG_HS host channel-9 characteristics register
    pub HCCHAR9: RWRegister<u32>,

    /// OTG_HS host channel-9 split control register
    pub HCSPLT9: RWRegister<u32>,

    /// OTG_HS host channel-9 interrupt register
    pub HCINT9: RWRegister<u32>,

    /// OTG_HS host channel-9 interrupt mask register
    pub HCINTMSK9: RWRegister<u32>,

    /// OTG_HS host channel-9 transfer size register
    pub HCTSIZ9: RWRegister<u32>,

    /// OTG_HS host channel-9 DMA address register
    pub HCDMA9: UnsafeRWRegister<u32>,

    _reserved13: [u32; 2],

    /// OTG_HS host channel-10 characteristics register
    pub HCCHAR10: RWRegister<u32>,

    /// OTG_HS host channel-10 split control register
    pub HCSPLT10: RWRegister<u32>,

    /// OTG_HS host channel-10 interrupt register
    pub HCINT10: RWRegister<u32>,

    /// OTG_HS host channel-10 interrupt mask register
    pub HCINTMSK10: RWRegister<u32>,

    /// OTG_HS host channel-10 transfer size register
    pub HCTSIZ10: RWRegister<u32>,

    /// OTG_HS host channel-10 DMA address register
    pub HCDMA10: UnsafeRWRegister<u32>,

    _reserved14: [u32; 2],

    /// OTG_HS host channel-11 characteristics register
    pub HCCHAR11: RWRegister<u32>,

    /// OTG_HS host channel-11 split control register
    pub HCSPLT11: RWRegister<u32>,

    /// OTG_HS host channel-11 interrupt register
    pub HCINT11: RWRegister<u32>,

    /// OTG_HS host channel-11 interrupt mask register
    pub HCINTMSK11: RWRegister<u32>,

    /// OTG_HS host channel-11 transfer size register
    pub HCTSIZ11: RWRegister<u32>,

    /// OTG_HS host channel-11 DMA address register
    pub HCDMA11: UnsafeRWRegister<u32>,

    /// OTG_HS host channel-12 characteristics register
    pub HCCHAR12: RWRegister<u32>,

    /// OTG_HS host channel-12 split control register
    pub HCSPLT12: RWRegister<u32>,

    /// OTG_HS host channel-12 interrupt register
    pub HCINT12: RWRegister<u32>,

    /// OTG_HS host channel-12 interrupt mask register
    pub HCINTMSK12: RWRegister<u32>,

    /// OTG_HS host channel-12 transfer size register
    pub HCTSIZ12: RWRegister<u32>,

    /// OTG_HS host channel-12 DMA address register
    pub HCDMA12: UnsafeRWRegister<u32>,

    /// OTG_HS host channel-13 characteristics register
    pub HCCHAR13: RWRegister<u32>,

    /// OTG_HS host channel-13 split control register
    pub HCSPLT13: RWRegister<u32>,

    /// OTG_HS host channel-13 interrupt register
    pub HCINT13: RWRegister<u32>,

    /// OTG_HS host channel-13 interrupt mask register
    pub HCINTMSK13: RWRegister<u32>,

    /// OTG_HS host channel-13 transfer size register
    pub HCTSIZ13: RWRegister<u32>,

    /// OTG_HS host channel-13 DMA address register
    pub HCDMA13: UnsafeRWRegister<u32>,

    /// OTG_HS host channel-14 characteristics register
    pub HCCHAR14: RWRegister<u32>,

    /// OTG_HS host channel-14 split control register
    pub HCSPLT14: RWRegister<u32>,

    /// OTG_HS host channel-14 interrupt register
    pub HCINT14: RWRegister<u32>,

    /// OTG_HS host channel-14 interrupt mask register
    pub HCINTMSK14: RWRegister<u32>,

    /// OTG_HS host channel-14 transfer size register
    pub HCTSIZ14: RWRegister<u32>,

    /// OTG_HS host channel-14 DMA address register
    pub HCDMA14: UnsafeRWRegister<u32>,

    /// OTG_HS host channel-15 characteristics register
    pub HCCHAR15: RWRegister<u32>,

    /// OTG_HS host channel-15 split control register
    pub HCSPLT15: RWRegister<u32>,

    /// OTG_HS host channel-15 interrupt register
    pub HCINT15: RWRegister<u32>,

    /// OTG_HS host channel-15 interrupt mask register
    pub HCINTMSK15: RWRegister<u32>,

    /// OTG_HS host channel-15 transfer size register
    pub HCTSIZ15: RWRegister<u32>,

    /// OTG_HS host channel-15 DMA address register
    pub HCDMA15: UnsafeRWRegister<u32>,
}
pub struct ResetValues {
    pub HCFG: u32,
    pub HFIR: u32,
    pub HFNUM: u32,
    pub HPTXSTS: u32,
    pub HAINT: u32,
    pub HAINTMSK: u32,
    pub HPRT: u32,
    pub HCCHAR0: u32,
    pub HCSPLT0: u32,
    pub HCINT0: u32,
    pub HCINTMSK0: u32,
    pub HCTSIZ0: u32,
    pub HCDMA0: u32,
    pub HCCHAR1: u32,
    pub HCSPLT1: u32,
    pub HCINT1: u32,
    pub HCINTMSK1: u32,
    pub HCTSIZ1: u32,
    pub HCDMA1: u32,
    pub HCCHAR2: u32,
    pub HCSPLT2: u32,
    pub HCINT2: u32,
    pub HCINTMSK2: u32,
    pub HCTSIZ2: u32,
    pub HCDMA2: u32,
    pub HCCHAR3: u32,
    pub HCSPLT3: u32,
    pub HCINT3: u32,
    pub HCINTMSK3: u32,
    pub HCTSIZ3: u32,
    pub HCDMA3: u32,
    pub HCCHAR4: u32,
    pub HCSPLT4: u32,
    pub HCINT4: u32,
    pub HCINTMSK4: u32,
    pub HCTSIZ4: u32,
    pub HCDMA4: u32,
    pub HCCHAR5: u32,
    pub HCSPLT5: u32,
    pub HCINT5: u32,
    pub HCINTMSK5: u32,
    pub HCTSIZ5: u32,
    pub HCDMA5: u32,
    pub HCCHAR6: u32,
    pub HCSPLT6: u32,
    pub HCINT6: u32,
    pub HCINTMSK6: u32,
    pub HCTSIZ6: u32,
    pub HCDMA6: u32,
    pub HCCHAR7: u32,
    pub HCSPLT7: u32,
    pub HCINT7: u32,
    pub HCINTMSK7: u32,
    pub HCTSIZ7: u32,
    pub HCDMA7: u32,
    pub HCCHAR8: u32,
    pub HCSPLT8: u32,
    pub HCINT8: u32,
    pub HCINTMSK8: u32,
    pub HCTSIZ8: u32,
    pub HCDMA8: u32,
    pub HCCHAR9: u32,
    pub HCSPLT9: u32,
    pub HCINT9: u32,
    pub HCINTMSK9: u32,
    pub HCTSIZ9: u32,
    pub HCDMA9: u32,
    pub HCCHAR10: u32,
    pub HCSPLT10: u32,
    pub HCINT10: u32,
    pub HCINTMSK10: u32,
    pub HCTSIZ10: u32,
    pub HCDMA10: u32,
    pub HCCHAR11: u32,
    pub HCSPLT11: u32,
    pub HCINT11: u32,
    pub HCINTMSK11: u32,
    pub HCTSIZ11: u32,
    pub HCDMA11: u32,
    pub HCCHAR12: u32,
    pub HCSPLT12: u32,
    pub HCINT12: u32,
    pub HCINTMSK12: u32,
    pub HCTSIZ12: u32,
    pub HCDMA12: u32,
    pub HCCHAR13: u32,
    pub HCSPLT13: u32,
    pub HCINT13: u32,
    pub HCINTMSK13: u32,
    pub HCTSIZ13: u32,
    pub HCDMA13: u32,
    pub HCCHAR14: u32,
    pub HCSPLT14: u32,
    pub HCINT14: u32,
    pub HCINTMSK14: u32,
    pub HCTSIZ14: u32,
    pub HCDMA14: u32,
    pub HCCHAR15: u32,
    pub HCSPLT15: u32,
    pub HCINT15: u32,
    pub HCINTMSK15: u32,
    pub HCTSIZ15: u32,
    pub HCDMA15: u32,
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

/// Access functions for the OTG1_HS_HOST peripheral instance
pub mod OTG1_HS_HOST {
    #[cfg(not(feature = "nosync"))]
    use external_cortex_m;

    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40040400,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in OTG1_HS_HOST
    pub const reset: ResetValues = ResetValues {
        HCFG: 0x00000000,
        HFIR: 0x0000EA60,
        HFNUM: 0x00003FFF,
        HPTXSTS: 0x00080100,
        HAINT: 0x00000000,
        HAINTMSK: 0x00000000,
        HPRT: 0x00000000,
        HCCHAR0: 0x00000000,
        HCCHAR1: 0x00000000,
        HCCHAR2: 0x00000000,
        HCCHAR3: 0x00000000,
        HCCHAR4: 0x00000000,
        HCCHAR5: 0x00000000,
        HCCHAR6: 0x00000000,
        HCCHAR7: 0x00000000,
        HCCHAR8: 0x00000000,
        HCCHAR9: 0x00000000,
        HCCHAR10: 0x00000000,
        HCCHAR11: 0x00000000,
        HCSPLT0: 0x00000000,
        HCSPLT1: 0x00000000,
        HCSPLT2: 0x00000000,
        HCSPLT3: 0x00000000,
        HCSPLT4: 0x00000000,
        HCSPLT5: 0x00000000,
        HCSPLT6: 0x00000000,
        HCSPLT7: 0x00000000,
        HCSPLT8: 0x00000000,
        HCSPLT9: 0x00000000,
        HCSPLT10: 0x00000000,
        HCSPLT11: 0x00000000,
        HCINT0: 0x00000000,
        HCINT1: 0x00000000,
        HCINT2: 0x00000000,
        HCINT3: 0x00000000,
        HCINT4: 0x00000000,
        HCINT5: 0x00000000,
        HCINT6: 0x00000000,
        HCINT7: 0x00000000,
        HCINT8: 0x00000000,
        HCINT9: 0x00000000,
        HCINT10: 0x00000000,
        HCINT11: 0x00000000,
        HCINTMSK0: 0x00000000,
        HCINTMSK1: 0x00000000,
        HCINTMSK2: 0x00000000,
        HCINTMSK3: 0x00000000,
        HCINTMSK4: 0x00000000,
        HCINTMSK5: 0x00000000,
        HCINTMSK6: 0x00000000,
        HCINTMSK7: 0x00000000,
        HCINTMSK8: 0x00000000,
        HCINTMSK9: 0x00000000,
        HCINTMSK10: 0x00000000,
        HCINTMSK11: 0x00000000,
        HCTSIZ0: 0x00000000,
        HCTSIZ1: 0x00000000,
        HCTSIZ2: 0x00000000,
        HCTSIZ3: 0x00000000,
        HCTSIZ4: 0x00000000,
        HCTSIZ5: 0x00000000,
        HCTSIZ6: 0x00000000,
        HCTSIZ7: 0x00000000,
        HCTSIZ8: 0x00000000,
        HCTSIZ9: 0x00000000,
        HCTSIZ10: 0x00000000,
        HCTSIZ11: 0x00000000,
        HCDMA0: 0x00000000,
        HCDMA1: 0x00000000,
        HCDMA2: 0x00000000,
        HCDMA3: 0x00000000,
        HCDMA4: 0x00000000,
        HCDMA5: 0x00000000,
        HCDMA6: 0x00000000,
        HCDMA7: 0x00000000,
        HCDMA8: 0x00000000,
        HCDMA9: 0x00000000,
        HCDMA10: 0x00000000,
        HCDMA11: 0x00000000,
        HCCHAR12: 0x00000000,
        HCSPLT12: 0x00000000,
        HCINT12: 0x00000000,
        HCINTMSK12: 0x00000000,
        HCTSIZ12: 0x00000000,
        HCDMA12: 0x00000000,
        HCCHAR13: 0x00000000,
        HCSPLT13: 0x00000000,
        HCINT13: 0x00000000,
        HCINTMSK13: 0x00000000,
        HCTSIZ13: 0x00000000,
        HCDMA13: 0x00000000,
        HCCHAR14: 0x00000000,
        HCSPLT14: 0x00000000,
        HCINT14: 0x00000000,
        HCINTMSK14: 0x00000000,
        HCTSIZ14: 0x00000000,
        HCDMA14: 0x00000000,
        HCCHAR15: 0x00000000,
        HCSPLT15: 0x00000000,
        HCINT15: 0x00000000,
        HCINTMSK15: 0x00000000,
        HCTSIZ15: 0x00000000,
        HCDMA15: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut OTG1_HS_HOST_TAKEN: bool = false;

    /// Safe access to OTG1_HS_HOST
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
            if OTG1_HS_HOST_TAKEN {
                None
            } else {
                OTG1_HS_HOST_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to OTG1_HS_HOST
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if OTG1_HS_HOST_TAKEN && inst.addr == INSTANCE.addr {
                OTG1_HS_HOST_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal OTG1_HS_HOST
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        OTG1_HS_HOST_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to OTG1_HS_HOST
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const OTG1_HS_HOST: *const RegisterBlock = 0x40040400 as *const _;

/// Access functions for the OTG2_HS_HOST peripheral instance
pub mod OTG2_HS_HOST {
    #[cfg(not(feature = "nosync"))]
    use external_cortex_m;

    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40080400,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in OTG2_HS_HOST
    pub const reset: ResetValues = ResetValues {
        HCFG: 0x00000000,
        HFIR: 0x0000EA60,
        HFNUM: 0x00003FFF,
        HPTXSTS: 0x00080100,
        HAINT: 0x00000000,
        HAINTMSK: 0x00000000,
        HPRT: 0x00000000,
        HCCHAR0: 0x00000000,
        HCCHAR1: 0x00000000,
        HCCHAR2: 0x00000000,
        HCCHAR3: 0x00000000,
        HCCHAR4: 0x00000000,
        HCCHAR5: 0x00000000,
        HCCHAR6: 0x00000000,
        HCCHAR7: 0x00000000,
        HCCHAR8: 0x00000000,
        HCCHAR9: 0x00000000,
        HCCHAR10: 0x00000000,
        HCCHAR11: 0x00000000,
        HCSPLT0: 0x00000000,
        HCSPLT1: 0x00000000,
        HCSPLT2: 0x00000000,
        HCSPLT3: 0x00000000,
        HCSPLT4: 0x00000000,
        HCSPLT5: 0x00000000,
        HCSPLT6: 0x00000000,
        HCSPLT7: 0x00000000,
        HCSPLT8: 0x00000000,
        HCSPLT9: 0x00000000,
        HCSPLT10: 0x00000000,
        HCSPLT11: 0x00000000,
        HCINT0: 0x00000000,
        HCINT1: 0x00000000,
        HCINT2: 0x00000000,
        HCINT3: 0x00000000,
        HCINT4: 0x00000000,
        HCINT5: 0x00000000,
        HCINT6: 0x00000000,
        HCINT7: 0x00000000,
        HCINT8: 0x00000000,
        HCINT9: 0x00000000,
        HCINT10: 0x00000000,
        HCINT11: 0x00000000,
        HCINTMSK0: 0x00000000,
        HCINTMSK1: 0x00000000,
        HCINTMSK2: 0x00000000,
        HCINTMSK3: 0x00000000,
        HCINTMSK4: 0x00000000,
        HCINTMSK5: 0x00000000,
        HCINTMSK6: 0x00000000,
        HCINTMSK7: 0x00000000,
        HCINTMSK8: 0x00000000,
        HCINTMSK9: 0x00000000,
        HCINTMSK10: 0x00000000,
        HCINTMSK11: 0x00000000,
        HCTSIZ0: 0x00000000,
        HCTSIZ1: 0x00000000,
        HCTSIZ2: 0x00000000,
        HCTSIZ3: 0x00000000,
        HCTSIZ4: 0x00000000,
        HCTSIZ5: 0x00000000,
        HCTSIZ6: 0x00000000,
        HCTSIZ7: 0x00000000,
        HCTSIZ8: 0x00000000,
        HCTSIZ9: 0x00000000,
        HCTSIZ10: 0x00000000,
        HCTSIZ11: 0x00000000,
        HCDMA0: 0x00000000,
        HCDMA1: 0x00000000,
        HCDMA2: 0x00000000,
        HCDMA3: 0x00000000,
        HCDMA4: 0x00000000,
        HCDMA5: 0x00000000,
        HCDMA6: 0x00000000,
        HCDMA7: 0x00000000,
        HCDMA8: 0x00000000,
        HCDMA9: 0x00000000,
        HCDMA10: 0x00000000,
        HCDMA11: 0x00000000,
        HCCHAR12: 0x00000000,
        HCSPLT12: 0x00000000,
        HCINT12: 0x00000000,
        HCINTMSK12: 0x00000000,
        HCTSIZ12: 0x00000000,
        HCDMA12: 0x00000000,
        HCCHAR13: 0x00000000,
        HCSPLT13: 0x00000000,
        HCINT13: 0x00000000,
        HCINTMSK13: 0x00000000,
        HCTSIZ13: 0x00000000,
        HCDMA13: 0x00000000,
        HCCHAR14: 0x00000000,
        HCSPLT14: 0x00000000,
        HCINT14: 0x00000000,
        HCINTMSK14: 0x00000000,
        HCTSIZ14: 0x00000000,
        HCDMA14: 0x00000000,
        HCCHAR15: 0x00000000,
        HCSPLT15: 0x00000000,
        HCINT15: 0x00000000,
        HCINTMSK15: 0x00000000,
        HCTSIZ15: 0x00000000,
        HCDMA15: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut OTG2_HS_HOST_TAKEN: bool = false;

    /// Safe access to OTG2_HS_HOST
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
            if OTG2_HS_HOST_TAKEN {
                None
            } else {
                OTG2_HS_HOST_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to OTG2_HS_HOST
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if OTG2_HS_HOST_TAKEN && inst.addr == INSTANCE.addr {
                OTG2_HS_HOST_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal OTG2_HS_HOST
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        OTG2_HS_HOST_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to OTG2_HS_HOST
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const OTG2_HS_HOST: *const RegisterBlock = 0x40080400 as *const _;
