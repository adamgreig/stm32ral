#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! General-purpose timers

use crate::{RWRegister, WORegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// control register 1
pub mod CR1 {

    /// Clock division
    pub mod CKD {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (2 bits: 0b11 << 8)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: t_DTS = t_CK_INT
            pub const Div1: u32 = 0b00;

            /// 0b01: t_DTS = 2 × t_CK_INT
            pub const Div2: u32 = 0b01;

            /// 0b10: t_DTS = 4 × t_CK_INT
            pub const Div4: u32 = 0b10;
        }
    }

    /// Auto-reload preload enable
    pub mod ARPE {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: TIMx_APRR register is not buffered
            pub const Disabled: u32 = 0b0;

            /// 0b1: TIMx_APRR register is buffered
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Update request source
    pub mod URS {
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

            /// 0b0: Any of counter overflow/underflow, setting UG, or update through slave mode, generates an update interrupt or DMA request
            pub const AnyEvent: u32 = 0b0;

            /// 0b1: Only counter overflow/underflow generates an update interrupt or DMA request
            pub const CounterOnly: u32 = 0b1;
        }
    }

    /// Update disable
    pub mod UDIS {
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

            /// 0b0: Update event enabled
            pub const Enabled: u32 = 0b0;

            /// 0b1: Update event disabled
            pub const Disabled: u32 = 0b1;
        }
    }

    /// Counter enable
    pub mod CEN {
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

            /// 0b0: Counter disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Counter enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// One-pulse mode
    pub mod OPM {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Counter is not stopped at update event
            pub const Disabled: u32 = 0b0;

            /// 0b1: Counter stops counting at the next update event (clearing the CEN bit)
            pub const Enabled: u32 = 0b1;
        }
    }
}

/// Interrupt enable register
pub mod DIER {

    /// Update interrupt enable
    pub mod UIE {
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

            /// 0b0: Update interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Update interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Capture/Compare 1 interrupt enable
    pub mod CC1IE {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// status register
pub mod SR {

    /// Capture/compare 1 overcapture flag
    pub mod CC1OF {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Capture/Compare 1 interrupt flag
    pub mod CC1IF {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Update interrupt flag
    pub mod UIF {
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

            /// 0b0: No update occurred
            pub const Clear: u32 = 0b0;

            /// 0b1: Update interrupt pending.
            pub const UpdatePending: u32 = 0b1;
        }
    }
}

/// event generation register
pub mod EGR {

    /// Capture/Compare 1 generation
    pub mod CC1G {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Update generation
    pub mod UG {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Re-initializes the timer counter and generates an update of the registers.
            pub const Update: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// CCMR1_Output and CCMR1_Input
/// CCMR1_Output: capture/compare mode register
/// CCMR1_Input: capture/compare mode register 1 (input mode)
pub mod CCMR1 {

    /// Capture/Compare 1 selection
    pub mod CC1S {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (2 bits: 0b11 << 0)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Output compare 1 fast enable
    pub mod OC1FE {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Output compare 1 preload enable
    pub mod OC1PE {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Output compare 1 mode
    pub mod OC1M {
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

            /// 0b000: The comparison between the output compare register TIMx_CCRy and the counter TIMx_CNT has no effect on the outputs
            pub const Frozen: u32 = 0b000;

            /// 0b001: Set channel to active level on match. OCyREF signal is forced high when the counter matches the capture/compare register
            pub const ActiveOnMatch: u32 = 0b001;

            /// 0b010: Set channel to inactive level on match. OCyREF signal is forced low when the counter matches the capture/compare register
            pub const InactiveOnMatch: u32 = 0b010;

            /// 0b011: OCyREF toggles when TIMx_CNT=TIMx_CCRy
            pub const Toggle: u32 = 0b011;

            /// 0b100: OCyREF is forced low
            pub const ForceInactive: u32 = 0b100;

            /// 0b101: OCyREF is forced high
            pub const ForceActive: u32 = 0b101;

            /// 0b110: In upcounting, channel is active as long as TIMx_CNT<TIMx_CCRy else inactive. In downcounting, channel is inactive as long as TIMx_CNT>TIMx_CCRy else active
            pub const PwmMode1: u32 = 0b110;

