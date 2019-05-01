#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! DMAMUX

#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;
use {RORegister, RWRegister, WORegister};

/// DMAMux - DMA request line multiplexer channel x control register
pub mod CCR0 {

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
pub mod CCR1 {
    pub use super::CCR0::DMAREQ_ID;
    pub use super::CCR0::EGE;
    pub use super::CCR0::NBREQ;
    pub use super::CCR0::SE;
    pub use super::CCR0::SOIE;
    pub use super::CCR0::SPOL;
    pub use super::CCR0::SYNC_ID;
}

/// DMAMux - DMA request line multiplexer channel x control register
pub mod CCR2 {
    pub use super::CCR0::DMAREQ_ID;
    pub use super::CCR0::EGE;
    pub use super::CCR0::NBREQ;
    pub use super::CCR0::SE;
    pub use super::CCR0::SOIE;
    pub use super::CCR0::SPOL;
    pub use super::CCR0::SYNC_ID;
}

/// DMAMux - DMA request line multiplexer channel x control register
pub mod CCR3 {
    pub use super::CCR0::DMAREQ_ID;
    pub use super::CCR0::EGE;
    pub use super::CCR0::NBREQ;
    pub use super::CCR0::SE;
    pub use super::CCR0::SOIE;
    pub use super::CCR0::SPOL;
    pub use super::CCR0::SYNC_ID;
}

/// DMAMux - DMA request line multiplexer channel x control register
pub mod CCR4 {
    pub use super::CCR0::DMAREQ_ID;
    pub use super::CCR0::EGE;
    pub use super::CCR0::NBREQ;
    pub use super::CCR0::SE;
    pub use super::CCR0::SOIE;
    pub use super::CCR0::SPOL;
    pub use super::CCR0::SYNC_ID;
}

/// DMAMux - DMA request line multiplexer channel x control register
pub mod CCR5 {
    pub use super::CCR0::DMAREQ_ID;
    pub use super::CCR0::EGE;
    pub use super::CCR0::NBREQ;
    pub use super::CCR0::SE;
    pub use super::CCR0::SOIE;
    pub use super::CCR0::SPOL;
    pub use super::CCR0::SYNC_ID;
}

/// DMAMux - DMA request line multiplexer channel x control register
pub mod CCR6 {
    pub use super::CCR0::DMAREQ_ID;
    pub use super::CCR0::EGE;
    pub use super::CCR0::NBREQ;
    pub use super::CCR0::SE;
    pub use super::CCR0::SOIE;
    pub use super::CCR0::SPOL;
    pub use super::CCR0::SYNC_ID;
}

/// DMAMux - DMA request line multiplexer channel x control register
pub mod CCR7 {
    pub use super::CCR0::DMAREQ_ID;
    pub use super::CCR0::EGE;
    pub use super::CCR0::NBREQ;
    pub use super::CCR0::SE;
    pub use super::CCR0::SOIE;
    pub use super::CCR0::SPOL;
    pub use super::CCR0::SYNC_ID;
}

/// DMAMux - DMA request line multiplexer channel x control register
pub mod CCR8 {
    pub use super::CCR0::DMAREQ_ID;
    pub use super::CCR0::EGE;
    pub use super::CCR0::NBREQ;
    pub use super::CCR0::SE;
    pub use super::CCR0::SOIE;
    pub use super::CCR0::SPOL;
    pub use super::CCR0::SYNC_ID;
}

/// DMAMux - DMA request line multiplexer channel x control register
pub mod CCR9 {
    pub use super::CCR0::DMAREQ_ID;
    pub use super::CCR0::EGE;
    pub use super::CCR0::NBREQ;
    pub use super::CCR0::SE;
    pub use super::CCR0::SOIE;
    pub use super::CCR0::SPOL;
    pub use super::CCR0::SYNC_ID;
}

/// DMAMux - DMA request line multiplexer channel x control register
pub mod CCR10 {
    pub use super::CCR0::DMAREQ_ID;
    pub use super::CCR0::EGE;
    pub use super::CCR0::NBREQ;
    pub use super::CCR0::SE;
    pub use super::CCR0::SOIE;
    pub use super::CCR0::SPOL;
    pub use super::CCR0::SYNC_ID;
}

/// DMAMux - DMA request line multiplexer channel x control register
pub mod CCR11 {
    pub use super::CCR0::DMAREQ_ID;
    pub use super::CCR0::EGE;
    pub use super::CCR0::NBREQ;
    pub use super::CCR0::SE;
    pub use super::CCR0::SOIE;
    pub use super::CCR0::SPOL;
    pub use super::CCR0::SYNC_ID;
}

/// DMAMux - DMA request line multiplexer channel x control register
pub mod CCR12 {
    pub use super::CCR0::DMAREQ_ID;
    pub use super::CCR0::EGE;
    pub use super::CCR0::NBREQ;
    pub use super::CCR0::SE;
    pub use super::CCR0::SOIE;
    pub use super::CCR0::SPOL;
    pub use super::CCR0::SYNC_ID;
}

/// DMAMux - DMA request line multiplexer channel x control register
pub mod CCR13 {
    pub use super::CCR0::DMAREQ_ID;
    pub use super::CCR0::EGE;
    pub use super::CCR0::NBREQ;
    pub use super::CCR0::SE;
    pub use super::CCR0::SOIE;
    pub use super::CCR0::SPOL;
    pub use super::CCR0::SYNC_ID;
}

/// DMAMux - DMA request line multiplexer channel x control register
pub mod CCR14 {
    pub use super::CCR0::DMAREQ_ID;
    pub use super::CCR0::EGE;
    pub use super::CCR0::NBREQ;
    pub use super::CCR0::SE;
    pub use super::CCR0::SOIE;
    pub use super::CCR0::SPOL;
    pub use super::CCR0::SYNC_ID;
}

/// DMAMux - DMA request line multiplexer channel x control register
pub mod CCR15 {
    pub use super::CCR0::DMAREQ_ID;
    pub use super::CCR0::EGE;
    pub use super::CCR0::NBREQ;
    pub use super::CCR0::SE;
    pub use super::CCR0::SOIE;
    pub use super::CCR0::SPOL;
    pub use super::CCR0::SYNC_ID;
}

/// DMAMux - DMA request generator channel x control register
pub mod RGCR0 {

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
pub mod RGCR1 {
    pub use super::RGCR0::GE;
    pub use super::RGCR0::GNBREQ;
    pub use super::RGCR0::GPOL;
    pub use super::RGCR0::OIE;
    pub use super::RGCR0::SIG_ID;
}

/// DMAMux - DMA request generator channel x control register
pub mod RGCR2 {
    pub use super::RGCR0::GE;
    pub use super::RGCR0::GNBREQ;
    pub use super::RGCR0::GPOL;
    pub use super::RGCR0::OIE;
    pub use super::RGCR0::SIG_ID;
}

/// DMAMux - DMA request generator channel x control register
pub mod RGCR3 {
    pub use super::RGCR0::GE;
    pub use super::RGCR0::GNBREQ;
    pub use super::RGCR0::GPOL;
    pub use super::RGCR0::OIE;
    pub use super::RGCR0::SIG_ID;
}

/// DMAMux - DMA request generator channel x control register
pub mod RGCR4 {
    pub use super::RGCR0::GE;
    pub use super::RGCR0::GNBREQ;
    pub use super::RGCR0::GPOL;
    pub use super::RGCR0::OIE;
    pub use super::RGCR0::SIG_ID;
}

/// DMAMux - DMA request generator channel x control register
pub mod RGCR5 {
    pub use super::RGCR0::GE;
    pub use super::RGCR0::GNBREQ;
    pub use super::RGCR0::GPOL;
    pub use super::RGCR0::OIE;
    pub use super::RGCR0::SIG_ID;
}

/// DMAMux - DMA request generator channel x control register
pub mod RGCR6 {
    pub use super::RGCR0::GE;
    pub use super::RGCR0::GNBREQ;
    pub use super::RGCR0::GPOL;
    pub use super::RGCR0::OIE;
    pub use super::RGCR0::SIG_ID;
}

/// DMAMux - DMA request generator channel x control register
pub mod RGCR7 {
    pub use super::RGCR0::GE;
    pub use super::RGCR0::GNBREQ;
    pub use super::RGCR0::GPOL;
    pub use super::RGCR0::OIE;
    pub use super::RGCR0::SIG_ID;
}

/// DMAMux - DMA request generator status register
pub mod RGSR {

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
pub mod RGCFR {

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
pub mod CSR {

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
pub mod CFR {

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
    pub CCR0: RWRegister<u32>,

