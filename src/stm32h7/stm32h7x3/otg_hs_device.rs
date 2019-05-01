#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! USB 1 on the go high speed

#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;
use {RORegister, RWRegister, UnsafeRWRegister};

/// OTG_HS device configuration register
pub mod DCFG {

    /// Device speed
    pub mod DSPD {
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

    /// Nonzero-length status OUT handshake
    pub mod NZLSOHSK {
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

    /// Device address
    pub mod DAD {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (7 bits: 0x7f << 4)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Periodic (micro)frame interval
    pub mod PFIVL {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (2 bits: 0b11 << 11)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Periodic scheduling interval
    pub mod PERSCHIVL {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (2 bits: 0b11 << 24)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// OTG_HS device control register
pub mod DCTL {

    /// Remote wakeup signaling
    pub mod RWUSIG {
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

    /// Soft disconnect
    pub mod SDIS {
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

    /// Global IN NAK status
    pub mod GINSTS {
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

    /// Global OUT NAK status
    pub mod GONSTS {
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

    /// Test control
    pub mod TCTL {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (3 bits: 0b111 << 4)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Set global IN NAK
    pub mod SGINAK {
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

    /// Clear global IN NAK
    pub mod CGINAK {
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

    /// Set global OUT NAK
    pub mod SGONAK {
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

    /// Clear global OUT NAK
    pub mod CGONAK {
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

    /// Power-on programming done
    pub mod POPRGDNE {
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
}

/// OTG_HS device status register
pub mod DSTS {

    /// Suspend status
    pub mod SUSPSTS {
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

    /// Enumerated speed
    pub mod ENUMSPD {
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

    /// Erratic error
    pub mod EERR {
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

    /// Frame number of the received SOF
    pub mod FNSOF {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (14 bits: 0x3fff << 8)
        pub const mask: u32 = 0x3fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// OTG_HS device IN endpoint common interrupt mask register
pub mod DIEPMSK {

    /// Transfer completed interrupt mask
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

    /// Endpoint disabled interrupt mask
    pub mod EPDM {
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

    /// Timeout condition mask (nonisochronous endpoints)
    pub mod TOM {
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

    /// IN token received when TxFIFO empty mask
    pub mod ITTXFEMSK {
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

    /// IN token received with EP mismatch mask
    pub mod INEPNMM {
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

    /// IN endpoint NAK effective mask
    pub mod INEPNEM {
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

    /// FIFO underrun mask
    pub mod TXFURM {
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

    /// BNA interrupt mask
    pub mod BIM {
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
}

/// OTG_HS device OUT endpoint common interrupt mask register
pub mod DOEPMSK {

    /// Transfer completed interrupt mask
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

    /// Endpoint disabled interrupt mask
    pub mod EPDM {
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

    /// SETUP phase done mask
    pub mod STUPM {
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

    /// OUT token received when endpoint disabled mask
    pub mod OTEPDM {
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

    /// Back-to-back SETUP packets received mask
    pub mod B2BSTUP {
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

    /// OUT packet error mask
    pub mod OPEM {
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

    /// BNA interrupt mask
    pub mod BOIM {
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
}

/// OTG_HS device all endpoints interrupt register
pub mod DAINT {

    /// IN endpoint interrupt bits
    pub mod IEPINT {
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

    /// OUT endpoint interrupt bits
    pub mod OEPINT {
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

/// OTG_HS all endpoints interrupt mask register
pub mod DAINTMSK {

    /// IN EP interrupt mask bits
    pub mod IEPM {
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

    /// OUT EP interrupt mask bits
    pub mod OEPM {
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

/// OTG_HS device VBUS discharge time register
pub mod DVBUSDIS {

    /// Device VBUS discharge time
    pub mod VBUSDT {
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

/// OTG_HS device VBUS pulsing time register
pub mod DVBUSPULSE {

    /// Device VBUS pulsing time
    pub mod DVBUSP {
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
}

/// OTG_HS Device threshold control register
pub mod DTHRCTL {

    /// Nonisochronous IN endpoints threshold enable
    pub mod NONISOTHREN {
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

    /// ISO IN endpoint threshold enable
    pub mod ISOTHREN {
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

    /// Transmit threshold length
    pub mod TXTHRLEN {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (9 bits: 0x1ff << 2)
        pub const mask: u32 = 0x1ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Receive threshold enable
    pub mod RXTHREN {
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

    /// Receive threshold length
    pub mod RXTHRLEN {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (9 bits: 0x1ff << 17)
        pub const mask: u32 = 0x1ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Arbiter parking enable
    pub mod ARPEN {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// OTG_HS device IN endpoint FIFO empty interrupt mask register
pub mod DIEPEMPMSK {

    /// IN EP Tx FIFO empty interrupt mask bits
    pub mod INEPTXFEM {
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

/// OTG_HS device each endpoint interrupt register
pub mod DEACHINT {

    /// IN endpoint 1interrupt bit
    pub mod IEP1INT {
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

    /// OUT endpoint 1 interrupt bit
    pub mod OEP1INT {
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
}

/// OTG_HS device each endpoint interrupt register mask
pub mod DEACHINTMSK {

    /// IN Endpoint 1 interrupt mask bit
    pub mod IEP1INTM {
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

    /// OUT Endpoint 1 interrupt mask bit
    pub mod OEP1INTM {
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
}

/// OTG device endpoint-0 control register
pub mod DIEPCTL0 {

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

    /// USB active endpoint
    pub mod USBAEP {
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

    /// Even/odd frame
    pub mod EONUM_DPID {
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

    /// NAK status
    pub mod NAKSTS {
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

    /// STALL handshake
    pub mod Stall {
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

    /// TxFIFO number
    pub mod TXFNUM {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (4 bits: 0b1111 << 22)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Clear NAK
    pub mod CNAK {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Set NAK
    pub mod SNAK {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Set DATA0 PID
    pub mod SD0PID_SEVNFRM {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Set odd frame
    pub mod SODDFRM {
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

    /// Endpoint disable
    pub mod EPDIS {
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

