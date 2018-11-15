#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! DMAMUX

#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;
use {RORegister, RWRegister, WORegister};

/// DMAMux - DMA request line multiplexer channel x control register
pub mod DMAMUX1_C0CR {

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
pub mod DMAMUX1_C1CR {
    pub use super::DMAMUX1_C0CR::DMAREQ_ID;
    pub use super::DMAMUX1_C0CR::EGE;
    pub use super::DMAMUX1_C0CR::NBREQ;
    pub use super::DMAMUX1_C0CR::SE;
    pub use super::DMAMUX1_C0CR::SOIE;
    pub use super::DMAMUX1_C0CR::SPOL;
    pub use super::DMAMUX1_C0CR::SYNC_ID;
}

/// DMAMux - DMA request line multiplexer channel x control register
pub mod DMAMUX1_C2CR {
    pub use super::DMAMUX1_C0CR::DMAREQ_ID;
    pub use super::DMAMUX1_C0CR::EGE;
    pub use super::DMAMUX1_C0CR::NBREQ;
    pub use super::DMAMUX1_C0CR::SE;
    pub use super::DMAMUX1_C0CR::SOIE;
    pub use super::DMAMUX1_C0CR::SPOL;
    pub use super::DMAMUX1_C0CR::SYNC_ID;
}

/// DMAMux - DMA request line multiplexer channel x control register
pub mod DMAMUX1_C3CR {
    pub use super::DMAMUX1_C0CR::DMAREQ_ID;
    pub use super::DMAMUX1_C0CR::EGE;
    pub use super::DMAMUX1_C0CR::NBREQ;
    pub use super::DMAMUX1_C0CR::SE;
    pub use super::DMAMUX1_C0CR::SOIE;
    pub use super::DMAMUX1_C0CR::SPOL;
    pub use super::DMAMUX1_C0CR::SYNC_ID;
}

/// DMAMux - DMA request line multiplexer channel x control register
pub mod DMAMUX1_C4CR {
    pub use super::DMAMUX1_C0CR::DMAREQ_ID;
    pub use super::DMAMUX1_C0CR::EGE;
    pub use super::DMAMUX1_C0CR::NBREQ;
    pub use super::DMAMUX1_C0CR::SE;
    pub use super::DMAMUX1_C0CR::SOIE;
    pub use super::DMAMUX1_C0CR::SPOL;
    pub use super::DMAMUX1_C0CR::SYNC_ID;
}

/// DMAMux - DMA request line multiplexer channel x control register
pub mod DMAMUX1_C5CR {
    pub use super::DMAMUX1_C0CR::DMAREQ_ID;
    pub use super::DMAMUX1_C0CR::EGE;
    pub use super::DMAMUX1_C0CR::NBREQ;
    pub use super::DMAMUX1_C0CR::SE;
    pub use super::DMAMUX1_C0CR::SOIE;
    pub use super::DMAMUX1_C0CR::SPOL;
    pub use super::DMAMUX1_C0CR::SYNC_ID;
}

/// DMAMux - DMA request line multiplexer channel x control register
pub mod DMAMUX1_C6CR {
    pub use super::DMAMUX1_C0CR::DMAREQ_ID;
    pub use super::DMAMUX1_C0CR::EGE;
    pub use super::DMAMUX1_C0CR::NBREQ;
    pub use super::DMAMUX1_C0CR::SE;
    pub use super::DMAMUX1_C0CR::SOIE;
    pub use super::DMAMUX1_C0CR::SPOL;
    pub use super::DMAMUX1_C0CR::SYNC_ID;
}

/// DMAMux - DMA request line multiplexer channel x control register
pub mod DMAMUX1_C7CR {
    pub use super::DMAMUX1_C0CR::DMAREQ_ID;
    pub use super::DMAMUX1_C0CR::EGE;
    pub use super::DMAMUX1_C0CR::NBREQ;
    pub use super::DMAMUX1_C0CR::SE;
    pub use super::DMAMUX1_C0CR::SOIE;
    pub use super::DMAMUX1_C0CR::SPOL;
    pub use super::DMAMUX1_C0CR::SYNC_ID;
}

/// DMAMux - DMA request line multiplexer channel x control register
pub mod DMAMUX1_C8CR {
    pub use super::DMAMUX1_C0CR::DMAREQ_ID;
    pub use super::DMAMUX1_C0CR::EGE;
    pub use super::DMAMUX1_C0CR::NBREQ;
    pub use super::DMAMUX1_C0CR::SE;
    pub use super::DMAMUX1_C0CR::SOIE;
    pub use super::DMAMUX1_C0CR::SPOL;
    pub use super::DMAMUX1_C0CR::SYNC_ID;
}

/// DMAMux - DMA request line multiplexer channel x control register
pub mod DMAMUX1_C9CR {
    pub use super::DMAMUX1_C0CR::DMAREQ_ID;
    pub use super::DMAMUX1_C0CR::EGE;
    pub use super::DMAMUX1_C0CR::NBREQ;
    pub use super::DMAMUX1_C0CR::SE;
    pub use super::DMAMUX1_C0CR::SOIE;
    pub use super::DMAMUX1_C0CR::SPOL;
    pub use super::DMAMUX1_C0CR::SYNC_ID;
}

/// DMAMux - DMA request line multiplexer channel x control register
pub mod DMAMUX1_C10CR {
    pub use super::DMAMUX1_C0CR::DMAREQ_ID;
    pub use super::DMAMUX1_C0CR::EGE;
    pub use super::DMAMUX1_C0CR::NBREQ;
    pub use super::DMAMUX1_C0CR::SE;
    pub use super::DMAMUX1_C0CR::SOIE;
    pub use super::DMAMUX1_C0CR::SPOL;
    pub use super::DMAMUX1_C0CR::SYNC_ID;
}

/// DMAMux - DMA request line multiplexer channel x control register
pub mod DMAMUX1_C11CR {
    pub use super::DMAMUX1_C0CR::DMAREQ_ID;
    pub use super::DMAMUX1_C0CR::EGE;
    pub use super::DMAMUX1_C0CR::NBREQ;
    pub use super::DMAMUX1_C0CR::SE;
    pub use super::DMAMUX1_C0CR::SOIE;
    pub use super::DMAMUX1_C0CR::SPOL;
    pub use super::DMAMUX1_C0CR::SYNC_ID;
}

/// DMAMux - DMA request line multiplexer channel x control register
pub mod DMAMUX1_C12CR {
    pub use super::DMAMUX1_C0CR::DMAREQ_ID;
    pub use super::DMAMUX1_C0CR::EGE;
    pub use super::DMAMUX1_C0CR::NBREQ;
    pub use super::DMAMUX1_C0CR::SE;
    pub use super::DMAMUX1_C0CR::SOIE;
    pub use super::DMAMUX1_C0CR::SPOL;
    pub use super::DMAMUX1_C0CR::SYNC_ID;
}

/// DMAMux - DMA request line multiplexer channel x control register
pub mod DMAMUX1_C13CR {
    pub use super::DMAMUX1_C0CR::DMAREQ_ID;
    pub use super::DMAMUX1_C0CR::EGE;
    pub use super::DMAMUX1_C0CR::NBREQ;
    pub use super::DMAMUX1_C0CR::SE;
    pub use super::DMAMUX1_C0CR::SOIE;
    pub use super::DMAMUX1_C0CR::SPOL;
    pub use super::DMAMUX1_C0CR::SYNC_ID;
}

/// DMAMux - DMA request line multiplexer channel x control register
pub mod DMAMUX1_C14CR {
    pub use super::DMAMUX1_C0CR::DMAREQ_ID;
    pub use super::DMAMUX1_C0CR::EGE;
    pub use super::DMAMUX1_C0CR::NBREQ;
    pub use super::DMAMUX1_C0CR::SE;
    pub use super::DMAMUX1_C0CR::SOIE;
    pub use super::DMAMUX1_C0CR::SPOL;
    pub use super::DMAMUX1_C0CR::SYNC_ID;
}

/// DMAMux - DMA request line multiplexer channel x control register
pub mod DMAMUX1_C15CR {
    pub use super::DMAMUX1_C0CR::DMAREQ_ID;
    pub use super::DMAMUX1_C0CR::EGE;
    pub use super::DMAMUX1_C0CR::NBREQ;
    pub use super::DMAMUX1_C0CR::SE;
    pub use super::DMAMUX1_C0CR::SOIE;
    pub use super::DMAMUX1_C0CR::SPOL;
    pub use super::DMAMUX1_C0CR::SYNC_ID;
}

/// DMAMux - DMA request generator channel x control register
pub mod DMAMUX1_RG0CR {

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
pub mod DMAMUX1_RG1CR {
    pub use super::DMAMUX1_RG0CR::GE;
    pub use super::DMAMUX1_RG0CR::GNBREQ;
    pub use super::DMAMUX1_RG0CR::GPOL;
    pub use super::DMAMUX1_RG0CR::OIE;
    pub use super::DMAMUX1_RG0CR::SIG_ID;
}

/// DMAMux - DMA request generator channel x control register
pub mod DMAMUX1_RG2CR {
    pub use super::DMAMUX1_RG0CR::GE;
    pub use super::DMAMUX1_RG0CR::GNBREQ;
    pub use super::DMAMUX1_RG0CR::GPOL;
    pub use super::DMAMUX1_RG0CR::OIE;
    pub use super::DMAMUX1_RG0CR::SIG_ID;
}

/// DMAMux - DMA request generator channel x control register
pub mod DMAMUX1_RG3CR {
    pub use super::DMAMUX1_RG0CR::GE;
    pub use super::DMAMUX1_RG0CR::GNBREQ;
    pub use super::DMAMUX1_RG0CR::GPOL;
    pub use super::DMAMUX1_RG0CR::OIE;
    pub use super::DMAMUX1_RG0CR::SIG_ID;
}

/// DMAMux - DMA request generator channel x control register
pub mod DMAMUX1_RG4CR {
    pub use super::DMAMUX1_RG0CR::GE;
    pub use super::DMAMUX1_RG0CR::GNBREQ;
    pub use super::DMAMUX1_RG0CR::GPOL;
    pub use super::DMAMUX1_RG0CR::OIE;
    pub use super::DMAMUX1_RG0CR::SIG_ID;
}

/// DMAMux - DMA request generator channel x control register
pub mod DMAMUX1_RG5CR {
    pub use super::DMAMUX1_RG0CR::GE;
    pub use super::DMAMUX1_RG0CR::GNBREQ;
    pub use super::DMAMUX1_RG0CR::GPOL;
    pub use super::DMAMUX1_RG0CR::OIE;
    pub use super::DMAMUX1_RG0CR::SIG_ID;
}

/// DMAMux - DMA request generator channel x control register
pub mod DMAMUX1_RG6CR {
    pub use super::DMAMUX1_RG0CR::GE;
    pub use super::DMAMUX1_RG0CR::GNBREQ;
    pub use super::DMAMUX1_RG0CR::GPOL;
    pub use super::DMAMUX1_RG0CR::OIE;
    pub use super::DMAMUX1_RG0CR::SIG_ID;
}

/// DMAMux - DMA request generator channel x control register
pub mod DMAMUX1_RG7CR {
    pub use super::DMAMUX1_RG0CR::GE;
    pub use super::DMAMUX1_RG0CR::GNBREQ;
    pub use super::DMAMUX1_RG0CR::GPOL;
    pub use super::DMAMUX1_RG0CR::OIE;
    pub use super::DMAMUX1_RG0CR::SIG_ID;
}

/// DMAMux - DMA request generator status register
pub mod DMAMUX1_RGSR {

