#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! System window watchdog
//!
//! Used by: stm32g030, stm32g031, stm32g041, stm32g050, stm32g051, stm32g061, stm32g070, stm32g071, stm32g081, stm32g0b0, stm32g0b1, stm32g0c1

use crate::RWRegister;
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// Control register
pub mod CR {

    /// Activation bit
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
            pub const Disabled: u32 = 0b0;

            /// 0b1: Watchdog enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// 7-bit counter (MSB to LSB)
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
}

/// Configuration register
pub mod CFR {

    /// Early wakeup interrupt
    pub mod EWI {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: interrupt occurs whenever the counter reaches the value 0x40
            pub const Enable: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// 7-bit window value
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

    /// Timer base
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

            /// 0b000: Counter clock (PCLK1 div 4096) div 1
            pub const Div1: u32 = 0b000;

            /// 0b001: Counter clock (PCLK1 div 4096) div 2
            pub const Div2: u32 = 0b001;

            /// 0b010: Counter clock (PCLK1 div 4096) div 4
            pub const Div4: u32 = 0b010;

            /// 0b011: Counter clock (PCLK1 div 4096) div 8
            pub const Div8: u32 = 0b011;

            /// 0b100: Counter clock (PCLK1 div 4096) div 16
            pub const Div16: u32 = 0b100;

            /// 0b101: Counter clock (PCLK1 div 4096) div 32
            pub const Div32: u32 = 0b101;

            /// 0b110: Counter clock (PCLK1 div 4096) div 64
            pub const Div64: u32 = 0b110;

            /// 0b111: Counter clock (PCLK1 div 4096) div 128
            pub const Div128: u32 = 0b111;
        }
    }
}

/// Status register
pub mod SR {

    /// Early wakeup interrupt flag
    pub mod EWIF {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: The EWI Interrupt Service Routine has been serviced
            pub const Finished: u32 = 0b0;

            /// 0b1: The EWI Interrupt Service Routine has been triggered
            pub const Pending: u32 = 0b1;
        }
        /// Write-only values
        pub mod W {

            /// 0b0: The EWI Interrupt Service Routine has been serviced
            pub const Finished: u32 = 0b0;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }
}
#[repr(C)]
pub struct RegisterBlock {
    /// Control register
    pub CR: RWRegister<u32>,

    /// Configuration register
    pub CFR: RWRegister<u32>,

    /// Status register
    pub SR: RWRegister<u32>,
}
pub struct ResetValues {
    pub CR: u32,
    pub CFR: u32,
    pub SR: u32,
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