    /// Endpoint enable
    pub mod EPENA {
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

/// OTG device endpoint-1 control register
pub mod DIEPCTL1 {
    pub use super::DIEPCTL0::Stall;
    pub use super::DIEPCTL0::CNAK;
    pub use super::DIEPCTL0::EONUM_DPID;
    pub use super::DIEPCTL0::EPDIS;
    pub use super::DIEPCTL0::EPENA;
    pub use super::DIEPCTL0::EPTYP;
    pub use super::DIEPCTL0::MPSIZ;
    pub use super::DIEPCTL0::NAKSTS;
    pub use super::DIEPCTL0::SD0PID_SEVNFRM;
    pub use super::DIEPCTL0::SNAK;
    pub use super::DIEPCTL0::SODDFRM;
    pub use super::DIEPCTL0::TXFNUM;
    pub use super::DIEPCTL0::USBAEP;
}

/// OTG device endpoint-2 control register
pub mod DIEPCTL2 {
    pub use super::DIEPCTL0::Stall;
    pub use super::DIEPCTL0::CNAK;
    pub use super::DIEPCTL0::EONUM_DPID;
    pub use super::DIEPCTL0::EPDIS;
    pub use super::DIEPCTL0::EPENA;
    pub use super::DIEPCTL0::EPTYP;
    pub use super::DIEPCTL0::MPSIZ;
    pub use super::DIEPCTL0::NAKSTS;
    pub use super::DIEPCTL0::SD0PID_SEVNFRM;
    pub use super::DIEPCTL0::SNAK;
    pub use super::DIEPCTL0::SODDFRM;
    pub use super::DIEPCTL0::TXFNUM;
    pub use super::DIEPCTL0::USBAEP;
}

/// OTG device endpoint-3 control register
pub mod DIEPCTL3 {
    pub use super::DIEPCTL0::Stall;
    pub use super::DIEPCTL0::CNAK;
    pub use super::DIEPCTL0::EONUM_DPID;
    pub use super::DIEPCTL0::EPDIS;
    pub use super::DIEPCTL0::EPENA;
    pub use super::DIEPCTL0::EPTYP;
    pub use super::DIEPCTL0::MPSIZ;
    pub use super::DIEPCTL0::NAKSTS;
    pub use super::DIEPCTL0::SD0PID_SEVNFRM;
    pub use super::DIEPCTL0::SNAK;
    pub use super::DIEPCTL0::SODDFRM;
    pub use super::DIEPCTL0::TXFNUM;
    pub use super::DIEPCTL0::USBAEP;
}

/// OTG device endpoint-4 control register
pub mod DIEPCTL4 {
    pub use super::DIEPCTL0::Stall;
    pub use super::DIEPCTL0::CNAK;
    pub use super::DIEPCTL0::EONUM_DPID;
    pub use super::DIEPCTL0::EPDIS;
    pub use super::DIEPCTL0::EPENA;
    pub use super::DIEPCTL0::EPTYP;
    pub use super::DIEPCTL0::MPSIZ;
    pub use super::DIEPCTL0::NAKSTS;
    pub use super::DIEPCTL0::SD0PID_SEVNFRM;
    pub use super::DIEPCTL0::SNAK;
    pub use super::DIEPCTL0::SODDFRM;
    pub use super::DIEPCTL0::TXFNUM;
    pub use super::DIEPCTL0::USBAEP;
}

/// DIEPCTL5 and DIEPTSIZ6
/// DIEPCTL5: OTG device endpoint-5 control register
/// DIEPTSIZ6: OTG_HS device endpoint transfer size register
pub mod DIEP {

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

    /// USB active endpoint
    pub mod USBAEP {
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

    /// Even/odd frame
    pub mod EONUM_DPID {
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

    /// NAK status
    pub mod NAKSTS {
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

    /// STALL handshake
    pub mod Stall {
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

    /// TxFIFO number
    pub mod TXFNUM {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (4 bits: 0b1111 << 22)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Clear NAK
    pub mod CNAK {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Set NAK
    pub mod SNAK {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Set DATA0 PID
    pub mod SD0PID_SEVNFRM {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Set odd frame
    pub mod SODDFRM {
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

    /// Endpoint disable
    pub mod EPDIS {
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

    /// Endpoint enable
    pub mod EPENA {
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

    /// Multi count
    pub mod MCNT {
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

/// OTG device endpoint-6 control register
pub mod DIEPCTL6 {
    pub use super::DIEPCTL0::Stall;
    pub use super::DIEPCTL0::CNAK;
    pub use super::DIEPCTL0::EONUM_DPID;
    pub use super::DIEPCTL0::EPDIS;
    pub use super::DIEPCTL0::EPENA;
    pub use super::DIEPCTL0::EPTYP;
    pub use super::DIEPCTL0::MPSIZ;
    pub use super::DIEPCTL0::NAKSTS;
    pub use super::DIEPCTL0::SD0PID_SEVNFRM;
    pub use super::DIEPCTL0::SNAK;
    pub use super::DIEPCTL0::SODDFRM;
    pub use super::DIEPCTL0::TXFNUM;
    pub use super::DIEPCTL0::USBAEP;
}

/// OTG device endpoint-7 control register
pub mod DIEPCTL7 {
    pub use super::DIEPCTL0::Stall;
    pub use super::DIEPCTL0::CNAK;
    pub use super::DIEPCTL0::EONUM_DPID;
    pub use super::DIEPCTL0::EPDIS;
    pub use super::DIEPCTL0::EPENA;
    pub use super::DIEPCTL0::EPTYP;
    pub use super::DIEPCTL0::MPSIZ;
    pub use super::DIEPCTL0::NAKSTS;
    pub use super::DIEPCTL0::SD0PID_SEVNFRM;
    pub use super::DIEPCTL0::SNAK;
    pub use super::DIEPCTL0::SODDFRM;
    pub use super::DIEPCTL0::TXFNUM;
    pub use super::DIEPCTL0::USBAEP;
}

/// OTG device endpoint-0 interrupt register
pub mod DIEPINT0 {

    /// Transfer completed interrupt
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

    /// Endpoint disabled interrupt
    pub mod EPDISD {
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

    /// Timeout condition
    pub mod TOC {
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

    /// IN token received when TxFIFO is empty
    pub mod ITTXFE {
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

    /// IN endpoint NAK effective
    pub mod INEPNE {
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

    /// Transmit FIFO empty
    pub mod TXFE {
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

    /// Transmit Fifo Underrun
    pub mod TXFIFOUDRN {
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

    /// Buffer not available interrupt
    pub mod BNA {
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

    /// Packet dropped status
    pub mod PKTDRPSTS {
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

    /// Babble error interrupt
    pub mod BERR {
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