    /// Trigger event overrun flag The flag is set when a trigger event occurs on DMA request generator channel x, while the DMA request generator counter value is lower than GNBREQ. The flag is cleared by writing 1 to the corresponding COFx bit in DMAMUX_RGCFR register.
    pub mod OF {
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

/// DMAMux - DMA request generator clear flag register
pub mod DMAMUX1_RGCFR {

    /// Clear trigger event overrun flag Upon setting, this bit clears the corresponding overrun flag OFx in the DMAMUX_RGCSR register.
    pub mod COF {
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

/// DMAMUX request line multiplexer interrupt channel status register
pub mod DMAMUX1_CSR {

    /// Synchronization overrun event flag
    pub mod SOF {
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

/// DMAMUX request line multiplexer interrupt clear flag register
pub mod DMAMUX1_CFR {

    /// Clear synchronization overrun event flag
    pub mod CSOF {
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
pub struct RegisterBlock {
    /// DMAMux - DMA request line multiplexer channel x control register
    pub DMAMUX1_C0CR: RWRegister<u32>,

    /// DMAMux - DMA request line multiplexer channel x control register
    pub DMAMUX1_C1CR: RWRegister<u32>,

    /// DMAMux - DMA request line multiplexer channel x control register
    pub DMAMUX1_C2CR: RWRegister<u32>,

