#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Public key accelerator
//!
//! Used by: stm32wl5x_cm0p, stm32wl5x_cm4, stm32wle5

use crate::{RORegister, RWRegister, WORegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// control register
pub mod CR {

    /// Address error interrupt enable
    pub mod ADDRERRIE {
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

            /// 0b0: No interrupt is generated when ADDRERRF flag is set in PKA_SR
            pub const Disabled: u32 = 0b0;

            /// 0b1: An interrupt is generated when ADDRERRF flag is set in PKA_SR
            pub const Enabled: u32 = 0b1;
        }
    }

    /// RAM error interrupt enable
    pub mod RAMERRIE {
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

            /// 0b0: No interrupt is generated when RAMERRF flag is set in PKA_SR
            pub const Disabled: u32 = 0b0;

            /// 0b1: An interrupt is generated when RAMERRF flag is set in PKA_SR
            pub const Enabled: u32 = 0b1;
        }
    }

    /// PROCENDIE
    pub mod PROCENDIE {
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

            /// 0b0: No interrupt is generated when PROCENDF flag is set in PKA_SR
            pub const Disabled: u32 = 0b0;

            /// 0b1: An interrupt is generated when PROCENDF flag is set in PKA_SR
            pub const Enabled: u32 = 0b1;
        }
    }

    /// PKA operation code
    pub mod MODE {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (6 bits: 0x3f << 8)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000000: Montgomery parameter computation then modular exponentiation
            pub const MontgomeryCompExp: u32 = 0b000000;

            /// 0b000001: Montgomery parameter computation only
            pub const MontgomeryComp: u32 = 0b000001;

            /// 0b000010: Modular exponentiation only (Montgomery parameter must be loaded first)
            pub const MontgomeryExp: u32 = 0b000010;

            /// 0b000111: RSA CRT exponentiation
            pub const RSA: u32 = 0b000111;

            /// 0b001000: Modular inversion
            pub const ModularInv: u32 = 0b001000;

            /// 0b001001: Arithmetic addition
            pub const ArithmeticAdd: u32 = 0b001001;

            /// 0b001010: Arithmetic subtraction
            pub const ArithmeticSub: u32 = 0b001010;

            /// 0b001011: Arithmetic multiplication
            pub const ArithmeticMul: u32 = 0b001011;

            /// 0b001100: Arithmetic comparison
            pub const ArithmeticComp: u32 = 0b001100;

            /// 0b001101: Modular reduction
            pub const ModularRed: u32 = 0b001101;

            /// 0b001110: Modular addition
            pub const ModularAdd: u32 = 0b001110;

            /// 0b001111: Modular subtraction
            pub const ModularSub: u32 = 0b001111;

            /// 0b010000: Montgomery multiplication
            pub const ModularMul: u32 = 0b010000;

            /// 0b100000: Montgomery parameter computation then ECC scalar multiplication
            pub const MontgomeryCompScalar: u32 = 0b100000;

            /// 0b100010: ECC scalar multiplication only (Montgomery parameter must be loaded first)
            pub const MontgomeryScalar: u32 = 0b100010;

            /// 0b100100: ECDSA sign
            pub const ECDSASign: u32 = 0b100100;

            /// 0b100110: ECDSA verification
            pub const ECDSAVerif: u32 = 0b100110;

            /// 0b101000: Point on elliptic curve Fp check
            pub const Elliptic: u32 = 0b101000;
        }
    }

    /// start the operation
    pub mod START {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Writing 1 to this bit starts the operation which is selected by MODE\[5:0\], using the operands and data already written to the PKA RAM - This bit is always read as 0
            pub const Start: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PKA enable.
    pub mod EN {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Disable PKA
            pub const Disabled: u32 = 0b0;

            /// 0b1: Enable PKA
            pub const Enabled: u32 = 0b1;
        }
    }
}

/// status register
pub mod SR {

    /// Address error flag
    pub mod ADDRERRF {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No error
            pub const NoError: u32 = 0b0;

            /// 0b1: Address access is out of range (unmapped address)
            pub const Error: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PKA RAM error flag
    pub mod RAMERRF {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No error
            pub const NoError: u32 = 0b0;

            /// 0b1: An AHB access to the PKA RAM occurred while the PKA core was computing and using its internal RAM (AHB PKA_RAM access are not allowed while PKA operation is in progress)
            pub const Error: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PKA End of Operation flag
    pub mod PROCENDF {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Operation in progress
            pub const InProgress: u32 = 0b0;

            /// 0b1: PKA operation is completed - set when BUSY is deasserted
            pub const Completed: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PKA operation is in progressThis bit is set to 1 whenever START bit in the PKA_CR is set. It is automatically cleared when the computation is complete, meaning that PKA RAM can be safely accessed and a new operation can be started.
    pub mod BUSY {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No operation in pgoress
            pub const Idle: u32 = 0b0;

            /// 0b1: Operation in progress
            pub const Busy: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// clear flag register
pub mod CLRFR {

    /// Clear Address error flag
    pub mod ADDRERRFC {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Clear ADDRERRF flag
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Clear PKA RAM error flag
    pub mod RAMERRFC {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Clear RAMERRF flag
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Clear PKA End of Operation flag
    pub mod PROCENDFC {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Clear PROCENDF flag
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }
}
#[repr(C)]
pub struct RegisterBlock {
    /// control register
    pub CR: RWRegister<u32>,

    /// status register
    pub SR: RORegister<u32>,

    /// clear flag register
    pub CLRFR: WORegister<u32>,
}
pub struct ResetValues {
    pub CR: u32,
    pub SR: u32,
    pub CLRFR: u32,
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
