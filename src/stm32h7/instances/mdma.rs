#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! MDMA
//!
//! Used by: stm32h743, stm32h743v, stm32h747cm4, stm32h747cm7, stm32h753, stm32h753v

#[cfg(not(feature = "nosync"))]
pub use crate::stm32h7::peripherals::mdma::Instance;
pub use crate::stm32h7::peripherals::mdma::{RegisterBlock, ResetValues};
pub use crate::stm32h7::peripherals::mdma::{
    C0BNDTR, C0BRUR, C0CR, C0DAR, C0ESR, C0IFCR, C0ISR, C0LAR, C0MAR, C0MDR, C0SAR, C0TBR, C0TCR,
    C10BNDTR, C10BRUR, C10CR, C10DAR, C10ESR, C10IFCR, C10ISR, C10LAR, C10MAR, C10MDR, C10SAR,
    C10TBR, C10TCR, C11BNDTR, C11BRUR, C11CR, C11DAR, C11ESR, C11IFCR, C11ISR, C11LAR, C11MAR,
    C11MDR, C11SAR, C11TBR, C11TCR, C12BNDTR, C12BRUR, C12CR, C12DAR, C12ESR, C12IFCR, C12ISR,
    C12LAR, C12MAR, C12MDR, C12SAR, C12TBR, C12TCR, C13BNDTR, C13BRUR, C13CR, C13DAR, C13ESR,
    C13IFCR, C13ISR, C13LAR, C13MAR, C13MDR, C13SAR, C13TBR, C13TCR, C14BNDTR, C14BRUR, C14CR,
    C14DAR, C14ESR, C14IFCR, C14ISR, C14LAR, C14MAR, C14MDR, C14SAR, C14TBR, C14TCR, C15BNDTR,
    C15BRUR, C15CR, C15DAR, C15ESR, C15IFCR, C15ISR, C15LAR, C15MAR, C15MDR, C15SAR, C15TBR,
    C15TCR, C1BNDTR, C1BRUR, C1CR, C1DAR, C1ESR, C1IFCR, C1ISR, C1LAR, C1MAR, C1MDR, C1SAR, C1TBR,
    C1TCR, C2BNDTR, C2BRUR, C2CR, C2DAR, C2ESR, C2IFCR, C2ISR, C2LAR, C2MAR, C2MDR, C2SAR, C2TBR,
    C2TCR, C3BNDTR, C3BRUR, C3CR, C3DAR, C3ESR, C3IFCR, C3ISR, C3LAR, C3MAR, C3MDR, C3SAR, C3TBR,
    C3TCR, C4BNDTR, C4BRUR, C4CR, C4DAR, C4ESR, C4IFCR, C4ISR, C4LAR, C4MAR, C4MDR, C4SAR, C4TBR,
    C4TCR, C5BNDTR, C5BRUR, C5CR, C5DAR, C5ESR, C5IFCR, C5ISR, C5LAR, C5MAR, C5MDR, C5SAR, C5TBR,
    C5TCR, C6BNDTR, C6BRUR, C6CR, C6DAR, C6ESR, C6IFCR, C6ISR, C6LAR, C6MAR, C6MDR, C6SAR, C6TBR,
    C6TCR, C7BNDTR, C7BRUR, C7CR, C7DAR, C7ESR, C7IFCR, C7ISR, C7LAR, C7MAR, C7MDR, C7SAR, C7TBR,
    C7TCR, C8BNDTR, C8BRUR, C8CR, C8DAR, C8ESR, C8IFCR, C8ISR, C8LAR, C8MAR, C8MDR, C8SAR, C8TBR,
    C8TCR, C9BNDTR, C9BRUR, C9CR, C9DAR, C9ESR, C9IFCR, C9ISR, C9LAR, C9MAR, C9MDR, C9SAR, C9TBR,
    C9TCR, GISR0,
};