    /// NAK interrupt
    pub mod NAK {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// OTG device endpoint-1 interrupt register
pub mod DIEPINT1 {
    pub use super::DIEPINT0::BERR;
    pub use super::DIEPINT0::BNA;
    pub use super::DIEPINT0::EPDISD;
    pub use super::DIEPINT0::INEPNE;
    pub use super::DIEPINT0::ITTXFE;
    pub use super::DIEPINT0::NAK;
    pub use super::DIEPINT0::PKTDRPSTS;
    pub use super::DIEPINT0::TOC;
    pub use super::DIEPINT0::TXFE;
    pub use super::DIEPINT0::TXFIFOUDRN;
    pub use super::DIEPINT0::XFRC;
}

/// OTG device endpoint-2 interrupt register
pub mod DIEPINT2 {
    pub use super::DIEPINT0::BERR;
    pub use super::DIEPINT0::BNA;
    pub use super::DIEPINT0::EPDISD;
    pub use super::DIEPINT0::INEPNE;
    pub use super::DIEPINT0::ITTXFE;
    pub use super::DIEPINT0::NAK;
    pub use super::DIEPINT0::PKTDRPSTS;
    pub use super::DIEPINT0::TOC;
    pub use super::DIEPINT0::TXFE;
    pub use super::DIEPINT0::TXFIFOUDRN;
    pub use super::DIEPINT0::XFRC;
}

/// OTG device endpoint-3 interrupt register
pub mod DIEPINT3 {
    pub use super::DIEPINT0::BERR;
    pub use super::DIEPINT0::BNA;
    pub use super::DIEPINT0::EPDISD;
    pub use super::DIEPINT0::INEPNE;
    pub use super::DIEPINT0::ITTXFE;
    pub use super::DIEPINT0::NAK;
    pub use super::DIEPINT0::PKTDRPSTS;
    pub use super::DIEPINT0::TOC;
    pub use super::DIEPINT0::TXFE;
    pub use super::DIEPINT0::TXFIFOUDRN;
    pub use super::DIEPINT0::XFRC;
}

/// OTG device endpoint-4 interrupt register
pub mod DIEPINT4 {
    pub use super::DIEPINT0::BERR;
    pub use super::DIEPINT0::BNA;
    pub use super::DIEPINT0::EPDISD;
    pub use super::DIEPINT0::INEPNE;
    pub use super::DIEPINT0::ITTXFE;
    pub use super::DIEPINT0::NAK;
    pub use super::DIEPINT0::PKTDRPSTS;
    pub use super::DIEPINT0::TOC;
    pub use super::DIEPINT0::TXFE;
    pub use super::DIEPINT0::TXFIFOUDRN;
    pub use super::DIEPINT0::XFRC;
}

/// DIEPINT5 and DIEPTSIZ7
/// DIEPINT5: OTG device endpoint-5 interrupt register
/// DIEPTSIZ7: OTG_HS device endpoint transfer size register
pub mod DIEPINT5 {

    /// Transfer completed interrupt
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

    /// Endpoint disabled interrupt
    pub mod EPDISD {
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

    /// Timeout condition
    pub mod TOC {
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

    /// IN token received when TxFIFO is empty
    pub mod ITTXFE {
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

    /// IN endpoint NAK effective
    pub mod INEPNE {
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

    /// Transmit FIFO empty
    pub mod TXFE {
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

    /// Transmit Fifo Underrun
    pub mod TXFIFOUDRN {
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

    /// Buffer not available interrupt
    pub mod BNA {
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

    /// Packet dropped status
    pub mod PKTDRPSTS {
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

    /// Babble error interrupt
    pub mod BERR {
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

    /// NAK interrupt
    pub mod NAK {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

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

    /// Multi count
    pub mod MCNT {
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

/// OTG device endpoint-6 interrupt register
pub mod DIEPINT6 {
    pub use super::DIEPINT0::BERR;
    pub use super::DIEPINT0::BNA;
    pub use super::DIEPINT0::EPDISD;
    pub use super::DIEPINT0::INEPNE;
    pub use super::DIEPINT0::ITTXFE;
    pub use super::DIEPINT0::NAK;
    pub use super::DIEPINT0::PKTDRPSTS;
    pub use super::DIEPINT0::TOC;
    pub use super::DIEPINT0::TXFE;
    pub use super::DIEPINT0::TXFIFOUDRN;
    pub use super::DIEPINT0::XFRC;
}

/// OTG device endpoint-7 interrupt register
pub mod DIEPINT7 {
    pub use super::DIEPINT0::BERR;
    pub use super::DIEPINT0::BNA;
    pub use super::DIEPINT0::EPDISD;
    pub use super::DIEPINT0::INEPNE;
    pub use super::DIEPINT0::ITTXFE;
    pub use super::DIEPINT0::NAK;
    pub use super::DIEPINT0::PKTDRPSTS;
    pub use super::DIEPINT0::TOC;
    pub use super::DIEPINT0::TXFE;
    pub use super::DIEPINT0::TXFIFOUDRN;
    pub use super::DIEPINT0::XFRC;
}

/// OTG_HS device IN endpoint 0 transfer size register
pub mod DIEPTSIZ0 {

    /// Transfer size
    pub mod XFRSIZ {
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

    /// Packet count
    pub mod PKTCNT {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (2 bits: 0b11 << 19)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// OTG_HS device endpoint-1 DMA address register
pub mod DIEPDMA1 {

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

/// OTG_HS device endpoint-2 DMA address register
pub mod DIEPDMA2 {
    pub use super::DIEPDMA1::DMAADDR;
}

/// OTG_HS device endpoint-3 DMA address register
pub mod DIEPDMA3 {
    pub use super::DIEPDMA1::DMAADDR;
}

/// OTG_HS device endpoint-4 DMA address register
pub mod DIEPDMA4 {
    pub use super::DIEPDMA1::DMAADDR;
}

/// OTG_HS device endpoint-5 DMA address register
pub mod DIEPDMA5 {
    pub use super::DIEPDMA1::DMAADDR;
}

/// OTG_HS device IN endpoint transmit FIFO status register
pub mod DTXFSTS0 {

    /// IN endpoint TxFIFO space avail
    pub mod INEPTFSAV {
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

/// OTG_HS device IN endpoint transmit FIFO status register
pub mod DTXFSTS1 {
    pub use super::DTXFSTS0::INEPTFSAV;
}

/// OTG_HS device IN endpoint transmit FIFO status register
pub mod DTXFSTS2 {
    pub use super::DTXFSTS0::INEPTFSAV;
}

/// OTG_HS device IN endpoint transmit FIFO status register
pub mod DTXFSTS3 {
    pub use super::DTXFSTS0::INEPTFSAV;
}

/// OTG_HS device IN endpoint transmit FIFO status register
pub mod DTXFSTS4 {
    pub use super::DTXFSTS0::INEPTFSAV;
}

/// OTG_HS device IN endpoint transmit FIFO status register
pub mod DTXFSTS5 {
    pub use super::DTXFSTS0::INEPTFSAV;
}

/// OTG_HS device endpoint transfer size register
pub mod DIEPTSIZ1 {

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

