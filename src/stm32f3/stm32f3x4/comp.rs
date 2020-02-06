#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! General purpose comparators

use crate::RWRegister;
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// control and status register
pub mod COMP2_CSR {

    /// Comparator 2 enable
    pub mod COMP2EN {
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

            /// 0b0: Comparator disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Comparator enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Comparator 2 inverting input selection
    pub mod COMP2INMSEL {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (3 bits: 0b111 << 4)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: 1/4 of VRefint
            pub const OneQuarterVRef: u32 = 0b000;

            /// 0b001: 1/2 of VRefint
            pub const OneHalfVRef: u32 = 0b001;

            /// 0b010: 3/4 of VRefint
            pub const ThreeQuarterVRef: u32 = 0b010;

            /// 0b011: VRefint
            pub const VRef: u32 = 0b011;

            /// 0b100: PA4 or DAC1_CH1 output if enabled
            pub const PA4_DAC1_CH1: u32 = 0b100;

            /// 0b101: DAC1_CH2
            pub const DAC1_CH2: u32 = 0b101;

            /// 0b110: PA2
            pub const PA2: u32 = 0b110;
        }
    }

    /// Comparator 2 output selection
    pub mod COMP2OUTSEL {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (4 bits: 0b1111 << 10)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: No selection
            pub const NoSelection: u32 = 0b0000;

            /// 0b0001: Timer 1 break input
            pub const Timer1BreakInput: u32 = 0b0001;

            /// 0b0010: Timer 1 break input 2
            pub const Timer1BreakInput2: u32 = 0b0010;

            /// 0b0110: Timer 1 OCREF_CLR input
            pub const Timer1OCRefClearInput: u32 = 0b0110;

            /// 0b0111: Timer 1 input capture 1
            pub const Timer1InputCapture1: u32 = 0b0111;

            /// 0b1000: Timer 2 input capture 4
            pub const Timer2InputCapture4: u32 = 0b1000;

            /// 0b1001: Timer 2 OCREF_CLR input
            pub const Timer2OCRefClearInput: u32 = 0b1001;

            /// 0b1010: Timer 3 input capture 1
            pub const Timer3InputCapture1: u32 = 0b1010;

