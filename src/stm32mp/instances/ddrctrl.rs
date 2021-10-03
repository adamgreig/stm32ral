#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! DDRCTRL
//!
//! Used by: stm32mp153, stm32mp157

#[cfg(not(feature = "nosync"))]
pub use crate::stm32mp::peripherals::ddrctrl::Instance;
pub use crate::stm32mp::peripherals::ddrctrl::{RegisterBlock, ResetValues};
pub use crate::stm32mp::peripherals::ddrctrl::{
    DDRCTRL_ADDRMAP1, DDRCTRL_ADDRMAP10, DDRCTRL_ADDRMAP11, DDRCTRL_ADDRMAP2, DDRCTRL_ADDRMAP3,
    DDRCTRL_ADDRMAP4, DDRCTRL_ADDRMAP5, DDRCTRL_ADDRMAP6, DDRCTRL_ADDRMAP9, DDRCTRL_CRCPARCTL0,
    DDRCTRL_CRCPARSTAT, DDRCTRL_DBG0, DDRCTRL_DBG1, DDRCTRL_DBGCAM, DDRCTRL_DBGCMD,
    DDRCTRL_DBGSTAT, DDRCTRL_DERATEEN, DDRCTRL_DERATEINT, DDRCTRL_DFILPCFG0, DDRCTRL_DFIMISC,
    DDRCTRL_DFIPHYMSTR, DDRCTRL_DFISTAT, DDRCTRL_DFITMG0, DDRCTRL_DFITMG1, DDRCTRL_DFIUPD0,
    DDRCTRL_DFIUPD1, DDRCTRL_DFIUPD2, DDRCTRL_DIMMCTL, DDRCTRL_DRAMTMG0, DDRCTRL_DRAMTMG1,
    DDRCTRL_DRAMTMG14, DDRCTRL_DRAMTMG15, DDRCTRL_DRAMTMG2, DDRCTRL_DRAMTMG3, DDRCTRL_DRAMTMG4,
    DDRCTRL_DRAMTMG5, DDRCTRL_DRAMTMG6, DDRCTRL_DRAMTMG7, DDRCTRL_DRAMTMG8, DDRCTRL_HWLPCTL,
    DDRCTRL_INIT0, DDRCTRL_INIT1, DDRCTRL_INIT2, DDRCTRL_INIT3, DDRCTRL_INIT4, DDRCTRL_INIT5,
    DDRCTRL_MRCTRL0, DDRCTRL_MRCTRL1, DDRCTRL_MRSTAT, DDRCTRL_MSTR, DDRCTRL_ODTCFG, DDRCTRL_ODTMAP,
    DDRCTRL_PCCFG, DDRCTRL_PCFGQOS0_0, DDRCTRL_PCFGQOS0_1, DDRCTRL_PCFGQOS1_0, DDRCTRL_PCFGQOS1_1,
    DDRCTRL_PCFGR_0, DDRCTRL_PCFGR_1, DDRCTRL_PCFGWQOS0_0, DDRCTRL_PCFGWQOS0_1,
    DDRCTRL_PCFGWQOS1_0, DDRCTRL_PCFGWQOS1_1, DDRCTRL_PCFGW_0, DDRCTRL_PCFGW_1, DDRCTRL_PCTRL_0,
    DDRCTRL_PCTRL_1, DDRCTRL_PERFHPR1, DDRCTRL_PERFLPR1, DDRCTRL_PERFWR1, DDRCTRL_POISONCFG,
    DDRCTRL_POISONSTAT, DDRCTRL_PSTAT, DDRCTRL_PWRCTL, DDRCTRL_PWRTMG, DDRCTRL_RFSHCTL0,
    DDRCTRL_RFSHCTL3, DDRCTRL_RFSHTMG, DDRCTRL_SCHED, DDRCTRL_SCHED1, DDRCTRL_STAT, DDRCTRL_SWCTL,
    DDRCTRL_SWSTAT, DDRCTRL_ZQCTL0, DDRCTRL_ZQCTL1, DDRCTRL_ZQCTL2, DDRCTRL_ZQSTAT,
};