    /// Multi count
    pub mod MCNT {
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

/// OTG_HS device endpoint transfer size register
pub mod DIEPTSIZ2 {
    pub use super::DIEPTSIZ1::MCNT;
    pub use super::DIEPTSIZ1::PKTCNT;
    pub use super::DIEPTSIZ1::XFRSIZ;
}

/// OTG_HS device endpoint transfer size register
pub mod DIEPTSIZ3 {
    pub use super::DIEPTSIZ1::MCNT;
    pub use super::DIEPTSIZ1::PKTCNT;
    pub use super::DIEPTSIZ1::XFRSIZ;
}

/// OTG_HS device endpoint transfer size register
pub mod DIEPTSIZ4 {
    pub use super::DIEPTSIZ1::MCNT;
    pub use super::DIEPTSIZ1::PKTCNT;
    pub use super::DIEPTSIZ1::XFRSIZ;
}

/// OTG_HS device endpoint transfer size register
pub mod DIEPTSIZ5 {
    pub use super::DIEPTSIZ1::MCNT;
    pub use super::DIEPTSIZ1::PKTCNT;
    pub use super::DIEPTSIZ1::XFRSIZ;
}

/// OTG_HS device control OUT endpoint 0 control register
pub mod DOEPCTL0 {

    /// Maximum packet size
    pub mod MPSIZ {
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

    /// USB active endpoint
    pub mod USBAEP {
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

    /// NAK status
    pub mod NAKSTS {
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

    /// Snoop mode
    pub mod SNPM {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// STALL handshake
    pub mod Stall {
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

    /// Clear NAK
    pub mod CNAK {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Set NAK
    pub mod SNAK {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Endpoint disable
    pub mod EPDIS {
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

    /// Endpoint enable
    pub mod EPENA {
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

/// OTG device endpoint-1 control register
pub mod DOEPCTL1 {

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

    /// USB active endpoint
    pub mod USBAEP {
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

    /// Even odd frame/Endpoint data PID
    pub mod EONUM_DPID {
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

    /// NAK status
    pub mod NAKSTS {
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

    /// Snoop mode
    pub mod SNPM {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// STALL handshake
    pub mod Stall {
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

    /// Clear NAK
    pub mod CNAK {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Set NAK
    pub mod SNAK {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Set DATA0 PID/Set even frame
    pub mod SD0PID_SEVNFRM {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Set odd frame
    pub mod SODDFRM {
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

    /// Endpoint disable
    pub mod EPDIS {
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

    /// Endpoint enable
    pub mod EPENA {
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

/// OTG device endpoint-2 control register
pub mod DOEPCTL2 {
    pub use super::DOEPCTL1::Stall;
    pub use super::DOEPCTL1::CNAK;
    pub use super::DOEPCTL1::EONUM_DPID;
    pub use super::DOEPCTL1::EPDIS;
    pub use super::DOEPCTL1::EPENA;
    pub use super::DOEPCTL1::EPTYP;
    pub use super::DOEPCTL1::MPSIZ;
    pub use super::DOEPCTL1::NAKSTS;
    pub use super::DOEPCTL1::SD0PID_SEVNFRM;
    pub use super::DOEPCTL1::SNAK;
    pub use super::DOEPCTL1::SNPM;
    pub use super::DOEPCTL1::SODDFRM;
    pub use super::DOEPCTL1::USBAEP;
}

/// OTG device endpoint-3 control register
pub mod DOEPCTL3 {
    pub use super::DOEPCTL1::Stall;
    pub use super::DOEPCTL1::CNAK;
    pub use super::DOEPCTL1::EONUM_DPID;
    pub use super::DOEPCTL1::EPDIS;
    pub use super::DOEPCTL1::EPENA;
    pub use super::DOEPCTL1::EPTYP;
    pub use super::DOEPCTL1::MPSIZ;
    pub use super::DOEPCTL1::NAKSTS;
    pub use super::DOEPCTL1::SD0PID_SEVNFRM;
    pub use super::DOEPCTL1::SNAK;
    pub use super::DOEPCTL1::SNPM;
    pub use super::DOEPCTL1::SODDFRM;
    pub use super::DOEPCTL1::USBAEP;
}

/// OTG_HS device endpoint-0 interrupt register
pub mod DOEPINT0 {

    /// Transfer completed interrupt
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

    /// Endpoint disabled interrupt
    pub mod EPDISD {
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

    /// SETUP phase done
    pub mod STUP {
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

    /// OUT token received when endpoint disabled
    pub mod OTEPDIS {
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

    /// Back-to-back SETUP packets received
    pub mod B2BSTUP {
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

    /// NYET interrupt
    pub mod NYET {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// OTG_HS device endpoint-1 interrupt register
pub mod DOEPINT1 {
    pub use super::DOEPINT0::B2BSTUP;
    pub use super::DOEPINT0::EPDISD;
    pub use super::DOEPINT0::NYET;
    pub use super::DOEPINT0::OTEPDIS;
    pub use super::DOEPINT0::STUP;
    pub use super::DOEPINT0::XFRC;
}

/// OTG_HS device endpoint-2 interrupt register
pub mod DOEPINT2 {
    pub use super::DOEPINT0::B2BSTUP;
    pub use super::DOEPINT0::EPDISD;
    pub use super::DOEPINT0::NYET;
    pub use super::DOEPINT0::OTEPDIS;
    pub use super::DOEPINT0::STUP;
    pub use super::DOEPINT0::XFRC;
}

/// OTG_HS device endpoint-3 interrupt register
pub mod DOEPINT3 {
    pub use super::DOEPINT0::B2BSTUP;
    pub use super::DOEPINT0::EPDISD;
    pub use super::DOEPINT0::NYET;
    pub use super::DOEPINT0::OTEPDIS;
    pub use super::DOEPINT0::STUP;
    pub use super::DOEPINT0::XFRC;
}

/// OTG_HS device endpoint-4 interrupt register
pub mod DOEPINT4 {
    pub use super::DOEPINT0::B2BSTUP;
    pub use super::DOEPINT0::EPDISD;
    pub use super::DOEPINT0::NYET;
    pub use super::DOEPINT0::OTEPDIS;
    pub use super::DOEPINT0::STUP;
    pub use super::DOEPINT0::XFRC;
}

/// OTG_HS device endpoint-5 interrupt register
pub mod DOEPINT5 {
    pub use super::DOEPINT0::B2BSTUP;
    pub use super::DOEPINT0::EPDISD;
    pub use super::DOEPINT0::NYET;
    pub use super::DOEPINT0::OTEPDIS;
    pub use super::DOEPINT0::STUP;
    pub use super::DOEPINT0::XFRC;
}

/// OTG_HS device endpoint-6 interrupt register
pub mod DOEPINT6 {
    pub use super::DOEPINT0::B2BSTUP;
    pub use super::DOEPINT0::EPDISD;
    pub use super::DOEPINT0::NYET;
    pub use super::DOEPINT0::OTEPDIS;
    pub use super::DOEPINT0::STUP;
    pub use super::DOEPINT0::XFRC;
}

/// OTG_HS device endpoint-7 interrupt register
pub mod DOEPINT7 {
    pub use super::DOEPINT0::B2BSTUP;
    pub use super::DOEPINT0::EPDISD;
    pub use super::DOEPINT0::NYET;
    pub use super::DOEPINT0::OTEPDIS;
    pub use super::DOEPINT0::STUP;
    pub use super::DOEPINT0::XFRC;
}

/// OTG_HS device endpoint-0 transfer size register
pub mod DOEPTSIZ0 {

