#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Independent watchdog
//!
//! Used by: stm32f7x5, stm32f7x6, stm32f7x7, stm32f7x9

#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;
use {RORegister, RWRegister, WORegister};

/// Key register
pub mod KR {

    /// Key value (write only, read 0000h)
    pub mod KEY {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0101010101010101: Enable access to PR, RLR and WINR registers (0x5555)
            pub const Enable: u32 = 0b0101010101010101;

            /// 0b1010101010101010: Reset the watchdog value (0xAAAA)
            pub const Reset: u32 = 0b1010101010101010;

            /// 0b1100110011001100: Start the watchdog (0xCCCC)
            pub const Start: u32 = 0b1100110011001100;
        }
    }
}

/// Prescaler register
pub mod PR {

    /// Prescaler divider
    pub mod PR {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Divider /4
            pub const DivideBy4: u32 = 0b000;

            /// 0b001: Divider /8
            pub const DivideBy8: u32 = 0b001;

            /// 0b010: Divider /16
            pub const DivideBy16: u32 = 0b010;

            /// 0b011: Divider /32
            pub const DivideBy32: u32 = 0b011;

            /// 0b100: Divider /64
            pub const DivideBy64: u32 = 0b100;

            /// 0b101: Divider /128
            pub const DivideBy128: u32 = 0b101;

            /// 0b110: Divider /256
            pub const DivideBy256: u32 = 0b110;

            /// 0b111: Divider /256
            pub const DivideBy256bis: u32 = 0b111;
        }
    }
}

/// Reload register
pub mod RLR {

    /// Watchdog counter reload value
    pub mod RL {
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

/// Status register
pub mod SR {

    /// Watchdog counter reload value update
    pub mod RVU {
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

    /// Watchdog prescaler value update
    pub mod PVU {
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

/// Window register
pub mod WINR {

    /// Watchdog counter window value
    pub mod WIN {
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
pub struct RegisterBlock {
    /// Key register
    pub KR: WORegister<u32>,

    /// Prescaler register
    pub PR: RWRegister<u32>,

    /// Reload register
    pub RLR: RWRegister<u32>,

    /// Status register
    pub SR: RORegister<u32>,

    /// Window register
    pub WINR: RWRegister<u32>,
}
pub struct ResetValues {
    pub KR: u32,
    pub PR: u32,
    pub RLR: u32,
    pub SR: u32,
    pub WINR: u32,
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