    /// DMAMux - DMA request line multiplexer channel x control register
    pub DMAMUX1_C3CR: RWRegister<u32>,

    /// DMAMux - DMA request line multiplexer channel x control register
    pub DMAMUX1_C4CR: RWRegister<u32>,

    /// DMAMux - DMA request line multiplexer channel x control register
    pub DMAMUX1_C5CR: RWRegister<u32>,

    /// DMAMux - DMA request line multiplexer channel x control register
    pub DMAMUX1_C6CR: RWRegister<u32>,

    /// DMAMux - DMA request line multiplexer channel x control register
    pub DMAMUX1_C7CR: RWRegister<u32>,

    /// DMAMux - DMA request line multiplexer channel x control register
    pub DMAMUX1_C8CR: RWRegister<u32>,

    /// DMAMux - DMA request line multiplexer channel x control register
    pub DMAMUX1_C9CR: RWRegister<u32>,

    /// DMAMux - DMA request line multiplexer channel x control register
    pub DMAMUX1_C10CR: RWRegister<u32>,

    /// DMAMux - DMA request line multiplexer channel x control register
    pub DMAMUX1_C11CR: RWRegister<u32>,

    /// DMAMux - DMA request line multiplexer channel x control register
    pub DMAMUX1_C12CR: RWRegister<u32>,

