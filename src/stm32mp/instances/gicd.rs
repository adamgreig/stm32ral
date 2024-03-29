#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! GICD
//!
//! Used by: stm32mp153, stm32mp157

#[cfg(not(feature = "nosync"))]
pub use crate::stm32mp::peripherals::gicd::Instance;
pub use crate::stm32mp::peripherals::gicd::{RegisterBlock, ResetValues};
pub use crate::stm32mp::peripherals::gicd::{
    GICD_CIDR0, GICD_CIDR1, GICD_CIDR2, GICD_CIDR3, GICD_CPENDSGIR0, GICD_CPENDSGIR1,
    GICD_CPENDSGIR2, GICD_CPENDSGIR3, GICD_CTLR, GICD_ICACTIVER0, GICD_ICACTIVER1, GICD_ICACTIVER2,
    GICD_ICACTIVER3, GICD_ICACTIVER4, GICD_ICACTIVER5, GICD_ICACTIVER6, GICD_ICACTIVER7,
    GICD_ICACTIVER8, GICD_ICENABLER0, GICD_ICENABLER1, GICD_ICENABLER2, GICD_ICENABLER3,
    GICD_ICENABLER4, GICD_ICENABLER5, GICD_ICENABLER6, GICD_ICENABLER7, GICD_ICENABLER8,
    GICD_ICFGR0, GICD_ICFGR1, GICD_ICFGR10, GICD_ICFGR11, GICD_ICFGR12, GICD_ICFGR13, GICD_ICFGR14,
    GICD_ICFGR15, GICD_ICFGR16, GICD_ICFGR17, GICD_ICFGR2, GICD_ICFGR3, GICD_ICFGR4, GICD_ICFGR5,
    GICD_ICFGR6, GICD_ICFGR7, GICD_ICFGR8, GICD_ICFGR9, GICD_ICPENDR0, GICD_ICPENDR1,
    GICD_ICPENDR2, GICD_ICPENDR3, GICD_ICPENDR4, GICD_ICPENDR5, GICD_ICPENDR6, GICD_ICPENDR7,
    GICD_ICPENDR8, GICD_IGROUPR0, GICD_IGROUPR1, GICD_IGROUPR2, GICD_IGROUPR3, GICD_IGROUPR4,
    GICD_IGROUPR5, GICD_IGROUPR6, GICD_IGROUPR7, GICD_IGROUPR8, GICD_IIDR, GICD_IPRIORITYR0,
    GICD_IPRIORITYR1, GICD_IPRIORITYR10, GICD_IPRIORITYR11, GICD_IPRIORITYR12, GICD_IPRIORITYR13,
    GICD_IPRIORITYR14, GICD_IPRIORITYR15, GICD_IPRIORITYR16, GICD_IPRIORITYR17, GICD_IPRIORITYR18,
    GICD_IPRIORITYR19, GICD_IPRIORITYR2, GICD_IPRIORITYR20, GICD_IPRIORITYR21, GICD_IPRIORITYR22,
    GICD_IPRIORITYR23, GICD_IPRIORITYR24, GICD_IPRIORITYR25, GICD_IPRIORITYR26, GICD_IPRIORITYR27,
    GICD_IPRIORITYR28, GICD_IPRIORITYR29, GICD_IPRIORITYR3, GICD_IPRIORITYR30, GICD_IPRIORITYR31,
    GICD_IPRIORITYR32, GICD_IPRIORITYR33, GICD_IPRIORITYR34, GICD_IPRIORITYR35, GICD_IPRIORITYR36,
    GICD_IPRIORITYR37, GICD_IPRIORITYR38, GICD_IPRIORITYR39, GICD_IPRIORITYR4, GICD_IPRIORITYR40,
    GICD_IPRIORITYR41, GICD_IPRIORITYR42, GICD_IPRIORITYR43, GICD_IPRIORITYR44, GICD_IPRIORITYR45,
    GICD_IPRIORITYR46, GICD_IPRIORITYR47, GICD_IPRIORITYR48, GICD_IPRIORITYR49, GICD_IPRIORITYR5,
    GICD_IPRIORITYR50, GICD_IPRIORITYR51, GICD_IPRIORITYR52, GICD_IPRIORITYR53, GICD_IPRIORITYR54,
    GICD_IPRIORITYR55, GICD_IPRIORITYR56, GICD_IPRIORITYR57, GICD_IPRIORITYR58, GICD_IPRIORITYR59,
    GICD_IPRIORITYR6, GICD_IPRIORITYR60, GICD_IPRIORITYR61, GICD_IPRIORITYR62, GICD_IPRIORITYR63,
    GICD_IPRIORITYR64, GICD_IPRIORITYR65, GICD_IPRIORITYR66, GICD_IPRIORITYR67, GICD_IPRIORITYR68,
    GICD_IPRIORITYR69, GICD_IPRIORITYR7, GICD_IPRIORITYR70, GICD_IPRIORITYR71, GICD_IPRIORITYR8,
    GICD_IPRIORITYR9, GICD_ISACTIVER0, GICD_ISACTIVER1, GICD_ISACTIVER2, GICD_ISACTIVER3,
    GICD_ISACTIVER4, GICD_ISACTIVER5, GICD_ISACTIVER6, GICD_ISACTIVER7, GICD_ISACTIVER8,
    GICD_ISENABLER0, GICD_ISENABLER1, GICD_ISENABLER2, GICD_ISENABLER3, GICD_ISENABLER4,
    GICD_ISENABLER5, GICD_ISENABLER6, GICD_ISENABLER7, GICD_ISENABLER8, GICD_ISPENDR0,
    GICD_ISPENDR1, GICD_ISPENDR2, GICD_ISPENDR3, GICD_ISPENDR4, GICD_ISPENDR5, GICD_ISPENDR6,
    GICD_ISPENDR7, GICD_ISPENDR8, GICD_ITARGETSR0, GICD_ITARGETSR1, GICD_ITARGETSR10,
    GICD_ITARGETSR11, GICD_ITARGETSR12, GICD_ITARGETSR13, GICD_ITARGETSR14, GICD_ITARGETSR15,
    GICD_ITARGETSR16, GICD_ITARGETSR17, GICD_ITARGETSR18, GICD_ITARGETSR19, GICD_ITARGETSR2,
    GICD_ITARGETSR20, GICD_ITARGETSR21, GICD_ITARGETSR22, GICD_ITARGETSR23, GICD_ITARGETSR24,
    GICD_ITARGETSR25, GICD_ITARGETSR26, GICD_ITARGETSR27, GICD_ITARGETSR28, GICD_ITARGETSR29,
    GICD_ITARGETSR3, GICD_ITARGETSR30, GICD_ITARGETSR31, GICD_ITARGETSR32, GICD_ITARGETSR33,
    GICD_ITARGETSR34, GICD_ITARGETSR35, GICD_ITARGETSR36, GICD_ITARGETSR37, GICD_ITARGETSR38,
    GICD_ITARGETSR39, GICD_ITARGETSR4, GICD_ITARGETSR40, GICD_ITARGETSR41, GICD_ITARGETSR42,
    GICD_ITARGETSR43, GICD_ITARGETSR44, GICD_ITARGETSR45, GICD_ITARGETSR46, GICD_ITARGETSR47,
    GICD_ITARGETSR48, GICD_ITARGETSR49, GICD_ITARGETSR5, GICD_ITARGETSR50, GICD_ITARGETSR51,
    GICD_ITARGETSR52, GICD_ITARGETSR53, GICD_ITARGETSR54, GICD_ITARGETSR55, GICD_ITARGETSR56,
    GICD_ITARGETSR57, GICD_ITARGETSR58, GICD_ITARGETSR59, GICD_ITARGETSR6, GICD_ITARGETSR60,
    GICD_ITARGETSR61, GICD_ITARGETSR62, GICD_ITARGETSR63, GICD_ITARGETSR64, GICD_ITARGETSR65,
    GICD_ITARGETSR66, GICD_ITARGETSR67, GICD_ITARGETSR68, GICD_ITARGETSR69, GICD_ITARGETSR7,
    GICD_ITARGETSR70, GICD_ITARGETSR71, GICD_ITARGETSR8, GICD_ITARGETSR9, GICD_PIDR0, GICD_PIDR1,
    GICD_PIDR2, GICD_PIDR3, GICD_PIDR4, GICD_PIDR5, GICD_PIDR6, GICD_PIDR7, GICD_PPISR, GICD_SGIR,
    GICD_SPENDSGIR0, GICD_SPENDSGIR1, GICD_SPENDSGIR2, GICD_SPENDSGIR3, GICD_SPISR1, GICD_SPISR2,
    GICD_SPISR3, GICD_SPISR4, GICD_SPISR5, GICD_SPISR6, GICD_SPISR7, GICD_TYPER,
};