            /// 0b111: Inversely to PwmMode1
            pub const PwmMode2: u32 = 0b111;
        }
    }

    /// Output compare 1 clear enable
    pub mod OC1CE {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Input capture 1 filter
    pub mod IC1F {
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

    /// Input capture 1 prescaler
    pub mod IC1PSC {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (2 bits: 0b11 << 2)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// capture/compare enable register
pub mod CCER {

    /// Capture/Compare 1 complementary output Polarity
    pub mod CC1NP {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Capture/Compare 1 output Polarity
    pub mod CC1P {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Capture/Compare 1 output enable
    pub mod CC1E {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// counter
pub mod CNT {

    /// TIM10 counter
    pub mod CNT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// prescaler
pub mod PSC {

    /// TIM9 prescaler
    pub mod PSC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// auto-reload register
pub mod ARR {

    /// Auto-reload value
    pub mod ARR {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// capture/compare register
pub mod CCR1 {

    /// Capture/Compare value
    pub mod CCR {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// option register
pub mod OR {

    /// TIM11 Input 1 remapping capability
    pub mod TI1_RMP {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (2 bits: 0b11 << 0)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Timer11 ETR remap
    pub mod ETR_RMP {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Timer11 Input 1 remap for Routing Interface (RI)
    pub mod TI1_RMP_RI {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// TIM 11 slave mode control register 1
pub mod SMCR {

    /// External trigger filter
    pub mod ETF {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (4 bits: 0b1111 << 8)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// External trigger prescaler
    pub mod ETPS {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (2 bits: 0b11 << 12)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// External clock enable
    pub mod ECE {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// External trigger polarity
    pub mod ETP {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
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
    /// control register 1
    pub CR1: RWRegister<u32>,

    _reserved1: [u8; 4],

    /// TIM 11 slave mode control register 1
    pub SMCR: RWRegister<u32>,

    /// Interrupt enable register
    pub DIER: RWRegister<u32>,

    /// status register
    pub SR: RWRegister<u32>,

    /// event generation register
    pub EGR: WORegister<u32>,

    /// CCMR1_Output and CCMR1_Input
    /// CCMR1_Output: capture/compare mode register
    /// CCMR1_Input: capture/compare mode register 1 (input mode)
    pub CCMR1: RWRegister<u32>,

    _reserved2: [u8; 4],

    /// capture/compare enable register
    pub CCER: RWRegister<u32>,

    /// counter
    pub CNT: RWRegister<u32>,

    /// prescaler
    pub PSC: RWRegister<u32>,

    /// auto-reload register
    pub ARR: RWRegister<u32>,

    _reserved3: [u8; 4],

    /// capture/compare register
    pub CCR1: RWRegister<u32>,

    _reserved4: [u8; 24],

    /// option register
    pub OR: RWRegister<u32>,
}
pub struct ResetValues {
    pub CR1: u32,
    pub SMCR: u32,
    pub DIER: u32,
    pub SR: u32,
    pub EGR: u32,
    pub CCMR1: u32,
    pub CCER: u32,
    pub CNT: u32,
    pub PSC: u32,
    pub ARR: u32,
    pub CCR1: u32,
    pub OR: u32,
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

/// Access functions for the TIM11 peripheral instance
pub mod TIM11 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40011000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in TIM11
    pub const reset: ResetValues = ResetValues {
        CR1: 0x00000000,
        DIER: 0x00000000,
        SR: 0x00000000,
        EGR: 0x00000000,
        CCMR1: 0x00000000,
        CCER: 0x00000000,
        CNT: 0x00000000,
        PSC: 0x00000000,
        ARR: 0x00000000,
        CCR1: 0x00000000,
        OR: 0x00000000,
        SMCR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut TIM11_TAKEN: bool = false;

    /// Safe access to TIM11
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
            if TIM11_TAKEN {
                None
            } else {
                TIM11_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to TIM11
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if TIM11_TAKEN && inst.addr == INSTANCE.addr {
                TIM11_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal TIM11
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        TIM11_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to TIM11
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const TIM11: *const RegisterBlock = 0x40011000 as *const _;
