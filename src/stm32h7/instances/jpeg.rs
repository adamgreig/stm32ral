#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! JPEG
//!
//! Used by: stm32h743, stm32h743v, stm32h747cm4, stm32h747cm7, stm32h753, stm32h753v, stm32h7b3

#[cfg(not(feature = "nosync"))]
pub use crate::stm32h7::peripherals::jpeg::Instance;
pub use crate::stm32h7::peripherals::jpeg::{RegisterBlock, ResetValues};
pub use crate::stm32h7::peripherals::jpeg::{
    CFR, CONFR0, CONFR1, CONFR2, CONFR3, CONFRN1, CONFRN2, CONFRN3, CONFRN4, CR, DIR, DOR, SR,
};

/// Access functions for the JPEG peripheral instance
pub mod JPEG {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x52003000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in JPEG
    pub const reset: ResetValues = ResetValues {
        CONFR0: 0x00000000,
        CONFR1: 0x00000000,
        CONFR2: 0x00000000,
        CONFR3: 0x00000000,
        CONFRN1: 0x00000000,
        CONFRN2: 0x00000000,
        CONFRN3: 0x00000000,
        CONFRN4: 0x00000000,
        CR: 0x00000000,
        SR: 0x00000006,
        CFR: 0x00000000,
        DIR: 0x00000000,
        DOR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut JPEG_TAKEN: bool = false;

    /// Safe access to JPEG
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
            if JPEG_TAKEN {
                None
            } else {
                JPEG_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to JPEG
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if JPEG_TAKEN && inst.addr == INSTANCE.addr {
                JPEG_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal JPEG
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        JPEG_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to JPEG
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const JPEG: *const RegisterBlock = 0x52003000 as *const _;
