#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! CORDIC Co-processor
//!
//! Used by: stm32g431, stm32g441, stm32g471, stm32g473, stm32g474, stm32g483, stm32g484, stm32g491, stm32g4a1

use crate::{RORegister, RWRegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// CORDIC Control Status register
pub mod CSR {

    /// FUNC
    pub mod FUNC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Cosine funciton
            pub const Cosine: u32 = 0b0000;

            /// 0b0001: Sine function
            pub const Sine: u32 = 0b0001;

            /// 0b0010: Phase function
            pub const Phase: u32 = 0b0010;

            /// 0b0011: Modulus function
            pub const Modulus: u32 = 0b0011;

            /// 0b0100: Arctangent function
            pub const Arctangent: u32 = 0b0100;

            /// 0b0101: Hyperbolic Cosine function
            pub const HyperbolicCosine: u32 = 0b0101;

            /// 0b0110: Hyperbolic Sine function
            pub const HyperbolicSine: u32 = 0b0110;

            /// 0b0111: Arctanh function
            pub const Arctanh: u32 = 0b0111;

            /// 0b1000: Natural Logarithm function
            pub const NaturalLogarithm: u32 = 0b1000;

            /// 0b1001: Square Root function
            pub const SquareRoot: u32 = 0b1001;
        }
    }

    /// Precision (number of iterations/cycles) required
    pub mod PRECISION {
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

    /// Scaling factor (2^-n for arguments, 2^n for results)
    pub mod SCALE {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (3 bits: 0b111 << 8)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// IEN
    pub mod IEN {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Disable interrupt request generation
            pub const Disabled: u32 = 0b0;

            /// 0b1: Enable intterrupt request generation
            pub const Enabled: u32 = 0b1;
        }
    }

    /// DMAREN
    pub mod DMAREN {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No DMA channel reads are generated
            pub const Disabled: u32 = 0b0;

            /// 0b1: Read requests are generated on the DMA channel when RRDY flag is set
            pub const Enabled: u32 = 0b1;
        }
    }

    /// DMAWEN
    pub mod DMAWEN {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No DMA channel writes are generated
            pub const Disabled: u32 = 0b0;

            /// 0b1: Write requests are generated on the DMA channel when no operation is pending
            pub const Enabled: u32 = 0b1;
        }
    }

    /// NRES
    pub mod NRES {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Only single result value will be returned. After a single read RRDY will be automatically cleared
            pub const Num1: u32 = 0b0;

            /// 0b1: Two return reads need to be performed. After two reads RRDY will be automatically cleared
            pub const Num2: u32 = 0b1;
        }
    }

    /// NARGS
    pub mod NARGS {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Only single argument write is needed for next calculation
            pub const Num1: u32 = 0b0;

            /// 0b1: Two argument writes need to be performed for next calculation
            pub const Num2: u32 = 0b1;
        }
    }

    /// RESSIZE
    pub mod RESSIZE {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Use 32 bit output values
            pub const Bits32: u32 = 0b0;

            /// 0b1: Use 16 bit output values
            pub const Bits16: u32 = 0b1;
        }
    }

    /// ARGSIZE
    pub mod ARGSIZE {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Use 32 bit input values
            pub const Bits32: u32 = 0b0;

            /// 0b1: Use 16 bit input values
            pub const Bits16: u32 = 0b1;
        }
    }

    /// RRDY
    pub mod RRDY {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Results from computation are not read
            pub const NotReady: u32 = 0b0;

            /// 0b1: Results are ready, this flag will be automatically cleared once value is read
            pub const Ready: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// CORDIC argument register
pub mod WDATA {

    /// ARG
    pub mod ARG {
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

/// CORDIC result register
pub mod RDATA {

    /// RES
    pub mod RES {
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
#[repr(C)]
pub struct RegisterBlock {
    /// CORDIC Control Status register
    pub CSR: RWRegister<u32>,

    /// CORDIC argument register
    pub WDATA: RWRegister<u32>,

    /// CORDIC result register
    pub RDATA: RORegister<u32>,
}
pub struct ResetValues {
    pub CSR: u32,
    pub WDATA: u32,
    pub RDATA: u32,
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
