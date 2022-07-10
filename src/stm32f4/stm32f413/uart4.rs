#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Universal synchronous asynchronous receiver transmitter

use crate::RWRegister;
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// Status register
pub mod SR {

    /// LIN break detection flag
    pub mod LBD {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Transmit data register empty
    pub mod TXE {
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

    /// Transmission complete
    pub mod TC {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Read data register not empty
    pub mod RXNE {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// IDLE line detected
    pub mod IDLE {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Overrun error
    pub mod ORE {
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

    /// Noise detected flag
    pub mod NF {
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

    /// Framing error
    pub mod FE {
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

    /// Parity error
    pub mod PE {
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

/// Data register
pub mod DR {

    /// Data value
    pub mod DR {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (9 bits: 0x1ff << 0)
        pub const mask: u32 = 0x1ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Baud rate register
pub mod BRR {

    /// mantissa of USARTDIV
    pub mod DIV_Mantissa {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (12 bits: 0xfff << 4)
        pub const mask: u32 = 0xfff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// fraction of USARTDIV
    pub mod DIV_Fraction {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Control register 1
pub mod CR1 {

    /// Oversampling mode
    pub mod OVER8 {
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

            /// 0b0: Oversampling by 16
            pub const Oversample16: u32 = 0b0;

            /// 0b1: Oversampling by 8
            pub const Oversample8: u32 = 0b1;
        }
    }

    /// USART enable
    pub mod UE {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: USART prescaler and outputs disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: USART enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Word length
    pub mod M {
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

            /// 0b0: 8 data bits
            pub const M8: u32 = 0b0;

            /// 0b1: 9 data bits
            pub const M9: u32 = 0b1;
        }
    }

    /// Wakeup method
    pub mod WAKE {
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

            /// 0b0: USART wakeup on idle line
            pub const IdleLine: u32 = 0b0;

            /// 0b1: USART wakeup on address mark
            pub const AddressMark: u32 = 0b1;
        }
    }

    /// Parity control enable
    pub mod PCE {
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

            /// 0b0: Parity control disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Parity control enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Parity selection
    pub mod PS {
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

            /// 0b0: Even parity
            pub const Even: u32 = 0b0;

            /// 0b1: Odd parity
            pub const Odd: u32 = 0b1;
        }
    }

    /// PE interrupt enable
    pub mod PEIE {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: PE interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: PE interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// TXE interrupt enable
    pub mod TXEIE {
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

            /// 0b0: TXE interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: TXE interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Transmission complete interrupt enable
    pub mod TCIE {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: TC interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: TC interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// RXNE interrupt enable
    pub mod RXNEIE {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: RXNE interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: RXNE interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// IDLE interrupt enable
    pub mod IDLEIE {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: IDLE interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: IDLE interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Transmitter enable
    pub mod TE {
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

            /// 0b0: Transmitter disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Transmitter enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Receiver enable
    pub mod RE {
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

            /// 0b0: Receiver disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Receiver enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Receiver wakeup
    pub mod RWU {
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

            /// 0b0: Receiver in active mode
            pub const Active: u32 = 0b0;

            /// 0b1: Receiver in mute mode
            pub const Mute: u32 = 0b1;
        }
    }

    /// Send break
    pub mod SBK {
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

            /// 0b0: No break character is transmitted
            pub const NoBreak: u32 = 0b0;

            /// 0b1: Break character transmitted
            pub const Break: u32 = 0b1;
        }
    }
}

/// Control register 2
pub mod CR2 {

    /// LIN mode enable
    pub mod LINEN {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: LIN mode disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: LIN mode enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// STOP bits
    pub mod STOP {
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

    /// LIN break detection interrupt enable
    pub mod LBDIE {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: LIN break detection interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: LIN break detection interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// lin break detection length
    pub mod LBDL {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: 10-bit break detection
            pub const LBDL10: u32 = 0b0;

            /// 0b1: 11-bit break detection
            pub const LBDL11: u32 = 0b1;
        }
    }

    /// Address of the USART node
    pub mod ADD {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Control register 3
pub mod CR3 {

    /// One sample bit method enable
    pub mod ONEBIT {
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

            /// 0b0: Three sample bit method
            pub const Sample3: u32 = 0b0;

            /// 0b1: One sample bit method
            pub const Sample1: u32 = 0b1;
        }
    }

    /// DMA enable transmitter
    pub mod DMAT {
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

            /// 0b0: DMA mode is disabled for transmission
            pub const Disabled: u32 = 0b0;

            /// 0b1: DMA mode is enabled for transmission
            pub const Enabled: u32 = 0b1;
        }
    }

    /// DMA enable receiver
    pub mod DMAR {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: DMA mode is disabled for reception
            pub const Disabled: u32 = 0b0;

            /// 0b1: DMA mode is enabled for reception
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Half-duplex selection
    pub mod HDSEL {
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

            /// 0b0: Half duplex mode is not selected
            pub const FullDuplex: u32 = 0b0;

            /// 0b1: Half duplex mode is selected
            pub const HalfDuplex: u32 = 0b1;
        }
    }

    /// IrDA low-power
    pub mod IRLP {
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

            /// 0b0: Normal mode
            pub const Normal: u32 = 0b0;

            /// 0b1: Low-power mode
            pub const LowPower: u32 = 0b1;
        }
    }

    /// IrDA mode enable
    pub mod IREN {
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

            /// 0b0: IrDA disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: IrDA enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Error interrupt enable
    pub mod EIE {
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

            /// 0b0: Error interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Error interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }
}

/// Guard Time and Prescaler Register
pub mod GTPR {

    /// IrDA Low-Power pulse width peripheral clock prescaler
    pub mod PSC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (8 bits: 0xff << 0)
        pub const mask: u32 = 0xff << offset;
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
    /// Status register
    pub SR: RWRegister<u32>,

    /// Data register
    pub DR: RWRegister<u32>,

    /// Baud rate register
    pub BRR: RWRegister<u32>,

    /// Control register 1
    pub CR1: RWRegister<u32>,

    /// Control register 2
    pub CR2: RWRegister<u32>,

    /// Control register 3
    pub CR3: RWRegister<u32>,

    /// Guard Time and Prescaler Register
    pub GTPR: RWRegister<u32>,
}
pub struct ResetValues {
    pub SR: u32,
    pub DR: u32,
    pub BRR: u32,
    pub CR1: u32,
    pub CR2: u32,
    pub CR3: u32,
    pub GTPR: u32,
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

/// Access functions for the UART4 peripheral instance
pub mod UART4 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40004c00,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in UART4
    pub const reset: ResetValues = ResetValues {
        SR: 0x00C00000,
        DR: 0x00000000,
        BRR: 0x00000000,
        CR1: 0x00000000,
        CR2: 0x00000000,
        CR3: 0x00000000,
        GTPR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut UART4_TAKEN: bool = false;

    /// Safe access to UART4
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
            if UART4_TAKEN {
                None
            } else {
                UART4_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to UART4
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if UART4_TAKEN && inst.addr == INSTANCE.addr {
                UART4_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal UART4
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        UART4_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to UART4
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const UART4: *const RegisterBlock = 0x40004c00 as *const _;
