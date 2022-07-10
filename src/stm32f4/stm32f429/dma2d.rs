#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! DMA2D controller

use crate::{RORegister, RWRegister, UnsafeRWRegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// control register
pub mod CR {

    /// DMA2D mode
    pub mod MODE {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (2 bits: 0b11 << 16)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Configuration Error Interrupt Enable
    pub mod CEIE {
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

    /// CLUT transfer complete interrupt enable
    pub mod CTCIE {
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

    /// CLUT access error interrupt enable
    pub mod CAEIE {
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

    /// Transfer watermark interrupt enable
    pub mod TWIE {
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

    /// Transfer complete interrupt enable
    pub mod TCIE {
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

    /// Transfer error interrupt enable
    pub mod TEIE {
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

    /// Abort
    pub mod ABORT {
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

    /// Suspend
    pub mod SUSP {
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

    /// Start
    pub mod START {
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
}

/// Interrupt Status Register
pub mod ISR {

    /// Configuration error interrupt flag
    pub mod CEIF {
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

    /// CLUT transfer complete interrupt flag
    pub mod CTCIF {
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

    /// CLUT access error interrupt flag
    pub mod CAEIF {
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

    /// Transfer watermark interrupt flag
    pub mod TWIF {
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

    /// Transfer complete interrupt flag
    pub mod TCIF {
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

    /// Transfer error interrupt flag
    pub mod TEIF {
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
}

/// interrupt flag clear register
pub mod IFCR {

    /// Clear configuration error interrupt flag
    pub mod CCEIF {
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

    /// Clear CLUT transfer complete interrupt flag
    pub mod CCTCIF {
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

    /// Clear CLUT access error interrupt flag
    pub mod CAECIF {
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

    /// Clear transfer watermark interrupt flag
    pub mod CTWIF {
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

    /// Clear transfer complete interrupt flag
    pub mod CTCIF {
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

    /// Clear Transfer error interrupt flag
    pub mod CTEIF {
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
}

/// foreground memory address register
pub mod FGMAR {

    /// Memory address
    pub mod MA {
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

/// foreground offset register
pub mod FGOR {

    /// Line offset
    pub mod LO {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (14 bits: 0x3fff << 0)
        pub const mask: u32 = 0x3fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// background memory address register
pub mod BGMAR {
    pub use super::FGMAR::MA;
}

/// background offset register
pub mod BGOR {
    pub use super::FGOR::LO;
}

/// foreground PFC control register
pub mod FGPFCCR {

    /// Alpha value
    pub mod ALPHA {
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

    /// Alpha mode
    pub mod AM {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (2 bits: 0b11 << 16)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CLUT size
    pub mod CS {
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

    /// Start
    pub mod START {
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

    /// CLUT color mode
    pub mod CCM {
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

    /// Color mode
    pub mod CM {
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

/// foreground color register
pub mod FGCOLR {

    /// Red Value
    pub mod RED {
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

    /// Green Value
    pub mod GREEN {
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

    /// Blue Value
    pub mod BLUE {
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

/// background PFC control register
pub mod BGPFCCR {
    pub use super::FGPFCCR::ALPHA;
    pub use super::FGPFCCR::AM;
    pub use super::FGPFCCR::CCM;
    pub use super::FGPFCCR::CM;
    pub use super::FGPFCCR::CS;
    pub use super::FGPFCCR::START;
}

/// background color register
pub mod BGCOLR {
    pub use super::FGCOLR::BLUE;
    pub use super::FGCOLR::GREEN;
    pub use super::FGCOLR::RED;
}

/// foreground CLUT memory address register
pub mod FGCMAR {
    pub use super::FGMAR::MA;
}

/// background CLUT memory address register
pub mod BGCMAR {
    pub use super::FGMAR::MA;
}

/// output PFC control register
pub mod OPFCCR {

    /// Color mode
    pub mod CM {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// output color register
pub mod OCOLR {

    /// Alpha Channel Value
    pub mod APLHA {
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

    /// Red Value
    pub mod RED {
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

    /// Green Value
    pub mod GREEN {
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

    /// Blue Value
    pub mod BLUE {
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

/// output memory address register
pub mod OMAR {
    pub use super::FGMAR::MA;
}

/// output offset register
pub mod OOR {
    pub use super::FGOR::LO;
}

/// number of line register
pub mod NLR {

    /// Pixel per lines
    pub mod PL {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (14 bits: 0x3fff << 16)
        pub const mask: u32 = 0x3fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Number of lines
    pub mod NL {
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

/// line watermark register
pub mod LWR {

    /// Line watermark
    pub mod LW {
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

/// AHB master timer configuration register
pub mod AMTCR {

    /// Dead Time
    pub mod DT {
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

    /// Enable
    pub mod EN {
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
}

/// FGCLUT
pub mod FGCLUT {
    pub use super::OCOLR::APLHA;
    pub use super::OCOLR::BLUE;
    pub use super::OCOLR::GREEN;
    pub use super::OCOLR::RED;
}

/// BGCLUT
pub mod BGCLUT {
    pub use super::OCOLR::APLHA;
    pub use super::OCOLR::BLUE;
    pub use super::OCOLR::GREEN;
    pub use super::OCOLR::RED;
}
#[repr(C)]
pub struct RegisterBlock {
    /// control register
    pub CR: RWRegister<u32>,

    /// Interrupt Status Register
    pub ISR: RORegister<u32>,

    /// interrupt flag clear register
    pub IFCR: RWRegister<u32>,

    /// foreground memory address register
    pub FGMAR: UnsafeRWRegister<u32>,

    /// foreground offset register
    pub FGOR: RWRegister<u32>,

    /// background memory address register
    pub BGMAR: UnsafeRWRegister<u32>,

    /// background offset register
    pub BGOR: RWRegister<u32>,

    /// foreground PFC control register
    pub FGPFCCR: RWRegister<u32>,

    /// foreground color register
    pub FGCOLR: RWRegister<u32>,

    /// background PFC control register
    pub BGPFCCR: RWRegister<u32>,

    /// background color register
    pub BGCOLR: RWRegister<u32>,

    /// foreground CLUT memory address register
    pub FGCMAR: UnsafeRWRegister<u32>,

    /// background CLUT memory address register
    pub BGCMAR: UnsafeRWRegister<u32>,

    /// output PFC control register
    pub OPFCCR: RWRegister<u32>,

    /// output color register
    pub OCOLR: RWRegister<u32>,

    /// output memory address register
    pub OMAR: UnsafeRWRegister<u32>,

    /// output offset register
    pub OOR: RWRegister<u32>,

    /// number of line register
    pub NLR: RWRegister<u32>,

    /// line watermark register
    pub LWR: RWRegister<u32>,

    /// AHB master timer configuration register
    pub AMTCR: RWRegister<u32>,

    _reserved1: [u8; 944],

    /// FGCLUT
    pub FGCLUT: RWRegister<u32>,

    _reserved2: [u8; 1020],

    /// BGCLUT
    pub BGCLUT: RWRegister<u32>,
}
pub struct ResetValues {
    pub CR: u32,
    pub ISR: u32,
    pub IFCR: u32,
    pub FGMAR: u32,
    pub FGOR: u32,
    pub BGMAR: u32,
    pub BGOR: u32,
    pub FGPFCCR: u32,
    pub FGCOLR: u32,
    pub BGPFCCR: u32,
    pub BGCOLR: u32,
    pub FGCMAR: u32,
    pub BGCMAR: u32,
    pub OPFCCR: u32,
    pub OCOLR: u32,
    pub OMAR: u32,
    pub OOR: u32,
    pub NLR: u32,
    pub LWR: u32,
    pub AMTCR: u32,
    pub FGCLUT: u32,
    pub BGCLUT: u32,
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

/// Access functions for the DMA2D peripheral instance
pub mod DMA2D {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x4002b000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in DMA2D
    pub const reset: ResetValues = ResetValues {
        CR: 0x00000000,
        ISR: 0x00000000,
        IFCR: 0x00000000,
        FGMAR: 0x00000000,
        FGOR: 0x00000000,
        BGMAR: 0x00000000,
        BGOR: 0x00000000,
        FGPFCCR: 0x00000000,
        FGCOLR: 0x00000000,
        BGPFCCR: 0x00000000,
        BGCOLR: 0x00000000,
        FGCMAR: 0x00000000,
        BGCMAR: 0x00000000,
        OPFCCR: 0x00000000,
        OCOLR: 0x00000000,
        OMAR: 0x00000000,
        OOR: 0x00000000,
        NLR: 0x00000000,
        LWR: 0x00000000,
        AMTCR: 0x00000000,
        FGCLUT: 0x00000000,
        BGCLUT: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut DMA2D_TAKEN: bool = false;

    /// Safe access to DMA2D
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
            if DMA2D_TAKEN {
                None
            } else {
                DMA2D_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to DMA2D
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if DMA2D_TAKEN && inst.addr == INSTANCE.addr {
                DMA2D_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal DMA2D
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        DMA2D_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to DMA2D
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const DMA2D: *const RegisterBlock = 0x4002b000 as *const _;