    /// DMAMux - DMA request line multiplexer channel x control register
    pub CCR1: RWRegister<u32>,

    /// DMAMux - DMA request line multiplexer channel x control register
    pub CCR2: RWRegister<u32>,

    /// DMAMux - DMA request line multiplexer channel x control register
    pub CCR3: RWRegister<u32>,

    /// DMAMux - DMA request line multiplexer channel x control register
    pub CCR4: RWRegister<u32>,

    /// DMAMux - DMA request line multiplexer channel x control register
    pub CCR5: RWRegister<u32>,

    /// DMAMux - DMA request line multiplexer channel x control register
    pub CCR6: RWRegister<u32>,

    /// DMAMux - DMA request line multiplexer channel x control register
    pub CCR7: RWRegister<u32>,

    /// DMAMux - DMA request line multiplexer channel x control register
    pub CCR8: RWRegister<u32>,

    /// DMAMux - DMA request line multiplexer channel x control register
    pub CCR9: RWRegister<u32>,

    /// DMAMux - DMA request line multiplexer channel x control register
    pub CCR10: RWRegister<u32>,

    /// DMAMux - DMA request line multiplexer channel x control register
    pub CCR11: RWRegister<u32>,

    /// DMAMux - DMA request line multiplexer channel x control register
    pub CCR12: RWRegister<u32>,

