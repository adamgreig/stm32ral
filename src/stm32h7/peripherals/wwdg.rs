#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! WWDG
//!
//! Used by: stm32h743, stm32h743v, stm32h747cm4, stm32h747cm7, stm32h753, stm32h753v

use crate::RWRegister;
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// Control register
pub mod CR {

    /// 7-bit counter (MSB to LSB) These bits contain the value of the watchdog counter. It is decremented every (4096 x 2WDGTB\[1:0\]) PCLK cycles. A reset is produced when it is decremented from 0x40 to 0x3F (T6 becomes cleared).
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

    /// Activation bit This bit is set by software and only cleared by hardware after a reset. When WDGA=1, the watchdog can generate a reset.
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
}

/// Configuration register
pub mod CFR {

    /// 7-bit window value These bits contain the window value to be compared to the downcounter.
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
        /// Write-only values
        pub mod W {

            /// 0b1: interrupt occurs whenever the counter reaches the value 0x40
            pub const Enable: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Timer base The time base of the prescaler can be modified as follows:
    pub mod WDGTB {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (2 bits: 0b11 << 11)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Counter clock (PCLK1 div 4096) div 1
            pub const Div1: u32 = 0b00;

            /// 0b01: Counter clock (PCLK1 div 4096) div 2
            pub const Div2: u32 = 0b01;

            /// 0b10: Counter clock (PCLK1 div 4096) div 4
            pub const Div4: u32 = 0b10;

            /// 0b11: Counter clock (PCLK1 div 4096) div 8
            pub const Div8: u32 = 0b11;
        }
    }
}

/// Status register
pub mod SR {

    /// Early wakeup interrupt flag This bit is set by hardware when the counter has reached the value 0x40. It must be cleared by software by writing 0. A write of 1 has no effect. This bit is also set if the interrupt is not enabled.
    pub mod EWIF {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b1: The EWI Interrupt Service Routine has been triggered
            pub const Pending: u32 = 0b1;

            /// 0b0: The EWI Interrupt Service Routine has been serviced
            pub const Finished: u32 = 0b0;
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
