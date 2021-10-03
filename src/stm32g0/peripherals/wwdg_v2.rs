#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! System window watchdog
//!
//! Used by: stm32g050, stm32g051, stm32g061, stm32g0b0, stm32g0b1, stm32g0c1

use crate::RWRegister;
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// Control register
pub mod WWDG_CR {

    /// 7-bit counter (MSB to LSB) These bits contain the value of the watchdog counter, decremented every (4096 x 2WDGTB\[1:0\]) PCLK cycles. A reset is produced when it is decremented from 0x40 to 0x3F (T6 becomes cleared).
    pub mod T {
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

    /// Activation bit This bit is set by software and only cleared by hardware after a reset. When WDGAÂ =Â 1, the watchdog can generate a reset.
    pub mod WDGA {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Watchdog disabled
            pub const B_0x0: u32 = 0b0;

            /// 0b1: Watchdog enabled
            pub const B_0x1: u32 = 0b1;
        }
    }
}

/// Configuration register
pub mod WWDG_CFR {

    /// 7-bit window value These bits contain the window value to be compared with the down-counter.
    pub mod W {
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

    /// Early wakeup interrupt When set, an interrupt occurs whenever the counter reaches the value 0x40. This interrupt is only cleared by hardware after a reset.
    pub mod EWI {
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

    /// Timer base The timebase of the prescaler can be modified as follows:
    pub mod WDGTB {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (3 bits: 0b111 << 11)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: CK Counter Clock (PCLK div 4096) div 1
            pub const B_0x0: u32 = 0b000;

            /// 0b001: CK Counter Clock (PCLK div 4096) div 2
            pub const B_0x1: u32 = 0b001;

            /// 0b010: CK Counter Clock (PCLK div 4096) div 4
            pub const B_0x2: u32 = 0b010;

            /// 0b011: CK Counter Clock (PCLK div 4096) div 8
            pub const B_0x3: u32 = 0b011;

            /// 0b100: CK Counter Clock (PCLK div 4096) div 16
            pub const B_0x4: u32 = 0b100;

            /// 0b101: CK Counter Clock (PCLK div 4096) div 32
            pub const B_0x5: u32 = 0b101;

            /// 0b110: CK Counter Clock (PCLK div 4096) div 64
            pub const B_0x6: u32 = 0b110;

            /// 0b111: CK Counter Clock (PCLK div 4096) div 128
            pub const B_0x7: u32 = 0b111;
        }
    }
}

/// Status register
pub mod WWDG_SR {

    /// Early wakeup interrupt flag
    pub mod EWIF {
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
#[repr(C)]
pub struct RegisterBlock {
    /// Control register
    pub WWDG_CR: RWRegister<u32>,

    /// Configuration register
    pub WWDG_CFR: RWRegister<u32>,

    /// Status register
    pub WWDG_SR: RWRegister<u32>,
}
pub struct ResetValues {
    pub WWDG_CR: u32,
    pub WWDG_CFR: u32,
    pub WWDG_SR: u32,
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
