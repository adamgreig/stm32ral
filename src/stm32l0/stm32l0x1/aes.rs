#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Advanced encryption standard hardware accelerator

#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;
use {RORegister, RWRegister};

/// control register
pub mod CR {

    /// Enable DMA management of data output phase
    pub mod DMAOUTEN {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Disable DMA Output
            pub const Disabled: u32 = 0b0;

            /// 0b1: Enabled DMA Output
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Enable DMA management of data input phase
    pub mod DMAINEN {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Disable DMA Input
            pub const Disabled: u32 = 0b0;

            /// 0b1: Enable DMA Input
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Error interrupt enable
    pub mod ERRIE {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Disable (mask) error interrupt
            pub const Disabled: u32 = 0b0;

            /// 0b1: Enable error interrupt
            pub const Enabled: u32 = 0b1;
        }
    }

    /// CCF flag interrupt enable
    pub mod CCFIE {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Disable (mask) CCF interrupt
            pub const Disabled: u32 = 0b0;

            /// 0b1: Enable CCF interrupt
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Error clear
    pub mod ERRC {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Clear RDERR and WRERR flags
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Computation Complete Flag Clear
    pub mod CCFC {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Clear computation complete flag
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// AES chaining mode
    pub mod CHMOD {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (2 bits: 0b11 << 5)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Electronic codebook (ECB)
            pub const ECB: u32 = 0b00;

            /// 0b01: Cipher-Block Chaining (CBC)
            pub const CBC: u32 = 0b01;

            /// 0b10: Counter Mode (CTR)
            pub const CTR: u32 = 0b10;
        }
    }

    /// AES operating mode
    pub mod MODE {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (2 bits: 0b11 << 3)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Mode 1: encryption
            pub const Mode1: u32 = 0b00;

            /// 0b01: Mode 2: key derivation (or key preparation for ECB/CBC decryption)
            pub const Mode2: u32 = 0b01;

            /// 0b10: Mode 3: decryption
            pub const Mode3: u32 = 0b10;

            /// 0b11: Mode 4: key derivation then single decryption
            pub const Mode4: u32 = 0b11;
        }
    }

    /// Data type selection (for data in and data out to/from the cryptographic block)
    pub mod DATATYPE {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (2 bits: 0b11 << 1)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Word
            pub const None: u32 = 0b00;

            /// 0b01: Half-word (16-bit)
            pub const HalfWord: u32 = 0b01;

            /// 0b10: Byte (8-bit)
            pub const Byte: u32 = 0b10;

            /// 0b11: Bit
            pub const Bit: u32 = 0b11;
        }
    }

    /// AES enable
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

            /// 0b0: Disable AES
            pub const Disabled: u32 = 0b0;

            /// 0b1: Enable AES
            pub const Enabled: u32 = 0b1;
        }
    }
}

/// status register
pub mod SR {

    /// Write error flag
    pub mod WRERR {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Write error not detected
            pub const NoError: u32 = 0b0;

            /// 0b1: Write error detected
            pub const Error: u32 = 0b1;
        }
    }

    /// Read error flag
    pub mod RDERR {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Read error not detected
            pub const NoError: u32 = 0b0;

            /// 0b1: Read error detected
            pub const Error: u32 = 0b1;
        }
    }

    /// Computation complete flag
    pub mod CCF {
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

            /// 0b0: Computation complete
            pub const Complete: u32 = 0b0;

            /// 0b1: Computation not complete
            pub const NotComplete: u32 = 0b1;
        }
    }
}

/// data input register
pub mod DINR {

    /// Data Input Register.
    pub mod DIN {
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

/// data output register
pub mod DOUTR {

    /// Data output register
    pub mod DOUT {
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

/// key register 0
pub mod KEYR0 {

    /// Data Output Register (LSB key \[31:0\])
    pub mod KEY0 {
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

/// key register 1
pub mod KEYR1 {

    /// AES key register (key \[63:32\])
    pub mod KEY1 {
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

/// key register 2
pub mod KEYR2 {

    /// AES key register (key \[95:64\])
    pub mod KEY2 {
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

/// key register 3
pub mod KEYR3 {

    /// AES key register (MSB key \[127:96\])
    pub mod KEY3 {
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

/// initialization vector register 0
pub mod IVR0 {

    /// initialization vector register (LSB IVR \[31:0\])
    pub mod IV0 {
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

/// initialization vector register 1
pub mod IVR1 {

    /// Initialization Vector Register (IVR \[63:32\])
    pub mod IV1 {
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

/// initialization vector register 2
pub mod IVR2 {

    /// Initialization Vector Register (IVR \[95:64\])
    pub mod IV2 {
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

/// initialization vector register 3
pub mod IVR3 {

    /// Initialization Vector Register (MSB IVR \[127:96\])
    pub mod IV3 {
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
pub struct RegisterBlock {
    /// control register
    pub CR: RWRegister<u32>,

    /// status register
    pub SR: RORegister<u32>,

    /// data input register
    pub DINR: RWRegister<u32>,

    /// data output register
    pub DOUTR: RORegister<u32>,

    /// key register 0
    pub KEYR0: RWRegister<u32>,

    /// key register 1
    pub KEYR1: RWRegister<u32>,

    /// key register 2
    pub KEYR2: RWRegister<u32>,

    /// key register 3
    pub KEYR3: RWRegister<u32>,

    /// initialization vector register 0
    pub IVR0: RWRegister<u32>,

    /// initialization vector register 1
    pub IVR1: RWRegister<u32>,

    /// initialization vector register 2
    pub IVR2: RWRegister<u32>,

    /// initialization vector register 3
    pub IVR3: RWRegister<u32>,
}
pub struct ResetValues {
    pub CR: u32,
    pub SR: u32,
    pub DINR: u32,
    pub DOUTR: u32,
    pub KEYR0: u32,
    pub KEYR1: u32,
    pub KEYR2: u32,
    pub KEYR3: u32,
    pub IVR0: u32,
    pub IVR1: u32,
    pub IVR2: u32,
    pub IVR3: u32,
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

/// Access functions for the AES peripheral instance
pub mod AES {
    #[cfg(not(feature = "nosync"))]
    use external_cortex_m;

    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40026000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in AES
    pub const reset: ResetValues = ResetValues {
        CR: 0x00000000,
        SR: 0x00000000,
        DINR: 0x00000000,
        DOUTR: 0x00000000,
        KEYR0: 0x00000000,
        KEYR1: 0x00000000,
        KEYR2: 0x00000000,
        KEYR3: 0x00000000,
        IVR0: 0x00000000,
        IVR1: 0x00000000,
        IVR2: 0x00000000,
        IVR3: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut AES_TAKEN: bool = false;

    /// Safe access to AES
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
            if AES_TAKEN {
                None
            } else {
                AES_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to AES
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if AES_TAKEN && inst.addr == INSTANCE.addr {
                AES_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal AES
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        AES_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to AES
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const AES: *const RegisterBlock = 0x40026000 as *const _;