    /// Transfer size
    pub mod XFRSIZ {
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

    /// Packet count
    pub mod PKTCNT {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SETUP packet count
    pub mod STUPCNT {
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

/// OTG_HS device endpoint-1 transfer size register
pub mod DOEPTSIZ1 {

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

    /// Received data PID/SETUP packet count
    pub mod RXDPID_STUPCNT {
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

/// OTG_HS device endpoint-2 transfer size register
pub mod DOEPTSIZ2 {
    pub use super::DOEPTSIZ1::PKTCNT;
    pub use super::DOEPTSIZ1::RXDPID_STUPCNT;
    pub use super::DOEPTSIZ1::XFRSIZ;
}

/// OTG_HS device endpoint-3 transfer size register
pub mod DOEPTSIZ3 {
    pub use super::DOEPTSIZ1::PKTCNT;
    pub use super::DOEPTSIZ1::RXDPID_STUPCNT;
    pub use super::DOEPTSIZ1::XFRSIZ;
}

/// OTG_HS device endpoint-4 transfer size register
pub mod DOEPTSIZ4 {
    pub use super::DOEPTSIZ1::PKTCNT;
    pub use super::DOEPTSIZ1::RXDPID_STUPCNT;
    pub use super::DOEPTSIZ1::XFRSIZ;
}

/// OTG_HS device IN endpoint transmit FIFO status register
pub mod DTXFSTS6 {

    /// IN endpoint TxFIFO space avail
    pub mod INEPTFSAV {
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

/// OTG_HS device IN endpoint transmit FIFO status register
pub mod DTXFSTS7 {
    pub use super::DTXFSTS6::INEPTFSAV;
}

/// OTG device endpoint-4 control register
pub mod DOEPCTL4 {
    pub use super::DOEPCTL1::Stall;
    pub use super::DOEPCTL1::CNAK;
    pub use super::DOEPCTL1::EONUM_DPID;
    pub use super::DOEPCTL1::EPDIS;
    pub use super::DOEPCTL1::EPENA;
    pub use super::DOEPCTL1::EPTYP;
    pub use super::DOEPCTL1::MPSIZ;
    pub use super::DOEPCTL1::NAKSTS;
    pub use super::DOEPCTL1::SD0PID_SEVNFRM;
    pub use super::DOEPCTL1::SNAK;
    pub use super::DOEPCTL1::SNPM;
    pub use super::DOEPCTL1::SODDFRM;
    pub use super::DOEPCTL1::USBAEP;
}

/// OTG device endpoint-5 control register
pub mod DOEPCTL5 {
    pub use super::DOEPCTL1::Stall;
    pub use super::DOEPCTL1::CNAK;
    pub use super::DOEPCTL1::EONUM_DPID;
    pub use super::DOEPCTL1::EPDIS;
    pub use super::DOEPCTL1::EPENA;
    pub use super::DOEPCTL1::EPTYP;
    pub use super::DOEPCTL1::MPSIZ;
    pub use super::DOEPCTL1::NAKSTS;
    pub use super::DOEPCTL1::SD0PID_SEVNFRM;
    pub use super::DOEPCTL1::SNAK;
    pub use super::DOEPCTL1::SNPM;
    pub use super::DOEPCTL1::SODDFRM;
    pub use super::DOEPCTL1::USBAEP;
}

/// OTG device endpoint-6 control register
pub mod DOEPCTL6 {
    pub use super::DOEPCTL1::Stall;
    pub use super::DOEPCTL1::CNAK;
    pub use super::DOEPCTL1::EONUM_DPID;
    pub use super::DOEPCTL1::EPDIS;
    pub use super::DOEPCTL1::EPENA;
    pub use super::DOEPCTL1::EPTYP;
    pub use super::DOEPCTL1::MPSIZ;
    pub use super::DOEPCTL1::NAKSTS;
    pub use super::DOEPCTL1::SD0PID_SEVNFRM;
    pub use super::DOEPCTL1::SNAK;
    pub use super::DOEPCTL1::SNPM;
    pub use super::DOEPCTL1::SODDFRM;
    pub use super::DOEPCTL1::USBAEP;
}

/// OTG device endpoint-7 control register
pub mod DOEPCTL7 {
    pub use super::DOEPCTL1::Stall;
    pub use super::DOEPCTL1::CNAK;
    pub use super::DOEPCTL1::EONUM_DPID;
    pub use super::DOEPCTL1::EPDIS;
    pub use super::DOEPCTL1::EPENA;
    pub use super::DOEPCTL1::EPTYP;
    pub use super::DOEPCTL1::MPSIZ;
    pub use super::DOEPCTL1::NAKSTS;
    pub use super::DOEPCTL1::SD0PID_SEVNFRM;
    pub use super::DOEPCTL1::SNAK;
    pub use super::DOEPCTL1::SNPM;
    pub use super::DOEPCTL1::SODDFRM;
    pub use super::DOEPCTL1::USBAEP;
}

/// OTG_HS device endpoint-5 transfer size register
pub mod DOEPTSIZ5 {
    pub use super::DOEPTSIZ1::PKTCNT;
    pub use super::DOEPTSIZ1::RXDPID_STUPCNT;
    pub use super::DOEPTSIZ1::XFRSIZ;
}

/// OTG_HS device endpoint-6 transfer size register
pub mod DOEPTSIZ6 {
    pub use super::DOEPTSIZ1::PKTCNT;
    pub use super::DOEPTSIZ1::RXDPID_STUPCNT;
    pub use super::DOEPTSIZ1::XFRSIZ;
}

/// OTG_HS device endpoint-7 transfer size register
pub mod DOEPTSIZ7 {
    pub use super::DOEPTSIZ1::PKTCNT;
    pub use super::DOEPTSIZ1::RXDPID_STUPCNT;
    pub use super::DOEPTSIZ1::XFRSIZ;
}
pub struct RegisterBlock {
    /// OTG_HS device configuration register
    pub DCFG: RWRegister<u32>,

    /// OTG_HS device control register
    pub DCTL: RWRegister<u32>,

    /// OTG_HS device status register
    pub DSTS: RORegister<u32>,

    _reserved1: [u32; 1],

    /// OTG_HS device IN endpoint common interrupt mask register
    pub DIEPMSK: RWRegister<u32>,

    /// OTG_HS device OUT endpoint common interrupt mask register
    pub DOEPMSK: RWRegister<u32>,

    /// OTG_HS device all endpoints interrupt register
    pub DAINT: RORegister<u32>,

    /// OTG_HS all endpoints interrupt mask register
    pub DAINTMSK: RWRegister<u32>,