    /// DMAMux - DMA request line multiplexer channel x control register
    pub DMAMUX1_C13CR: RWRegister<u32>,

    /// DMAMux - DMA request line multiplexer channel x control register
    pub DMAMUX1_C14CR: RWRegister<u32>,

    /// DMAMux - DMA request line multiplexer channel x control register
    pub DMAMUX1_C15CR: RWRegister<u32>,

    _reserved1: [u32; 16],

    /// DMAMUX request line multiplexer interrupt channel status register
    pub DMAMUX1_CSR: RORegister<u32>,

    /// DMAMUX request line multiplexer interrupt clear flag register
    pub DMAMUX1_CFR: WORegister<u32>,

    _reserved2: [u32; 30],

    /// DMAMux - DMA request generator channel x control register
    pub DMAMUX1_RG0CR: RWRegister<u32>,

    /// DMAMux - DMA request generator channel x control register
    pub DMAMUX1_RG1CR: RWRegister<u32>,

    /// DMAMux - DMA request generator channel x control register
    pub DMAMUX1_RG2CR: RWRegister<u32>,

    /// DMAMux - DMA request generator channel x control register
    pub DMAMUX1_RG3CR: RWRegister<u32>,

    /// DMAMux - DMA request generator channel x control register
    pub DMAMUX1_RG4CR: RWRegister<u32>,

    /// DMAMux - DMA request generator channel x control register
    pub DMAMUX1_RG5CR: RWRegister<u32>,

    /// DMAMux - DMA request generator channel x control register
    pub DMAMUX1_RG6CR: RWRegister<u32>,

    /// DMAMux - DMA request generator channel x control register
    pub DMAMUX1_RG7CR: RWRegister<u32>,