/// Access functions for the MDMA peripheral instance
pub mod MDMA {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x52000000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in MDMA
    pub const reset: ResetValues = ResetValues {
        GISR0: 0x00000000,
        C0ISR: 0x00000000,
        C0IFCR: 0x00000000,
        C0ESR: 0x00000000,
        C0CR: 0x00000000,
        C0TCR: 0x00000000,
        C0BNDTR: 0x00000000,
        C0SAR: 0x00000000,
        C0DAR: 0x00000000,
        C0BRUR: 0x00000000,
        C0LAR: 0x00000000,
        C0TBR: 0x00000000,
        C0MAR: 0x00000000,
        C0MDR: 0x00000000,
        C1ISR: 0x00000000,
        C1IFCR: 0x00000000,
        C1ESR: 0x00000000,
        C1CR: 0x00000000,
        C1TCR: 0x00000000,
        C1BNDTR: 0x00000000,
        C1SAR: 0x00000000,
        C1DAR: 0x00000000,
        C1BRUR: 0x00000000,
        C1LAR: 0x00000000,
        C1TBR: 0x00000000,
        C1MAR: 0x00000000,
        C1MDR: 0x00000000,
        C2ISR: 0x00000000,
        C2IFCR: 0x00000000,
        C2ESR: 0x00000000,
        C2CR: 0x00000000,
        C2TCR: 0x00000000,
        C2BNDTR: 0x00000000,
        C2SAR: 0x00000000,
        C2DAR: 0x00000000,
        C2BRUR: 0x00000000,
        C2LAR: 0x00000000,
        C2TBR: 0x00000000,
        C2MAR: 0x00000000,
        C2MDR: 0x00000000,
        C3ISR: 0x00000000,
        C3IFCR: 0x00000000,
        C3ESR: 0x00000000,
        C3CR: 0x00000000,
        C3TCR: 0x00000000,
        C3BNDTR: 0x00000000,
        C3SAR: 0x00000000,
        C3DAR: 0x00000000,
        C3BRUR: 0x00000000,
        C3LAR: 0x00000000,
        C3TBR: 0x00000000,
        C3MAR: 0x00000000,
        C3MDR: 0x00000000,
        C4ISR: 0x00000000,
        C4IFCR: 0x00000000,
        C4ESR: 0x00000000,
        C4CR: 0x00000000,
        C4TCR: 0x00000000,
        C4BNDTR: 0x00000000,
        C4SAR: 0x00000000,
        C4DAR: 0x00000000,
        C4BRUR: 0x00000000,
        C4LAR: 0x00000000,
        C4TBR: 0x00000000,
        C4MAR: 0x00000000,
        C4MDR: 0x00000000,
        C5ISR: 0x00000000,
        C5IFCR: 0x00000000,
        C5ESR: 0x00000000,
        C5CR: 0x00000000,
        C5TCR: 0x00000000,
        C5BNDTR: 0x00000000,
        C5SAR: 0x00000000,
        C5DAR: 0x00000000,
        C5BRUR: 0x00000000,
        C5LAR: 0x00000000,
        C5TBR: 0x00000000,
        C5MAR: 0x00000000,
        C5MDR: 0x00000000,
        C6ISR: 0x00000000,
        C6IFCR: 0x00000000,
        C6ESR: 0x00000000,
        C6CR: 0x00000000,
        C6TCR: 0x00000000,
        C6BNDTR: 0x00000000,
        C6SAR: 0x00000000,
        C6DAR: 0x00000000,
        C6BRUR: 0x00000000,
        C6LAR: 0x00000000,
        C6TBR: 0x00000000,
        C6MAR: 0x00000000,
        C6MDR: 0x00000000,
        C7ISR: 0x00000000,
        C7IFCR: 0x00000000,
        C7ESR: 0x00000000,
        C7CR: 0x00000000,
        C7TCR: 0x00000000,
        C7BNDTR: 0x00000000,
        C7SAR: 0x00000000,
        C7DAR: 0x00000000,
        C7BRUR: 0x00000000,
        C7LAR: 0x00000000,
        C7TBR: 0x00000000,
        C7MAR: 0x00000000,
        C7MDR: 0x00000000,
        C8ISR: 0x00000000,
        C8IFCR: 0x00000000,
        C8ESR: 0x00000000,
        C8CR: 0x00000000,
        C8TCR: 0x00000000,
        C8BNDTR: 0x00000000,
        C8SAR: 0x00000000,
        C8DAR: 0x00000000,
        C8BRUR: 0x00000000,
        C8LAR: 0x00000000,
        C8TBR: 0x00000000,
        C8MAR: 0x00000000,
        C8MDR: 0x00000000,
        C9ISR: 0x00000000,
        C9IFCR: 0x00000000,
        C9ESR: 0x00000000,
        C9CR: 0x00000000,
        C9TCR: 0x00000000,
        C9BNDTR: 0x00000000,
        C9SAR: 0x00000000,
        C9DAR: 0x00000000,
        C9BRUR: 0x00000000,
        C9LAR: 0x00000000,
        C9TBR: 0x00000000,
        C9MAR: 0x00000000,
        C9MDR: 0x00000000,
        C10ISR: 0x00000000,
        C10IFCR: 0x00000000,
        C10ESR: 0x00000000,
        C10CR: 0x00000000,
        C10TCR: 0x00000000,
        C10BNDTR: 0x00000000,
        C10SAR: 0x00000000,
        C10DAR: 0x00000000,
        C10BRUR: 0x00000000,
        C10LAR: 0x00000000,
        C10TBR: 0x00000000,
        C10MAR: 0x00000000,
        C10MDR: 0x00000000,
        C11ISR: 0x00000000,
        C11IFCR: 0x00000000,
        C11ESR: 0x00000000,
        C11CR: 0x00000000,
        C11TCR: 0x00000000,
        C11BNDTR: 0x00000000,
        C11SAR: 0x00000000,
        C11DAR: 0x00000000,
        C11BRUR: 0x00000000,
        C11LAR: 0x00000000,
        C11TBR: 0x00000000,
        C11MAR: 0x00000000,
        C11MDR: 0x00000000,
        C12ISR: 0x00000000,
        C12IFCR: 0x00000000,
        C12ESR: 0x00000000,
        C12CR: 0x00000000,
        C12TCR: 0x00000000,
        C12BNDTR: 0x00000000,
        C12SAR: 0x00000000,
        C12DAR: 0x00000000,
        C12BRUR: 0x00000000,
        C12LAR: 0x00000000,
        C12TBR: 0x00000000,
        C12MAR: 0x00000000,
        C12MDR: 0x00000000,
        C13ISR: 0x00000000,
        C13IFCR: 0x00000000,
        C13ESR: 0x00000000,
        C13CR: 0x00000000,
        C13TCR: 0x00000000,
        C13BNDTR: 0x00000000,
        C13SAR: 0x00000000,
        C13DAR: 0x00000000,
        C13BRUR: 0x00000000,
        C13LAR: 0x00000000,
        C13TBR: 0x00000000,
        C13MAR: 0x00000000,
        C13MDR: 0x00000000,
        C14ISR: 0x00000000,
        C14IFCR: 0x00000000,
        C14ESR: 0x00000000,
        C14CR: 0x00000000,
        C14TCR: 0x00000000,
        C14BNDTR: 0x00000000,
        C14SAR: 0x00000000,
        C14DAR: 0x00000000,
        C14BRUR: 0x00000000,
        C14LAR: 0x00000000,
        C14TBR: 0x00000000,
        C14MAR: 0x00000000,
        C14MDR: 0x00000000,
        C15ISR: 0x00000000,
        C15IFCR: 0x00000000,
        C15ESR: 0x00000000,
        C15CR: 0x00000000,
        C15TCR: 0x00000000,
        C15BNDTR: 0x00000000,
        C15SAR: 0x00000000,
        C15DAR: 0x00000000,
        C15BRUR: 0x00000000,
        C15LAR: 0x00000000,
        C15TBR: 0x00000000,
        C15MAR: 0x00000000,
        C15MDR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut MDMA_TAKEN: bool = false;

    /// Safe access to MDMA
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
            if MDMA_TAKEN {
                None
            } else {
                MDMA_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to MDMA
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if MDMA_TAKEN && inst.addr == INSTANCE.addr {
                MDMA_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal MDMA
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        MDMA_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to MDMA
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const MDMA: *const RegisterBlock = 0x52000000 as *const _;
