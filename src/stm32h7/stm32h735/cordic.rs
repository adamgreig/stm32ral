#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! CORDIC co-processor

use crate::RWRegister;
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// Control and status register
pub mod CSR {

    /// Result ready flag
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

    /// Width of input data
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

    /// Width of output data
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

    /// Number of arguments expected by the WDATA register
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

    /// Number of results in the RDATA register
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

    /// Enable DMA write channel
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

    /// Enable DMA wread channel
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

    /// Enable interrupt
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

            /// 0b1: Enable interrupt request generation
            pub const Enabled: u32 = 0b1;
        }
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

    /// Precision required (number of iterations/cycles), where PRECISION = (number of iterations/4)
    pub mod PRECISION {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0001: 4 iterations
            pub const Iters4: u32 = 0b0001;

            /// 0b0010: 8 iterations
            pub const Iters8: u32 = 0b0010;

            /// 0b0011: 12 iterations
            pub const Iters12: u32 = 0b0011;

            /// 0b0100: 16 iterations
            pub const Iters16: u32 = 0b0100;

            /// 0b0101: 20 iterations
            pub const Iters20: u32 = 0b0101;

            /// 0b0110: 24 iterations
            pub const Iters24: u32 = 0b0110;

            /// 0b0111: 28 iterations
            pub const Iters28: u32 = 0b0111;

            /// 0b1000: 32 iterations
            pub const Iters32: u32 = 0b1000;

            /// 0b1001: 36 iterations
            pub const Iters36: u32 = 0b1001;

            /// 0b1010: 40 iterations
            pub const Iters40: u32 = 0b1010;

            /// 0b1011: 44 iterations
            pub const Iters44: u32 = 0b1011;

            /// 0b1100: 48 iterations
            pub const Iters48: u32 = 0b1100;

            /// 0b1101: 52 iterations
            pub const Iters52: u32 = 0b1101;

            /// 0b1110: 56 iterations
            pub const Iters56: u32 = 0b1110;

            /// 0b1111: 60 iterations
            pub const Iters60: u32 = 0b1111;
        }
    }

    /// Function
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

            /// 0b0000: Cosine function
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
}

/// Argument register
pub mod WDATA {

    /// Function input arguments
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

/// Result register
pub mod RDATA {

    /// Function result
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
    /// Control and status register
    pub CSR: RWRegister<u32>,

    /// Argument register
    pub WDATA: RWRegister<u32>,

    /// Result register
    pub RDATA: RWRegister<u32>,
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

/// Access functions for the CORDIC peripheral instance
pub mod CORDIC {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x48024400,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in CORDIC
    pub const reset: ResetValues = ResetValues {
        CSR: 0x00000050,
        WDATA: 0xFFFFFFFF,
        RDATA: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut CORDIC_TAKEN: bool = false;

    /// Safe access to CORDIC
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
            if CORDIC_TAKEN {
                None
            } else {
                CORDIC_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to CORDIC
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if CORDIC_TAKEN && inst.addr == INSTANCE.addr {
                CORDIC_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal CORDIC
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        CORDIC_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to CORDIC
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const CORDIC: *const RegisterBlock = 0x48024400 as *const _;