    _reserved3: [u32; 8],

    /// DMAMux - DMA request generator status register
    pub DMAMUX1_RGSR: RORegister<u32>,

    /// DMAMux - DMA request generator clear flag register
    pub DMAMUX1_RGCFR: WORegister<u32>,
}
pub struct ResetValues {
    pub DMAMUX1_C0CR: u32,
    pub DMAMUX1_C1CR: u32,
    pub DMAMUX1_C2CR: u32,
    pub DMAMUX1_C3CR: u32,
    pub DMAMUX1_C4CR: u32,
    pub DMAMUX1_C5CR: u32,
    pub DMAMUX1_C6CR: u32,
    pub DMAMUX1_C7CR: u32,
    pub DMAMUX1_C8CR: u32,
    pub DMAMUX1_C9CR: u32,
    pub DMAMUX1_C10CR: u32,
    pub DMAMUX1_C11CR: u32,
    pub DMAMUX1_C12CR: u32,
    pub DMAMUX1_C13CR: u32,
    pub DMAMUX1_C14CR: u32,
    pub DMAMUX1_C15CR: u32,
    pub DMAMUX1_CSR: u32,
    pub DMAMUX1_CFR: u32,
    pub DMAMUX1_RG0CR: u32,
    pub DMAMUX1_RG1CR: u32,
    pub DMAMUX1_RG2CR: u32,
    pub DMAMUX1_RG3CR: u32,
    pub DMAMUX1_RG4CR: u32,
    pub DMAMUX1_RG5CR: u32,
    pub DMAMUX1_RG6CR: u32,
    pub DMAMUX1_RG7CR: u32,
    pub DMAMUX1_RGSR: u32,
    pub DMAMUX1_RGCFR: u32,
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

/// Access functions for the DMAMUX1 peripheral instance
pub mod DMAMUX1 {
    #[cfg(not(feature = "nosync"))]
    use external_cortex_m;

    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40020800,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in DMAMUX1
    pub const reset: ResetValues = ResetValues {
        DMAMUX1_C0CR: 0x00000000,
        DMAMUX1_C1CR: 0x00000000,
        DMAMUX1_C2CR: 0x00000000,
        DMAMUX1_C3CR: 0x00000000,
        DMAMUX1_C4CR: 0x00000000,
        DMAMUX1_C5CR: 0x00000000,
        DMAMUX1_C6CR: 0x00000000,
        DMAMUX1_C7CR: 0x00000000,
        DMAMUX1_C8CR: 0x00000000,
        DMAMUX1_C9CR: 0x00000000,
        DMAMUX1_C10CR: 0x00000000,
        DMAMUX1_C11CR: 0x00000000,
        DMAMUX1_C12CR: 0x00000000,
        DMAMUX1_C13CR: 0x00000000,
        DMAMUX1_C14CR: 0x00000000,
        DMAMUX1_C15CR: 0x00000000,
        DMAMUX1_RG0CR: 0x00000000,
        DMAMUX1_RG1CR: 0x00000000,
        DMAMUX1_RG2CR: 0x00000000,
        DMAMUX1_RG3CR: 0x00000000,
        DMAMUX1_RG4CR: 0x00000000,
        DMAMUX1_RG5CR: 0x00000000,
        DMAMUX1_RG6CR: 0x00000000,
        DMAMUX1_RG7CR: 0x00000000,
        DMAMUX1_RGSR: 0x00000000,
        DMAMUX1_RGCFR: 0x00000000,
        DMAMUX1_CSR: 0x00000000,
        DMAMUX1_CFR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[no_mangle]
    static mut DMAMUX1_TAKEN: bool = false;

    /// Safe access to DMAMUX1
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
            if DMAMUX1_TAKEN {
                None
            } else {
                DMAMUX1_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to DMAMUX1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if DMAMUX1_TAKEN && inst.addr == INSTANCE.addr {
                DMAMUX1_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }
}

/// Raw pointer to DMAMUX1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const DMAMUX1: *const RegisterBlock = 0x40020800 as *const _;