/// Access functions for the DDRCTRL peripheral instance
pub mod DDRCTRL {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x5a003000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in DDRCTRL
    pub const reset: ResetValues = ResetValues {
        DDRCTRL_MSTR: 0x00040001,
        DDRCTRL_STAT: 0x00000000,
        DDRCTRL_MRCTRL0: 0x00000010,
        DDRCTRL_MRCTRL1: 0x00000000,
        DDRCTRL_MRSTAT: 0x00000000,
        DDRCTRL_DERATEEN: 0x00000000,
        DDRCTRL_DERATEINT: 0x00800000,
        DDRCTRL_PWRCTL: 0x00000000,
        DDRCTRL_PWRTMG: 0x00402010,
        DDRCTRL_HWLPCTL: 0x00000003,
        DDRCTRL_RFSHCTL0: 0x00210000,
        DDRCTRL_RFSHCTL3: 0x00000000,
        DDRCTRL_RFSHTMG: 0x0062008C,
        DDRCTRL_CRCPARCTL0: 0x00000000,
        DDRCTRL_CRCPARSTAT: 0x00000000,
        DDRCTRL_INIT0: 0x0002004E,
        DDRCTRL_INIT1: 0x00000000,
        DDRCTRL_INIT2: 0x00000D05,
        DDRCTRL_INIT3: 0x00000510,
        DDRCTRL_INIT4: 0x00000000,
        DDRCTRL_INIT5: 0x00100004,
        DDRCTRL_DIMMCTL: 0x00000000,
        DDRCTRL_DRAMTMG0: 0x0F101B0F,
        DDRCTRL_DRAMTMG1: 0x00080414,
        DDRCTRL_DRAMTMG2: 0x0305060D,
        DDRCTRL_DRAMTMG3: 0x0050400C,
        DDRCTRL_DRAMTMG4: 0x05040405,
        DDRCTRL_DRAMTMG5: 0x05050403,
        DDRCTRL_DRAMTMG6: 0x02020005,
        DDRCTRL_DRAMTMG7: 0x00000202,
        DDRCTRL_DRAMTMG8: 0x00004405,
        DDRCTRL_DRAMTMG14: 0x000000A0,
        DDRCTRL_DRAMTMG15: 0x00000000,
        DDRCTRL_ZQCTL0: 0x02000040,
        DDRCTRL_ZQCTL1: 0x02000100,
        DDRCTRL_ZQCTL2: 0x00000000,
        DDRCTRL_ZQSTAT: 0x00000000,
        DDRCTRL_DFITMG0: 0x07020002,
        DDRCTRL_DFITMG1: 0x00000404,
        DDRCTRL_DFILPCFG0: 0x07000000,
        DDRCTRL_DFIUPD0: 0x00400003,
        DDRCTRL_DFIUPD1: 0x00010001,
        DDRCTRL_DFIUPD2: 0x80000000,
        DDRCTRL_DFIMISC: 0x00000001,
        DDRCTRL_DFISTAT: 0x00000000,
        DDRCTRL_DFIPHYMSTR: 0x00000001,
        DDRCTRL_ADDRMAP1: 0x00000000,
        DDRCTRL_ADDRMAP2: 0x00000000,
        DDRCTRL_ADDRMAP3: 0x00000000,
        DDRCTRL_ADDRMAP4: 0x00000000,
        DDRCTRL_ADDRMAP5: 0x00000000,
        DDRCTRL_ADDRMAP6: 0x00000000,
        DDRCTRL_ADDRMAP9: 0x00000000,
        DDRCTRL_ADDRMAP10: 0x00000000,
        DDRCTRL_ADDRMAP11: 0x00000000,
        DDRCTRL_ODTCFG: 0x04000400,
        DDRCTRL_ODTMAP: 0x00000011,
        DDRCTRL_SCHED: 0x00000805,
        DDRCTRL_SCHED1: 0x00000000,
        DDRCTRL_PERFHPR1: 0x0F000001,
        DDRCTRL_PERFLPR1: 0x0F00007F,
        DDRCTRL_PERFWR1: 0x0F00007F,
        DDRCTRL_DBG0: 0x00000000,
        DDRCTRL_DBG1: 0x00000000,
        DDRCTRL_DBGCAM: 0x00000000,
        DDRCTRL_DBGCMD: 0x00000000,
        DDRCTRL_DBGSTAT: 0x00000000,
        DDRCTRL_SWCTL: 0x00000001,
        DDRCTRL_SWSTAT: 0x00000001,
        DDRCTRL_POISONCFG: 0x00110011,
        DDRCTRL_POISONSTAT: 0x00000000,
        DDRCTRL_PSTAT: 0x00000000,
        DDRCTRL_PCCFG: 0x00000000,
        DDRCTRL_PCFGR_0: 0x00004000,
        DDRCTRL_PCFGW_0: 0x00004000,
        DDRCTRL_PCTRL_0: 0x00000000,
        DDRCTRL_PCFGQOS0_0: 0x02000E00,
        DDRCTRL_PCFGQOS1_0: 0x00000000,
        DDRCTRL_PCFGWQOS0_0: 0x00000E00,
        DDRCTRL_PCFGWQOS1_0: 0x00000000,
        DDRCTRL_PCFGR_1: 0x00004000,
        DDRCTRL_PCFGW_1: 0x00004000,
        DDRCTRL_PCTRL_1: 0x00000000,
        DDRCTRL_PCFGQOS0_1: 0x02000E00,
        DDRCTRL_PCFGQOS1_1: 0x00000000,
        DDRCTRL_PCFGWQOS0_1: 0x00000E00,
        DDRCTRL_PCFGWQOS1_1: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut DDRCTRL_TAKEN: bool = false;

    /// Safe access to DDRCTRL
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
            if DDRCTRL_TAKEN {
                None
            } else {
                DDRCTRL_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to DDRCTRL
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if DDRCTRL_TAKEN && inst.addr == INSTANCE.addr {
                DDRCTRL_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal DDRCTRL
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        DDRCTRL_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to DDRCTRL
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const DDRCTRL: *const RegisterBlock = 0x5a003000 as *const _;
