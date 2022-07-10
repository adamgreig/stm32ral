#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! DMAMUX
//!
//! Used by: stm32g070, stm32g071, stm32g081

use crate::{RORegister, RWRegister, WORegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// DMAMux - DMA request line multiplexer channel x control register
pub mod C0CR {

    /// Input DMA request line selected
    pub mod DMAREQ_ID {
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

    /// Interrupt enable at synchronization event overrun
    pub mod SOIE {
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

    /// Event generation enable/disable
    pub mod EGE {
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

    /// Synchronous operating mode enable/disable
    pub mod SE {
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

    /// Synchronization event type selector Defines the synchronization event on the selected synchronization input:
    pub mod SPOL {
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

    /// Number of DMA requests to forward Defines the number of DMA requests forwarded before output event is generated. In synchronous mode, it also defines the number of DMA requests to forward after a synchronization event, then stop forwarding. The actual number of DMA requests forwarded is NBREQ+1. Note: This field can only be written when both SE and EGE bits are reset.
    pub mod NBREQ {
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

    /// Synchronization input selected
    pub mod SYNC_ID {
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
}

/// DMAMux - DMA request line multiplexer channel x control register
pub mod C1CR {
    pub use super::C0CR::DMAREQ_ID;
    pub use super::C0CR::EGE;
    pub use super::C0CR::NBREQ;
    pub use super::C0CR::SE;
    pub use super::C0CR::SOIE;
    pub use super::C0CR::SPOL;
    pub use super::C0CR::SYNC_ID;
}

/// DMAMux - DMA request line multiplexer channel x control register
pub mod C2CR {
    pub use super::C0CR::DMAREQ_ID;
    pub use super::C0CR::EGE;
    pub use super::C0CR::NBREQ;
    pub use super::C0CR::SE;
    pub use super::C0CR::SOIE;
    pub use super::C0CR::SPOL;
    pub use super::C0CR::SYNC_ID;
}

/// DMAMux - DMA request line multiplexer channel x control register
pub mod C3CR {
    pub use super::C0CR::DMAREQ_ID;
    pub use super::C0CR::EGE;
    pub use super::C0CR::NBREQ;
    pub use super::C0CR::SE;
    pub use super::C0CR::SOIE;
    pub use super::C0CR::SPOL;
    pub use super::C0CR::SYNC_ID;
}

/// DMAMux - DMA request line multiplexer channel x control register
pub mod C4CR {
    pub use super::C0CR::DMAREQ_ID;
    pub use super::C0CR::EGE;
    pub use super::C0CR::NBREQ;
    pub use super::C0CR::SE;
    pub use super::C0CR::SOIE;
    pub use super::C0CR::SPOL;
    pub use super::C0CR::SYNC_ID;
}

/// DMAMux - DMA request line multiplexer channel x control register
pub mod C5CR {
    pub use super::C0CR::DMAREQ_ID;
    pub use super::C0CR::EGE;
    pub use super::C0CR::NBREQ;
    pub use super::C0CR::SE;
    pub use super::C0CR::SOIE;
    pub use super::C0CR::SPOL;
    pub use super::C0CR::SYNC_ID;
}

/// DMAMux - DMA request line multiplexer channel x control register
pub mod C6CR {
    pub use super::C0CR::DMAREQ_ID;
    pub use super::C0CR::EGE;
    pub use super::C0CR::NBREQ;
    pub use super::C0CR::SE;
    pub use super::C0CR::SOIE;
    pub use super::C0CR::SPOL;
    pub use super::C0CR::SYNC_ID;
}

/// DMAMux - DMA request generator channel x control register
pub mod RG0CR {

    /// DMA request trigger input selected
    pub mod SIG_ID {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (5 bits: 0b11111 << 0)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Interrupt enable at trigger event overrun
    pub mod OIE {
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

    /// DMA request generator channel enable/disable
    pub mod GE {
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

    /// DMA request generator trigger event type selection Defines the trigger event on the selected DMA request trigger input
    pub mod GPOL {
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

    /// Number of DMA requests to generate Defines the number of DMA requests generated after a trigger event, then stop generating. The actual number of generated DMA requests is GNBREQ+1. Note: This field can only be written when GE bit is reset.
    pub mod GNBREQ {
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
}

/// DMAMux - DMA request generator channel x control register
pub mod RG1CR {
    pub use super::RG0CR::GE;
    pub use super::RG0CR::GNBREQ;
    pub use super::RG0CR::GPOL;
    pub use super::RG0CR::OIE;
    pub use super::RG0CR::SIG_ID;
}

/// DMAMux - DMA request generator channel x control register
pub mod RG2CR {
    pub use super::RG0CR::GE;
    pub use super::RG0CR::GNBREQ;
    pub use super::RG0CR::GPOL;
    pub use super::RG0CR::OIE;
    pub use super::RG0CR::SIG_ID;
}

/// DMAMux - DMA request generator channel x control register
pub mod RG3CR {
    pub use super::RG0CR::GE;
    pub use super::RG0CR::GNBREQ;
    pub use super::RG0CR::GPOL;
    pub use super::RG0CR::OIE;
    pub use super::RG0CR::SIG_ID;
}

/// DMAMux - DMA request generator status register
pub mod RGSR {

    /// Trigger event overrun flag The flag is set when a trigger event occurs on DMA request generator channel x, while the DMA request generator counter value is lower than GNBREQ. The flag is cleared by writing 1 to the corresponding COFx bit in DMAMUX_RGCFR register.
    pub mod OF {
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

/// DMAMux - DMA request generator clear flag register
pub mod RGCFR {

    /// Clear trigger event overrun flag Upon setting, this bit clears the corresponding overrun flag OFx in the DMAMUX_RGCSR register.
    pub mod COF {
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

/// DMAMUX request line multiplexer interrupt channel status register
pub mod CSR {

    /// Synchronization overrun event flag
    pub mod SOF {
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
}

/// DMAMUX request line multiplexer interrupt clear flag register
pub mod CFR {

    /// Clear synchronization overrun event flag
    pub mod CSOF {
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
}

/// DMAMUX size identification register
pub mod SIDR {

    /// Size identification
    pub mod SID {
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

/// DMAMUX IP identification register
pub mod IPIDR {

    /// IP identification
    pub mod ID {
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

/// DMAMUX version register
pub mod VERR {

    /// Minor IP revision
    pub mod MINREV {
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

    /// Major IP revision
    pub mod MAJREV {
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
}

/// DMAMUX hardware configuration 1 register
pub mod HWCFGR1 {

    /// number of DMA request line multiplexer (output) channels
    pub mod NUM_DMA_STREAMS {
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

    /// number of DMA request lines from peripherals
    pub mod NUM_DMA_PERIPH_REQ {
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

    /// number of synchronization inputs
    pub mod NUM_DMA_TRIG {
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

    /// number of DMA request generator channels
    pub mod NUM_DMA_REQGEN {
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

/// DMAMUX hardware configuration 2 register
pub mod HWCFGR2 {

    /// Number of DMA request trigger inputs
    pub mod NUM_DMA_EXT_REQ {
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
    /// DMAMux - DMA request line multiplexer channel x control register
    pub C0CR: RWRegister<u32>,

    /// DMAMux - DMA request line multiplexer channel x control register
    pub C1CR: RWRegister<u32>,

    /// DMAMux - DMA request line multiplexer channel x control register
    pub C2CR: RWRegister<u32>,

    /// DMAMux - DMA request line multiplexer channel x control register
    pub C3CR: RWRegister<u32>,

    /// DMAMux - DMA request line multiplexer channel x control register
    pub C4CR: RWRegister<u32>,

    /// DMAMux - DMA request line multiplexer channel x control register
    pub C5CR: RWRegister<u32>,

    /// DMAMux - DMA request line multiplexer channel x control register
    pub C6CR: RWRegister<u32>,

    _reserved1: [u8; 100],

    /// DMAMUX request line multiplexer interrupt channel status register
    pub CSR: RORegister<u32>,

    /// DMAMUX request line multiplexer interrupt clear flag register
    pub CFR: WORegister<u32>,

    _reserved2: [u8; 120],

    /// DMAMux - DMA request generator channel x control register
    pub RG0CR: RWRegister<u32>,

    /// DMAMux - DMA request generator channel x control register
    pub RG1CR: RWRegister<u32>,

    /// DMAMux - DMA request generator channel x control register
    pub RG2CR: RWRegister<u32>,

    /// DMAMux - DMA request generator channel x control register
    pub RG3CR: RWRegister<u32>,

    _reserved3: [u8; 48],

    /// DMAMux - DMA request generator status register
    pub RGSR: RORegister<u32>,

    /// DMAMux - DMA request generator clear flag register
    pub RGCFR: WORegister<u32>,

    _reserved4: [u8; 676],

    /// DMAMUX hardware configuration 2 register
    pub HWCFGR2: RORegister<u32>,

    /// DMAMUX hardware configuration 1 register
    pub HWCFGR1: RORegister<u32>,

    /// DMAMUX version register
    pub VERR: RORegister<u32>,

    /// DMAMUX IP identification register
    pub IPIDR: RORegister<u32>,

    /// DMAMUX size identification register
    pub SIDR: RORegister<u32>,
}
pub struct ResetValues {
    pub C0CR: u32,
    pub C1CR: u32,
    pub C2CR: u32,
    pub C3CR: u32,
    pub C4CR: u32,
    pub C5CR: u32,
    pub C6CR: u32,
    pub CSR: u32,
    pub CFR: u32,
    pub RG0CR: u32,
    pub RG1CR: u32,
    pub RG2CR: u32,
    pub RG3CR: u32,
    pub RGSR: u32,
    pub RGCFR: u32,
    pub HWCFGR2: u32,
    pub HWCFGR1: u32,
    pub VERR: u32,
    pub IPIDR: u32,
    pub SIDR: u32,
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
