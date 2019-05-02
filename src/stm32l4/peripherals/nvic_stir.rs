#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Nested vectored interrupt controller
//!
//! Used by: stm32l4x1, stm32l4x2, stm32l4x3, stm32l4x5, stm32l4x6

#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;
use RWRegister;

/// Software trigger interrupt register
pub mod STIR {

    /// Software generated interrupt ID
    pub mod INTID {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (9 bits: 0x1ff << 0)
        pub const mask: u32 = 0x1ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}
pub struct RegisterBlock {
    /// Software trigger interrupt register
    pub STIR: RWRegister<u32>,
}
pub struct ResetValues {
    pub STIR: u32,
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