    _reserved2: [u32; 2],

    /// OTG_HS device VBUS discharge time register
    pub DVBUSDIS: RWRegister<u32>,

    /// OTG_HS device VBUS pulsing time register
    pub DVBUSPULSE: RWRegister<u32>,

    /// OTG_HS Device threshold control register
    pub DTHRCTL: RWRegister<u32>,

    /// OTG_HS device IN endpoint FIFO empty interrupt mask register
    pub DIEPEMPMSK: RWRegister<u32>,

    /// OTG_HS device each endpoint interrupt register
    pub DEACHINT: RWRegister<u32>,

    /// OTG_HS device each endpoint interrupt register mask
    pub DEACHINTMSK: RWRegister<u32>,

    _reserved3: [u32; 48],

    /// OTG device endpoint-0 control register
    pub DIEPCTL0: RWRegister<u32>,

    _reserved4: [u32; 1],

    /// OTG device endpoint-0 interrupt register
    pub DIEPINT0: RWRegister<u32>,

    _reserved5: [u32; 1],

    /// OTG_HS device IN endpoint 0 transfer size register
    pub DIEPTSIZ0: RWRegister<u32>,

    /// OTG_HS device endpoint-1 DMA address register
    pub DIEPDMA1: UnsafeRWRegister<u32>,

    /// OTG_HS device IN endpoint transmit FIFO status register
    pub DTXFSTS0: RORegister<u32>,

    _reserved6: [u32; 1],

    /// OTG device endpoint-1 control register
    pub DIEPCTL1: RWRegister<u32>,

    _reserved7: [u32; 1],

    /// OTG device endpoint-1 interrupt register
    pub DIEPINT1: RWRegister<u32>,

    _reserved8: [u32; 1],

    /// OTG_HS device endpoint transfer size register
    pub DIEPTSIZ1: RWRegister<u32>,

    /// OTG_HS device endpoint-2 DMA address register
    pub DIEPDMA2: UnsafeRWRegister<u32>,

    /// OTG_HS device IN endpoint transmit FIFO status register
    pub DTXFSTS1: RORegister<u32>,

    _reserved9: [u32; 1],

    /// OTG device endpoint-2 control register
    pub DIEPCTL2: RWRegister<u32>,

    _reserved10: [u32; 1],

    /// OTG device endpoint-2 interrupt register
    pub DIEPINT2: RWRegister<u32>,

    _reserved11: [u32; 1],

    /// OTG_HS device endpoint transfer size register
    pub DIEPTSIZ2: RWRegister<u32>,

    /// OTG_HS device endpoint-3 DMA address register
    pub DIEPDMA3: UnsafeRWRegister<u32>,

    /// OTG_HS device IN endpoint transmit FIFO status register
    pub DTXFSTS2: RORegister<u32>,

    _reserved12: [u32; 1],

    /// OTG device endpoint-3 control register
    pub DIEPCTL3: RWRegister<u32>,

    _reserved13: [u32; 1],

    /// OTG device endpoint-3 interrupt register
    pub DIEPINT3: RWRegister<u32>,

    _reserved14: [u32; 1],

    /// OTG_HS device endpoint transfer size register
    pub DIEPTSIZ3: RWRegister<u32>,

    /// OTG_HS device endpoint-4 DMA address register
    pub DIEPDMA4: UnsafeRWRegister<u32>,

    /// OTG_HS device IN endpoint transmit FIFO status register
    pub DTXFSTS3: RORegister<u32>,

    _reserved15: [u32; 1],

    /// OTG device endpoint-4 control register
    pub DIEPCTL4: RWRegister<u32>,

    _reserved16: [u32; 1],

    /// OTG device endpoint-4 interrupt register
    pub DIEPINT4: RWRegister<u32>,

    _reserved17: [u32; 1],

    /// OTG_HS device endpoint transfer size register
    pub DIEPTSIZ4: RWRegister<u32>,

    /// OTG_HS device endpoint-5 DMA address register
    pub DIEPDMA5: UnsafeRWRegister<u32>,

    /// OTG_HS device IN endpoint transmit FIFO status register
    pub DTXFSTS4: RORegister<u32>,

    _reserved18: [u32; 1],

    /// DIEPCTL5 and DIEPTSIZ6
    /// DIEPCTL5: OTG device endpoint-5 control register
    /// DIEPTSIZ6: OTG_HS device endpoint transfer size register
    pub DIEP: RWRegister<u32>,

    /// OTG_HS device IN endpoint transmit FIFO status register
    pub DTXFSTS6: RWRegister<u32>,

    /// DIEPINT5 and DIEPTSIZ7
    /// DIEPINT5: OTG device endpoint-5 interrupt register
    /// DIEPTSIZ7: OTG_HS device endpoint transfer size register
    pub DIEPINT5: RWRegister<u32>,

    /// OTG_HS device IN endpoint transmit FIFO status register
    pub DTXFSTS7: RWRegister<u32>,

    /// OTG_HS device endpoint transfer size register
    pub DIEPTSIZ5: RWRegister<u32>,

    _reserved19: [u32; 1],

    /// OTG_HS device IN endpoint transmit FIFO status register
    pub DTXFSTS5: RORegister<u32>,

    _reserved20: [u32; 1],

    /// OTG device endpoint-6 control register
    pub DIEPCTL6: RWRegister<u32>,

    _reserved21: [u32; 1],

    /// OTG device endpoint-6 interrupt register
    pub DIEPINT6: RWRegister<u32>,

    _reserved22: [u32; 5],

    /// OTG device endpoint-7 control register
    pub DIEPCTL7: RWRegister<u32>,

    _reserved23: [u32; 1],

    /// OTG device endpoint-7 interrupt register
    pub DIEPINT7: RWRegister<u32>,

    _reserved24: [u32; 69],

    /// OTG_HS device control OUT endpoint 0 control register
    pub DOEPCTL0: RWRegister<u32>,

    _reserved25: [u32; 1],

    /// OTG_HS device endpoint-0 interrupt register
    pub DOEPINT0: RWRegister<u32>,

    _reserved26: [u32; 1],

    /// OTG_HS device endpoint-0 transfer size register
    pub DOEPTSIZ0: RWRegister<u32>,

    _reserved27: [u32; 3],

    /// OTG device endpoint-1 control register
    pub DOEPCTL1: RWRegister<u32>,

    _reserved28: [u32; 1],

    /// OTG_HS device endpoint-1 interrupt register
    pub DOEPINT1: RWRegister<u32>,

    _reserved29: [u32; 1],

    /// OTG_HS device endpoint-1 transfer size register
    pub DOEPTSIZ1: RWRegister<u32>,

    _reserved30: [u32; 3],

    /// OTG device endpoint-2 control register
    pub DOEPCTL2: RWRegister<u32>,