/// Access functions for the GICD peripheral instance
pub mod GICD {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0xa0021000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in GICD
    pub const reset: ResetValues = ResetValues {
        GICD_CTLR: 0x00000000,
        GICD_TYPER: 0x0000FC28,
        GICD_IIDR: 0x0100143B,
        GICD_IGROUPR0: 0x00000000,
        GICD_IGROUPR1: 0x00000000,
        GICD_IGROUPR2: 0x00000000,
        GICD_IGROUPR3: 0x00000000,
        GICD_IGROUPR4: 0x00000000,
        GICD_IGROUPR5: 0x00000000,
        GICD_IGROUPR6: 0x00000000,
        GICD_IGROUPR7: 0x00000000,
        GICD_IGROUPR8: 0x00000000,
        GICD_ISENABLER0: 0x0000FFFF,
        GICD_ISENABLER1: 0x00000000,
        GICD_ISENABLER2: 0x00000000,
        GICD_ISENABLER3: 0x00000000,
        GICD_ISENABLER4: 0x00000000,
        GICD_ISENABLER5: 0x00000000,
        GICD_ISENABLER6: 0x00000000,
        GICD_ISENABLER7: 0x00000000,
        GICD_ISENABLER8: 0x00000000,
        GICD_ICENABLER0: 0x0000FFFF,
        GICD_ICENABLER1: 0x00000000,
        GICD_ICENABLER2: 0x00000000,
        GICD_ICENABLER3: 0x00000000,
        GICD_ICENABLER4: 0x00000000,
        GICD_ICENABLER5: 0x00000000,
        GICD_ICENABLER6: 0x00000000,
        GICD_ICENABLER7: 0x00000000,
        GICD_ICENABLER8: 0x00000000,
        GICD_ISPENDR0: 0x00000000,
        GICD_ISPENDR1: 0x00000000,
        GICD_ISPENDR2: 0x00000000,
        GICD_ISPENDR3: 0x00000000,
        GICD_ISPENDR4: 0x00000000,
        GICD_ISPENDR5: 0x00000000,
        GICD_ISPENDR6: 0x00000000,
        GICD_ISPENDR7: 0x00000000,
        GICD_ISPENDR8: 0x00000000,
        GICD_ICPENDR0: 0x00000000,
        GICD_ICPENDR1: 0x00000000,
        GICD_ICPENDR2: 0x00000000,
        GICD_ICPENDR3: 0x00000000,
        GICD_ICPENDR4: 0x00000000,
        GICD_ICPENDR5: 0x00000000,
        GICD_ICPENDR6: 0x00000000,
        GICD_ICPENDR7: 0x00000000,
        GICD_ICPENDR8: 0x00000000,
        GICD_ISACTIVER0: 0x00000000,
        GICD_ISACTIVER1: 0x00000000,
        GICD_ISACTIVER2: 0x00000000,
        GICD_ISACTIVER3: 0x00000000,
        GICD_ISACTIVER4: 0x00000000,
        GICD_ISACTIVER5: 0x00000000,
        GICD_ISACTIVER6: 0x00000000,
        GICD_ISACTIVER7: 0x00000000,
        GICD_ISACTIVER8: 0x00000000,
        GICD_ICACTIVER0: 0x00000000,
        GICD_ICACTIVER1: 0x00000000,
        GICD_ICACTIVER2: 0x00000000,
        GICD_ICACTIVER3: 0x00000000,
        GICD_ICACTIVER4: 0x00000000,
        GICD_ICACTIVER5: 0x00000000,
        GICD_ICACTIVER6: 0x00000000,
        GICD_ICACTIVER7: 0x00000000,
        GICD_ICACTIVER8: 0x00000000,
        GICD_IPRIORITYR0: 0x00000000,
        GICD_IPRIORITYR1: 0x00000000,
        GICD_IPRIORITYR2: 0x00000000,
        GICD_IPRIORITYR3: 0x00000000,
        GICD_IPRIORITYR4: 0x00000000,
        GICD_IPRIORITYR5: 0x00000000,
        GICD_IPRIORITYR6: 0x00000000,
        GICD_IPRIORITYR7: 0x00000000,
        GICD_IPRIORITYR8: 0x00000000,
        GICD_IPRIORITYR9: 0x00000000,
        GICD_IPRIORITYR10: 0x00000000,
        GICD_IPRIORITYR11: 0x00000000,
        GICD_IPRIORITYR12: 0x00000000,
        GICD_IPRIORITYR13: 0x00000000,
        GICD_IPRIORITYR14: 0x00000000,
        GICD_IPRIORITYR15: 0x00000000,
        GICD_IPRIORITYR16: 0x00000000,
        GICD_IPRIORITYR17: 0x00000000,
        GICD_IPRIORITYR18: 0x00000000,
        GICD_IPRIORITYR19: 0x00000000,
        GICD_IPRIORITYR20: 0x00000000,
        GICD_IPRIORITYR21: 0x00000000,
        GICD_IPRIORITYR22: 0x00000000,
        GICD_IPRIORITYR23: 0x00000000,
        GICD_IPRIORITYR24: 0x00000000,
        GICD_IPRIORITYR25: 0x00000000,
        GICD_IPRIORITYR26: 0x00000000,
        GICD_IPRIORITYR27: 0x00000000,
        GICD_IPRIORITYR28: 0x00000000,
        GICD_IPRIORITYR29: 0x00000000,
        GICD_IPRIORITYR30: 0x00000000,
        GICD_IPRIORITYR31: 0x00000000,
        GICD_IPRIORITYR32: 0x00000000,
        GICD_IPRIORITYR33: 0x00000000,
        GICD_IPRIORITYR34: 0x00000000,
        GICD_IPRIORITYR35: 0x00000000,
        GICD_IPRIORITYR36: 0x00000000,
        GICD_IPRIORITYR37: 0x00000000,
        GICD_IPRIORITYR38: 0x00000000,
        GICD_IPRIORITYR39: 0x00000000,
        GICD_IPRIORITYR40: 0x00000000,
        GICD_IPRIORITYR41: 0x00000000,
        GICD_IPRIORITYR42: 0x00000000,
        GICD_IPRIORITYR43: 0x00000000,
        GICD_IPRIORITYR44: 0x00000000,
        GICD_IPRIORITYR45: 0x00000000,
        GICD_IPRIORITYR46: 0x00000000,
        GICD_IPRIORITYR47: 0x00000000,
        GICD_IPRIORITYR48: 0x00000000,
        GICD_IPRIORITYR49: 0x00000000,
        GICD_IPRIORITYR50: 0x00000000,
        GICD_IPRIORITYR51: 0x00000000,
        GICD_IPRIORITYR52: 0x00000000,
        GICD_IPRIORITYR53: 0x00000000,
        GICD_IPRIORITYR54: 0x00000000,
        GICD_IPRIORITYR55: 0x00000000,
        GICD_IPRIORITYR56: 0x00000000,
        GICD_IPRIORITYR57: 0x00000000,
        GICD_IPRIORITYR58: 0x00000000,
        GICD_IPRIORITYR59: 0x00000000,
        GICD_IPRIORITYR60: 0x00000000,
        GICD_IPRIORITYR61: 0x00000000,
        GICD_IPRIORITYR62: 0x00000000,
        GICD_IPRIORITYR63: 0x00000000,
        GICD_IPRIORITYR64: 0x00000000,
        GICD_IPRIORITYR65: 0x00000000,
        GICD_IPRIORITYR66: 0x00000000,
        GICD_IPRIORITYR67: 0x00000000,
        GICD_IPRIORITYR68: 0x00000000,
        GICD_IPRIORITYR69: 0x00000000,
        GICD_IPRIORITYR70: 0x00000000,
        GICD_IPRIORITYR71: 0x00000000,
        GICD_ITARGETSR0: 0x00000000,
        GICD_ITARGETSR1: 0x00000000,
        GICD_ITARGETSR2: 0x00000000,
        GICD_ITARGETSR3: 0x00000000,
        GICD_ITARGETSR4: 0x00000000,
        GICD_ITARGETSR5: 0x00000000,
        GICD_ITARGETSR6: 0x00000000,
        GICD_ITARGETSR7: 0x00000000,
        GICD_ITARGETSR8: 0x00000000,
        GICD_ITARGETSR9: 0x00000000,
        GICD_ITARGETSR10: 0x00000000,
        GICD_ITARGETSR11: 0x00000000,
        GICD_ITARGETSR12: 0x00000000,
        GICD_ITARGETSR13: 0x00000000,
        GICD_ITARGETSR14: 0x00000000,
        GICD_ITARGETSR15: 0x00000000,
        GICD_ITARGETSR16: 0x00000000,
        GICD_ITARGETSR17: 0x00000000,
        GICD_ITARGETSR18: 0x00000000,
        GICD_ITARGETSR19: 0x00000000,
        GICD_ITARGETSR20: 0x00000000,
        GICD_ITARGETSR21: 0x00000000,
        GICD_ITARGETSR22: 0x00000000,
        GICD_ITARGETSR23: 0x00000000,
        GICD_ITARGETSR24: 0x00000000,
        GICD_ITARGETSR25: 0x00000000,
        GICD_ITARGETSR26: 0x00000000,
        GICD_ITARGETSR27: 0x00000000,
        GICD_ITARGETSR28: 0x00000000,
        GICD_ITARGETSR29: 0x00000000,
        GICD_ITARGETSR30: 0x00000000,
        GICD_ITARGETSR31: 0x00000000,
        GICD_ITARGETSR32: 0x00000000,
        GICD_ITARGETSR33: 0x00000000,
        GICD_ITARGETSR34: 0x00000000,
        GICD_ITARGETSR35: 0x00000000,
        GICD_ITARGETSR36: 0x00000000,
        GICD_ITARGETSR37: 0x00000000,
        GICD_ITARGETSR38: 0x00000000,
        GICD_ITARGETSR39: 0x00000000,
        GICD_ITARGETSR40: 0x00000000,
        GICD_ITARGETSR41: 0x00000000,
        GICD_ITARGETSR42: 0x00000000,
        GICD_ITARGETSR43: 0x00000000,
        GICD_ITARGETSR44: 0x00000000,
        GICD_ITARGETSR45: 0x00000000,
        GICD_ITARGETSR46: 0x00000000,
        GICD_ITARGETSR47: 0x00000000,
        GICD_ITARGETSR48: 0x00000000,
        GICD_ITARGETSR49: 0x00000000,
        GICD_ITARGETSR50: 0x00000000,
        GICD_ITARGETSR51: 0x00000000,
        GICD_ITARGETSR52: 0x00000000,
        GICD_ITARGETSR53: 0x00000000,
        GICD_ITARGETSR54: 0x00000000,
        GICD_ITARGETSR55: 0x00000000,
        GICD_ITARGETSR56: 0x00000000,
        GICD_ITARGETSR57: 0x00000000,
        GICD_ITARGETSR58: 0x00000000,
        GICD_ITARGETSR59: 0x00000000,
        GICD_ITARGETSR60: 0x00000000,
        GICD_ITARGETSR61: 0x00000000,
        GICD_ITARGETSR62: 0x00000000,
        GICD_ITARGETSR63: 0x00000000,
        GICD_ITARGETSR64: 0x00000000,
        GICD_ITARGETSR65: 0x00000000,
        GICD_ITARGETSR66: 0x00000000,
        GICD_ITARGETSR67: 0x00000000,
        GICD_ITARGETSR68: 0x00000000,
        GICD_ITARGETSR69: 0x00000000,
        GICD_ITARGETSR70: 0x00000000,
        GICD_ITARGETSR71: 0x00000000,
        GICD_ICFGR0: 0xAAAAAAAA,
        GICD_ICFGR1: 0x55540000,
        GICD_ICFGR2: 0x55555555,
        GICD_ICFGR3: 0x55555555,
        GICD_ICFGR4: 0x55555555,
        GICD_ICFGR5: 0x55555555,
        GICD_ICFGR6: 0x55555555,
        GICD_ICFGR7: 0x55555555,
        GICD_ICFGR8: 0x55555555,
        GICD_ICFGR9: 0x55555555,
        GICD_ICFGR10: 0x55555555,
        GICD_ICFGR11: 0x55555555,
        GICD_ICFGR12: 0x55555555,
        GICD_ICFGR13: 0x55555555,
        GICD_ICFGR14: 0x55555555,
        GICD_ICFGR15: 0x55555555,
        GICD_ICFGR16: 0x55555555,
        GICD_ICFGR17: 0x55555555,
        GICD_PPISR: 0x00000000,
        GICD_SPISR1: 0x00000000,
        GICD_SPISR2: 0x00000000,
        GICD_SPISR3: 0x00000000,
        GICD_SPISR4: 0x00000000,
        GICD_SPISR5: 0x00000000,
        GICD_SPISR6: 0x00000000,
        GICD_SPISR7: 0x00000000,
        GICD_SGIR: 0x00000000,
        GICD_CPENDSGIR0: 0x00000000,
        GICD_CPENDSGIR1: 0x00000000,
        GICD_CPENDSGIR2: 0x00000000,
        GICD_CPENDSGIR3: 0x00000000,
        GICD_SPENDSGIR0: 0x00000000,
        GICD_SPENDSGIR1: 0x00000000,
        GICD_SPENDSGIR2: 0x00000000,
        GICD_SPENDSGIR3: 0x00000000,
        GICD_PIDR4: 0x00000004,
        GICD_PIDR5: 0x00000000,
        GICD_PIDR6: 0x00000000,
        GICD_PIDR7: 0x00000000,
        GICD_PIDR0: 0x00000090,
        GICD_PIDR1: 0x000000B4,
        GICD_PIDR2: 0x0000002B,
        GICD_PIDR3: 0x00000000,
        GICD_CIDR0: 0x0000000D,
        GICD_CIDR1: 0x000000F0,
        GICD_CIDR2: 0x00000005,
        GICD_CIDR3: 0x000000B1,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut GICD_TAKEN: bool = false;

    /// Safe access to GICD
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
            if GICD_TAKEN {
                None
            } else {
                GICD_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to GICD
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if GICD_TAKEN && inst.addr == INSTANCE.addr {
                GICD_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal GICD
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        GICD_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to GICD
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const GICD: *const RegisterBlock = 0xa0021000 as *const _;