    /// DMAMux - DMA request line multiplexer channel x control register
    pub CCR13: RWRegister<u32>,

    /// DMAMux - DMA request line multiplexer channel x control register
    pub CCR14: RWRegister<u32>,

    /// DMAMux - DMA request line multiplexer channel x control register
    pub CCR15: RWRegister<u32>,

    _reserved1: [u32; 16],

    /// DMAMUX request line multiplexer interrupt channel status register
    pub CSR: RORegister<u32>,

    /// DMAMUX request line multiplexer interrupt clear flag register
    pub CFR: WORegister<u32>,

    _reserved2: [u32; 30],

    /// DMAMux - DMA request generator channel x control register
    pub RGCR0: RWRegister<u32>,

    /// DMAMux - DMA request generator channel x control register
    pub RGCR1: RWRegister<u32>,

    /// DMAMux - DMA request generator channel x control register
    pub RGCR2: RWRegister<u32>,

    /// DMAMux - DMA request generator channel x control register
    pub RGCR3: RWRegister<u32>,

    /// DMAMux - DMA request generator channel x control register
    pub RGCR4: RWRegister<u32>,

    /// DMAMux - DMA request generator channel x control register
    pub RGCR5: RWRegister<u32>,

    /// DMAMux - DMA request generator channel x control register
    pub RGCR6: RWRegister<u32>,

    /// DMAMux - DMA request generator channel x control register
    pub RGCR7: RWRegister<u32>,

    _reserved3: [u32; 8],

    /// DMAMux - DMA request generator status register
    pub RGSR: RORegister<u32>,

    /// DMAMux - DMA request generator clear flag register
    pub RGCFR: WORegister<u32>,
}
pub struct ResetValues {
    pub CCR0: u32,
    pub CCR1: u32,
    pub CCR2: u32,
    pub CCR3: u32,
    pub CCR4: u32,
    pub CCR5: u32,
    pub CCR6: u32,
    pub CCR7: u32,
    pub CCR8: u32,
    pub CCR9: u32,
    pub CCR10: u32,
    pub CCR11: u32,
    pub CCR12: u32,
    pub CCR13: u32,
    pub CCR14: u32,
    pub CCR15: u32,
    pub CSR: u32,
    pub CFR: u32,
    pub RGCR0: u32,
    pub RGCR1: u32,
    pub RGCR2: u32,
    pub RGCR3: u32,
    pub RGCR4: u32,
    pub RGCR5: u32,
    pub RGCR6: u32,
    pub RGCR7: u32,
    pub RGSR: u32,
    pub RGCFR: u32,
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
        CCR0: 0x00000000,
        CCR1: 0x00000000,
        CCR2: 0x00000000,
        CCR3: 0x00000000,
        CCR4: 0x00000000,
        CCR5: 0x00000000,
        CCR6: 0x00000000,
        CCR7: 0x00000000,
        CCR8: 0x00000000,
        CCR9: 0x00000000,
        CCR10: 0x00000000,
        CCR11: 0x00000000,
        CCR12: 0x00000000,
        CCR13: 0x00000000,
        CCR14: 0x00000000,
        CCR15: 0x00000000,
        RGCR0: 0x00000000,
        RGCR1: 0x00000000,
        RGCR2: 0x00000000,
        RGCR3: 0x00000000,
        RGCR4: 0x00000000,
        RGCR5: 0x00000000,
        RGCR6: 0x00000000,
        RGCR7: 0x00000000,
        RGSR: 0x00000000,
        RGCFR: 0x00000000,
        CSR: 0x00000000,
        CFR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
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