    _reserved31: [u32; 1],

    /// OTG_HS device endpoint-2 interrupt register
    pub DOEPINT2: RWRegister<u32>,

    _reserved32: [u32; 1],

    /// OTG_HS device endpoint-2 transfer size register
    pub DOEPTSIZ2: RWRegister<u32>,

    _reserved33: [u32; 3],

    /// OTG device endpoint-3 control register
    pub DOEPCTL3: RWRegister<u32>,

    _reserved34: [u32; 1],

    /// OTG_HS device endpoint-3 interrupt register
    pub DOEPINT3: RWRegister<u32>,

    _reserved35: [u32; 1],

    /// OTG_HS device endpoint-3 transfer size register
    pub DOEPTSIZ3: RWRegister<u32>,

    _reserved36: [u32; 3],

    /// OTG device endpoint-4 control register
    pub DOEPCTL4: RWRegister<u32>,

    _reserved37: [u32; 1],

    /// OTG_HS device endpoint-4 interrupt register
    pub DOEPINT4: RWRegister<u32>,

    _reserved38: [u32; 1],

    /// OTG_HS device endpoint-4 transfer size register
    pub DOEPTSIZ4: RWRegister<u32>,

    _reserved39: [u32; 3],

    /// OTG device endpoint-5 control register
    pub DOEPCTL5: RWRegister<u32>,

    _reserved40: [u32; 1],

    /// OTG_HS device endpoint-5 interrupt register
    pub DOEPINT5: RWRegister<u32>,

    _reserved41: [u32; 1],

    /// OTG_HS device endpoint-5 transfer size register
    pub DOEPTSIZ5: RWRegister<u32>,

    _reserved42: [u32; 3],

    /// OTG device endpoint-6 control register
    pub DOEPCTL6: RWRegister<u32>,

    _reserved43: [u32; 1],

    /// OTG_HS device endpoint-6 interrupt register
    pub DOEPINT6: RWRegister<u32>,

    _reserved44: [u32; 1],

    /// OTG_HS device endpoint-6 transfer size register
    pub DOEPTSIZ6: RWRegister<u32>,

    _reserved45: [u32; 3],

    /// OTG device endpoint-7 control register
    pub DOEPCTL7: RWRegister<u32>,

    _reserved46: [u32; 1],

    /// OTG_HS device endpoint-7 interrupt register
    pub DOEPINT7: RWRegister<u32>,

    _reserved47: [u32; 1],