            /// 0b1011: Timer 3 OCREF_CLR input
            pub const Timer3OCRefClearInput: u32 = 0b1011;
        }
    }

    /// Comparator 2 output polarity
    pub mod COMP2POL {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Output is not inverted
            pub const NotInverted: u32 = 0b0;

            /// 0b1: Output is inverted
            pub const Inverted: u32 = 0b1;
        }
    }

    /// Comparator 2 blanking source
    pub mod COMP2_BLANKING {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (3 bits: 0b111 << 18)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: No blanking
            pub const NoBlanking: u32 = 0b000;

            /// 0b001: TIM1 OC5 selected as blanking source
            pub const TIM1OC5: u32 = 0b001;

            /// 0b010: TIM2 OC3 selected as blanking source
            pub const TIM2OC3: u32 = 0b010;

            /// 0b011: TIM3 OC3 selected as blanking source
            pub const TIM3OC3: u32 = 0b011;
        }
    }

    /// Comparator 2 output
    pub mod COMP2OUT {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Non-inverting input below inverting input
            pub const Low: u32 = 0b0;

            /// 0b1: Non-inverting input above inverting input
            pub const High: u32 = 0b1;
        }
    }

    /// Comparator 2 lock
    pub mod COMP2LOCK {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Comparator CSR bits are read-write
            pub const Unlocked: u32 = 0b0;

            /// 0b1: Comparator CSR bits are read-only
            pub const Locked: u32 = 0b1;
        }
    }

    /// Comparator 2 inverting input selection
    pub mod COMP2INMSEL3 {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// control and status register
pub mod COMP4_CSR {

    /// Comparator 4 enable
    pub mod COMP4EN {
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

            /// 0b0: Comparator disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Comparator enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Comparator 4 inverting input selection
    pub mod COMP4INMSEL {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (3 bits: 0b111 << 4)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: 1/4 of VRefint
            pub const OneQuarterVRef: u32 = 0b000;

            /// 0b001: 1/2 of VRefint
            pub const OneHalfVRef: u32 = 0b001;

            /// 0b010: 3/4 of VRefint
            pub const ThreeQuarterVRef: u32 = 0b010;

            /// 0b011: VRefint
            pub const VRef: u32 = 0b011;

            /// 0b100: PA4 or DAC1_CH1 output if enabled
            pub const PA4_DAC1_CH1: u32 = 0b100;

            /// 0b101: DAC1_CH2
            pub const DAC1_CH2: u32 = 0b101;

            /// 0b111: PB2
            pub const PB2: u32 = 0b111;
        }
    }

    /// Comparator 4 output selection
    pub mod COMP4OUTSEL {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (4 bits: 0b1111 << 10)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: No selection
            pub const NoSelection: u32 = 0b0000;

            /// 0b0001: Timer 1 break input
            pub const Timer1BreakInput: u32 = 0b0001;

            /// 0b0010: Timer 1 break input 2
            pub const Timer1BreakInput2: u32 = 0b0010;

            /// 0b0110: Timer 3 input capture 3
            pub const Timer3InputCapture3: u32 = 0b0110;

            /// 0b1000: Timer 15 input capture 2
            pub const Timer15InputCapture2: u32 = 0b1000;

            /// 0b1010: Timer 15 OCREF_CLR input
            pub const Timer15OCRefClearInput: u32 = 0b1010;

            /// 0b1011: Timer 3 OCREF_CLR input
            pub const Timer3OCRefClearInput: u32 = 0b1011;
        }
    }

    /// Comparator 4 output polarity
    pub mod COMP4POL {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Output is not inverted
            pub const NotInverted: u32 = 0b0;

            /// 0b1: Output is inverted
            pub const Inverted: u32 = 0b1;
        }
    }

    /// Comparator 4 blanking source
    pub mod COMP4_BLANKING {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (3 bits: 0b111 << 18)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: No blanking
            pub const NoBlanking: u32 = 0b000;

            /// 0b001: TIM3 OC4 selected as blanking source
            pub const TIM3OC4: u32 = 0b001;

            /// 0b011: TIM15 OC1 selected as blanking source
            pub const TIM15OC1: u32 = 0b011;
        }
    }

    /// Comparator 4 output
    pub mod COMP4OUT {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Non-inverting input below inverting input
            pub const Low: u32 = 0b0;

            /// 0b1: Non-inverting input above inverting input
            pub const High: u32 = 0b1;
        }
    }

    /// Comparator 4 lock
    pub mod COMP4LOCK {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Comparator CSR bits are read-write
            pub const Unlocked: u32 = 0b0;

            /// 0b1: Comparator CSR bits are read-only
            pub const Locked: u32 = 0b1;
        }
    }

    /// Comparator 4 inverting input selection
    pub mod COMP4INMSEL3 {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// control and status register
pub mod COMP6_CSR {

    /// Comparator 6 enable
    pub mod COMP6EN {
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

            /// 0b0: Comparator disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Comparator enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Comparator 6 inverting input selection
    pub mod COMP6INMSEL {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (3 bits: 0b111 << 4)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: 1/4 of VRefint
            pub const OneQuarterVRef: u32 = 0b000;

            /// 0b001: 1/2 of VRefint
            pub const OneHalfVRef: u32 = 0b001;

            /// 0b010: 3/4 of VRefint
            pub const ThreeQuarterVRef: u32 = 0b010;

            /// 0b011: VRefint
            pub const VRef: u32 = 0b011;

            /// 0b100: PA4 or DAC1_CH1 output if enabled
            pub const PA4_DAC1_CH1: u32 = 0b100;

            /// 0b101: DAC1_CH2
            pub const DAC1_CH2: u32 = 0b101;

            /// 0b111: PB15
            pub const PB15: u32 = 0b111;
        }
    }

    /// Comparator 6 output selection
    pub mod COMP6OUTSEL {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (4 bits: 0b1111 << 10)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: No selection
            pub const NoSelection: u32 = 0b0000;

            /// 0b0001: Timer 1 break input
            pub const Timer1BreakInput: u32 = 0b0001;

            /// 0b0010: Timer 1 break input 2
            pub const Timer1BreakInput2: u32 = 0b0010;

            /// 0b0110: Timer 2 input capture 2
            pub const Timer2InputCapture2: u32 = 0b0110;

            /// 0b1000: Timer 2 OCREF_CLR input
            pub const Timer2OCRefClearInput: u32 = 0b1000;

            /// 0b1001: Timer 16 OCREF_CLR input
            pub const Timer16OCRefClearInput: u32 = 0b1001;

            /// 0b1010: Timer 16 input capture 1
            pub const Timer16InputCapture1: u32 = 0b1010;
        }
    }

    /// Comparator 6 output polarity
    pub mod COMP6POL {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Output is not inverted
            pub const NotInverted: u32 = 0b0;

            /// 0b1: Output is inverted
            pub const Inverted: u32 = 0b1;
        }
    }

    /// Comparator 6 blanking source
    pub mod COMP6_BLANKING {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (3 bits: 0b111 << 18)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: No blanking
            pub const NoBlanking: u32 = 0b000;

            /// 0b011: TIM2 OC4 selected as blanking source
            pub const TIM2OC4: u32 = 0b011;

            /// 0b100: TIM15 OC2 selected as blanking source
            pub const TIM15OC2: u32 = 0b100;
        }
    }

    /// Comparator 6 output
    pub mod COMP6OUT {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Non-inverting input below inverting input
            pub const Low: u32 = 0b0;

            /// 0b1: Non-inverting input above inverting input
            pub const High: u32 = 0b1;
        }
    }

    /// Comparator 6 lock
    pub mod COMP6LOCK {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Comparator CSR bits are read-write
            pub const Unlocked: u32 = 0b0;

            /// 0b1: Comparator CSR bits are read-only
            pub const Locked: u32 = 0b1;
        }
    }

    /// Comparator 6 inverting input selection
    pub mod COMP6INMSEL3 {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}
pub struct RegisterBlock {
    _reserved1: [u32; 8],

    /// control and status register
    pub COMP2_CSR: RWRegister<u32>,

    _reserved2: [u32; 1],

    /// control and status register
    pub COMP4_CSR: RWRegister<u32>,

    _reserved3: [u32; 1],

    /// control and status register
    pub COMP6_CSR: RWRegister<u32>,
}
pub struct ResetValues {
    pub COMP2_CSR: u32,
    pub COMP4_CSR: u32,
    pub COMP6_CSR: u32,
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

/// Access functions for the COMP peripheral instance
pub mod COMP {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40010000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in COMP
    pub const reset: ResetValues = ResetValues {
        COMP2_CSR: 0x00000000,
        COMP4_CSR: 0x00000000,
        COMP6_CSR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut COMP_TAKEN: bool = false;

    /// Safe access to COMP
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
            if COMP_TAKEN {
                None
            } else {
                COMP_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to COMP
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if COMP_TAKEN && inst.addr == INSTANCE.addr {
                COMP_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal COMP
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        COMP_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to COMP
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const COMP: *const RegisterBlock = 0x40010000 as *const _;
