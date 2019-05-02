#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Firewall

use crate::RWRegister;
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// Code segment start address
pub mod CSSA {

    /// code segment start address
    pub mod ADD {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (16 bits: 0xffff << 8)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Code segment length
pub mod CSL {

    /// code segment length
    pub mod LENG {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (14 bits: 0x3fff << 8)
        pub const mask: u32 = 0x3fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Non-volatile data segment start address
pub mod NVDSSA {
    pub use super::CSSA::ADD;
}

/// Non-volatile data segment length
pub mod NVDSL {
    pub use super::CSL::LENG;
}

/// Volatile data segment start address
pub mod VDSSA {

    /// Volatile data segment start address
    pub mod ADD {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (10 bits: 0x3ff << 6)
        pub const mask: u32 = 0x3ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Volatile data segment length
pub mod VDSL {

    /// Non-volatile data segment length
    pub mod LENG {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (10 bits: 0x3ff << 6)
        pub const mask: u32 = 0x3ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Configuration register
pub mod CR {

    /// Volatile data execution
    pub mod VDE {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Volatile data segment cannot be executed if VDS = 0
            pub const NotExecutable: u32 = 0b0;

            /// 0b1: Volatile data segment is declared executable whatever VDS bit value
            pub const Executable: u32 = 0b1;
        }
        /// Write-only values
        pub mod W {

            /// 0b0: Resets volatile data execution bit
            pub const Reset: u32 = 0b0;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Volatile data shared
    pub mod VDS {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Volatile data segment is not shared and cannot be hit by a non protected executable code when the Firewall is closed
            pub const NotShared: u32 = 0b0;

            /// 0b1: Volatile data segment is shared with non protected application code
            pub const Shared: u32 = 0b1;
        }
        /// Write-only values
        pub mod W {

            /// 0b0: Resets volatile data shared bit
            pub const Reset: u32 = 0b0;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Firewall pre alarm
    pub mod FPA {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b0: Any code executed outside the protected segment when the Firewall is opened will generate a system reset
            pub const PreArmReset: u32 = 0b0;

            /// 0b1: Any code executed outside the protected segment will close the Firewall
            pub const PreArmSet: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }
}
pub struct RegisterBlock {
    /// Code segment start address
    pub CSSA: RWRegister<u32>,

    /// Code segment length
    pub CSL: RWRegister<u32>,

    /// Non-volatile data segment start address
    pub NVDSSA: RWRegister<u32>,

    /// Non-volatile data segment length
    pub NVDSL: RWRegister<u32>,

    /// Volatile data segment start address
    pub VDSSA: RWRegister<u32>,

    /// Volatile data segment length
    pub VDSL: RWRegister<u32>,

    _reserved1: [u32; 2],

    /// Configuration register
    pub CR: RWRegister<u32>,
}
pub struct ResetValues {
    pub CSSA: u32,
    pub CSL: u32,
    pub NVDSSA: u32,
    pub NVDSL: u32,
    pub VDSSA: u32,
    pub VDSL: u32,
    pub CR: u32,
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

/// Access functions for the FW peripheral instance
pub mod FW {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40011c00,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in FW
    pub const reset: ResetValues = ResetValues {
        CSSA: 0x00000000,
        CSL: 0x00000000,
        NVDSSA: 0x00000000,
        NVDSL: 0x00000000,
        VDSSA: 0x00000000,
        VDSL: 0x00000000,
        CR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut FW_TAKEN: bool = false;

    /// Safe access to FW
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
            if FW_TAKEN {
                None
            } else {
                FW_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to FW
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if FW_TAKEN && inst.addr == INSTANCE.addr {
                FW_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal FW
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        FW_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to FW
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const FW: *const RegisterBlock = 0x40011c00 as *const _;