    /// OTG_HS device endpoint-7 transfer size register
    pub DOEPTSIZ7: RWRegister<u32>,
}
pub struct ResetValues {
    pub DCFG: u32,
    pub DCTL: u32,
    pub DSTS: u32,
    pub DIEPMSK: u32,
    pub DOEPMSK: u32,
    pub DAINT: u32,
    pub DAINTMSK: u32,
    pub DVBUSDIS: u32,
    pub DVBUSPULSE: u32,
    pub DTHRCTL: u32,
    pub DIEPEMPMSK: u32,
    pub DEACHINT: u32,
    pub DEACHINTMSK: u32,
    pub DIEPCTL0: u32,
    pub DIEPINT0: u32,
    pub DIEPTSIZ0: u32,
    pub DIEPDMA1: u32,
    pub DTXFSTS0: u32,
    pub DIEPCTL1: u32,
    pub DIEPINT1: u32,
    pub DIEPTSIZ1: u32,
    pub DIEPDMA2: u32,
    pub DTXFSTS1: u32,
    pub DIEPCTL2: u32,
    pub DIEPINT2: u32,
    pub DIEPTSIZ2: u32,
    pub DIEPDMA3: u32,
    pub DTXFSTS2: u32,
    pub DIEPCTL3: u32,
    pub DIEPINT3: u32,
    pub DIEPTSIZ3: u32,
    pub DIEPDMA4: u32,
    pub DTXFSTS3: u32,
    pub DIEPCTL4: u32,
    pub DIEPINT4: u32,
    pub DIEPTSIZ4: u32,
    pub DIEPDMA5: u32,
    pub DTXFSTS4: u32,
    pub DIEP: u32,
    pub DTXFSTS6: u32,
    pub DIEPINT5: u32,
    pub DTXFSTS7: u32,
    pub DIEPTSIZ5: u32,
    pub DTXFSTS5: u32,
    pub DIEPCTL6: u32,
    pub DIEPINT6: u32,
    pub DIEPCTL7: u32,
    pub DIEPINT7: u32,
    pub DOEPCTL0: u32,
    pub DOEPINT0: u32,
    pub DOEPTSIZ0: u32,
    pub DOEPCTL1: u32,
    pub DOEPINT1: u32,
    pub DOEPTSIZ1: u32,
    pub DOEPCTL2: u32,
    pub DOEPINT2: u32,
    pub DOEPTSIZ2: u32,
    pub DOEPCTL3: u32,
    pub DOEPINT3: u32,
    pub DOEPTSIZ3: u32,
    pub DOEPCTL4: u32,
    pub DOEPINT4: u32,
    pub DOEPTSIZ4: u32,
    pub DOEPCTL5: u32,
    pub DOEPINT5: u32,
    pub DOEPTSIZ5: u32,
    pub DOEPCTL6: u32,
    pub DOEPINT6: u32,
    pub DOEPTSIZ6: u32,
    pub DOEPCTL7: u32,
    pub DOEPINT7: u32,
    pub DOEPTSIZ7: u32,
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

/// Access functions for the OTG1_HS_DEVICE peripheral instance
pub mod OTG1_HS_DEVICE {
    #[cfg(not(feature = "nosync"))]
    use external_cortex_m;

    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40040800,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in OTG1_HS_DEVICE
    pub const reset: ResetValues = ResetValues {
        DCFG: 0x02200000,
        DCTL: 0x00000000,
        DSTS: 0x00000010,
        DIEPMSK: 0x00000000,
        DOEPMSK: 0x00000000,
        DAINT: 0x00000000,
        DAINTMSK: 0x00000000,
        DVBUSDIS: 0x000017D7,
        DVBUSPULSE: 0x000005B8,
        DTHRCTL: 0x00000000,
        DIEPEMPMSK: 0x00000000,
        DEACHINT: 0x00000000,
        DEACHINTMSK: 0x00000000,
        DIEPCTL0: 0x00000000,
        DIEPCTL1: 0x00000000,
        DIEPCTL2: 0x00000000,
        DIEPCTL3: 0x00000000,
        DIEPCTL4: 0x00000000,
        DIEP: 0x00000000,
        DIEPCTL6: 0x00000000,
        DIEPCTL7: 0x00000000,
        DIEPINT0: 0x00000080,
        DIEPINT1: 0x00000000,
        DIEPINT2: 0x00000000,
        DIEPINT3: 0x00000000,
        DIEPINT4: 0x00000000,
        DIEPINT5: 0x00000000,
        DIEPINT6: 0x00000000,
        DIEPINT7: 0x00000000,
        DIEPTSIZ0: 0x00000000,
        DIEPDMA1: 0x00000000,
        DIEPDMA2: 0x00000000,
        DIEPDMA3: 0x00000000,
        DIEPDMA4: 0x00000000,
        DIEPDMA5: 0x00000000,
        DTXFSTS0: 0x00000000,
        DTXFSTS1: 0x00000000,
        DTXFSTS2: 0x00000000,
        DTXFSTS3: 0x00000000,
        DTXFSTS4: 0x00000000,
        DTXFSTS5: 0x00000000,
        DIEPTSIZ1: 0x00000000,
        DIEPTSIZ2: 0x00000000,
        DIEPTSIZ3: 0x00000000,
        DIEPTSIZ4: 0x00000000,
        DIEPTSIZ5: 0x00000000,
        DOEPCTL0: 0x00008000,
        DOEPCTL1: 0x00000000,
        DOEPCTL2: 0x00000000,
        DOEPCTL3: 0x00000000,
        DOEPINT0: 0x00000080,
        DOEPINT1: 0x00000000,
        DOEPINT2: 0x00000000,
        DOEPINT3: 0x00000000,
        DOEPINT4: 0x00000000,
        DOEPINT5: 0x00000000,
        DOEPINT6: 0x00000000,
        DOEPINT7: 0x00000000,
        DOEPTSIZ0: 0x00000000,
        DOEPTSIZ1: 0x00000000,
        DOEPTSIZ2: 0x00000000,
        DOEPTSIZ3: 0x00000000,
        DOEPTSIZ4: 0x00000000,
        DTXFSTS6: 0x00000000,
        DTXFSTS7: 0x00000000,
        DOEPCTL4: 0x00000000,
        DOEPCTL5: 0x00000000,
        DOEPCTL6: 0x00000000,
        DOEPCTL7: 0x00000000,
        DOEPTSIZ5: 0x00000000,
        DOEPTSIZ6: 0x00000000,
        DOEPTSIZ7: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut OTG1_HS_DEVICE_TAKEN: bool = false;

    /// Safe access to OTG1_HS_DEVICE
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
            if OTG1_HS_DEVICE_TAKEN {
                None
            } else {
                OTG1_HS_DEVICE_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to OTG1_HS_DEVICE
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if OTG1_HS_DEVICE_TAKEN && inst.addr == INSTANCE.addr {
                OTG1_HS_DEVICE_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }
}

/// Raw pointer to OTG1_HS_DEVICE
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const OTG1_HS_DEVICE: *const RegisterBlock = 0x40040800 as *const _;

/// Access functions for the OTG2_HS_DEVICE peripheral instance
pub mod OTG2_HS_DEVICE {
    #[cfg(not(feature = "nosync"))]
    use external_cortex_m;

    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40080800,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in OTG2_HS_DEVICE
    pub const reset: ResetValues = ResetValues {
        DCFG: 0x02200000,
        DCTL: 0x00000000,
        DSTS: 0x00000010,
        DIEPMSK: 0x00000000,
        DOEPMSK: 0x00000000,
        DAINT: 0x00000000,
        DAINTMSK: 0x00000000,
        DVBUSDIS: 0x000017D7,
        DVBUSPULSE: 0x000005B8,
        DTHRCTL: 0x00000000,
        DIEPEMPMSK: 0x00000000,
        DEACHINT: 0x00000000,
        DEACHINTMSK: 0x00000000,
        DIEPCTL0: 0x00000000,
        DIEPCTL1: 0x00000000,
        DIEPCTL2: 0x00000000,
        DIEPCTL3: 0x00000000,
        DIEPCTL4: 0x00000000,
        DIEP: 0x00000000,
        DIEPCTL6: 0x00000000,
        DIEPCTL7: 0x00000000,
        DIEPINT0: 0x00000080,
        DIEPINT1: 0x00000000,
        DIEPINT2: 0x00000000,
        DIEPINT3: 0x00000000,
        DIEPINT4: 0x00000000,
        DIEPINT5: 0x00000000,
        DIEPINT6: 0x00000000,
        DIEPINT7: 0x00000000,
        DIEPTSIZ0: 0x00000000,
        DIEPDMA1: 0x00000000,
        DIEPDMA2: 0x00000000,
        DIEPDMA3: 0x00000000,
        DIEPDMA4: 0x00000000,
        DIEPDMA5: 0x00000000,
        DTXFSTS0: 0x00000000,
        DTXFSTS1: 0x00000000,
        DTXFSTS2: 0x00000000,
        DTXFSTS3: 0x00000000,
        DTXFSTS4: 0x00000000,
        DTXFSTS5: 0x00000000,
        DIEPTSIZ1: 0x00000000,
        DIEPTSIZ2: 0x00000000,
        DIEPTSIZ3: 0x00000000,
        DIEPTSIZ4: 0x00000000,
        DIEPTSIZ5: 0x00000000,
        DOEPCTL0: 0x00008000,
        DOEPCTL1: 0x00000000,
        DOEPCTL2: 0x00000000,
        DOEPCTL3: 0x00000000,
        DOEPINT0: 0x00000080,
        DOEPINT1: 0x00000000,
        DOEPINT2: 0x00000000,
        DOEPINT3: 0x00000000,
        DOEPINT4: 0x00000000,
        DOEPINT5: 0x00000000,
        DOEPINT6: 0x00000000,
        DOEPINT7: 0x00000000,
        DOEPTSIZ0: 0x00000000,
        DOEPTSIZ1: 0x00000000,
        DOEPTSIZ2: 0x00000000,
        DOEPTSIZ3: 0x00000000,
        DOEPTSIZ4: 0x00000000,
        DTXFSTS6: 0x00000000,
        DTXFSTS7: 0x00000000,
        DOEPCTL4: 0x00000000,
        DOEPCTL5: 0x00000000,
        DOEPCTL6: 0x00000000,
        DOEPCTL7: 0x00000000,
        DOEPTSIZ5: 0x00000000,
        DOEPTSIZ6: 0x00000000,
        DOEPTSIZ7: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut OTG2_HS_DEVICE_TAKEN: bool = false;

    /// Safe access to OTG2_HS_DEVICE
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
            if OTG2_HS_DEVICE_TAKEN {
                None
            } else {
                OTG2_HS_DEVICE_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to OTG2_HS_DEVICE
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if OTG2_HS_DEVICE_TAKEN && inst.addr == INSTANCE.addr {
                OTG2_HS_DEVICE_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }
}

/// Raw pointer to OTG2_HS_DEVICE
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const OTG2_HS_DEVICE: *const RegisterBlock = 0x40080800 as *const _;
