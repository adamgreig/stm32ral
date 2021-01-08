#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! External interrupt/event controller

use crate::{RORegister, RWRegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// rising trigger selection register
pub mod RTSR1 {

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod RT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (22 bits: 0x3fffff << 0)
        pub const mask: u32 = 0x3fffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod RT_31 {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// falling trigger selection register
pub mod FTSR1 {

    /// Falling trigger event configuration bit of Configurable Event input
    pub mod FT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (22 bits: 0x3fffff << 0)
        pub const mask: u32 = 0x3fffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Falling trigger event configuration bit of Configurable Event input
    pub mod FT_31 {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// software interrupt event register
pub mod SWIER1 {

    /// Software interrupt on event
    pub mod SWI {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (22 bits: 0x3fffff << 0)
        pub const mask: u32 = 0x3fffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Software interrupt on event
    pub mod SWI_31 {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// EXTI pending register
pub mod PR1 {

    /// Configurable event inputs Pending bit
    pub mod PIF {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (22 bits: 0x3fffff << 0)
        pub const mask: u32 = 0x3fffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Configurable event inputs Pending bit
    pub mod PIF_31 {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// rising trigger selection register
pub mod RTSR2 {

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod RT33 {
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

    /// Rising trigger event configuration bit of Configurable Event input
    pub mod RT40_41 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (2 bits: 0b11 << 8)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// falling trigger selection register
pub mod FTSR2 {

    /// Falling trigger event configuration bit of Configurable Event input
    pub mod FT33 {
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

    /// Falling trigger event configuration bit of Configurable Event input
    pub mod FT40_41 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (2 bits: 0b11 << 8)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// software interrupt event register
pub mod SWIER2 {

    /// Software interrupt on event
    pub mod SWI33 {
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

    /// Software interrupt on event
    pub mod SWI40_41 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (2 bits: 0b11 << 8)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// pending register
pub mod PR2 {

    /// Configurable event inputs x+32 Pending bit.
    pub mod PIF33 {
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

    /// Configurable event inputs x+32 Pending bit.
    pub mod PIF40_41 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (2 bits: 0b11 << 8)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// CPUm wakeup with interrupt mask register
pub mod C1IMR1 {

    /// CPU(m) wakeup with interrupt Mask on Event input
    pub mod IM {
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

/// CPUm wakeup with interrupt mask register
pub mod C2IMR1 {
    pub use super::C1IMR1::IM;
}

/// CPUm wakeup with event mask register
pub mod C1EMR1 {

    /// CPU(m) Wakeup with event generation Mask on Event input
    pub mod EM0_15 {
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

    /// CPU(m) Wakeup with event generation Mask on Event input
    pub mod EM17_21 {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (5 bits: 0b11111 << 17)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// CPUm wakeup with event mask register
pub mod C2EMR1 {
    pub use super::C1EMR1::EM0_15;
    pub use super::C1EMR1::EM17_21;
}

/// CPUm wakeup with interrupt mask register
pub mod C1IMR2 {

    /// CPUm Wakeup with interrupt Mask on Event input
    pub mod IM {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (17 bits: 0x1ffff << 0)
        pub const mask: u32 = 0x1ffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// CPUm wakeup with interrupt mask register
pub mod C2IMR2 {
    pub use super::C1IMR2::IM;
}

/// CPUm wakeup with event mask register
pub mod C1EMR2 {

    /// CPU(m) Wakeup with event generation Mask on Event input
    pub mod EM {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (2 bits: 0b11 << 8)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// CPUm wakeup with event mask register
pub mod C2EMR2 {
    pub use super::C1EMR2::EM;
}

/// Hardware configuration registers
pub mod HWCFGR5 {

    /// HW configuration CPU event generation
    pub mod CPUEVENT {
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

/// Hardware configuration registers
pub mod HWCFGR6 {
    pub use super::HWCFGR5::CPUEVENT;
}

/// EXTI Hardware configuration registers
pub mod HWCFGR7 {
    pub use super::HWCFGR5::CPUEVENT;
}

/// Hardware configuration registers
pub mod HWCFGR2 {

    /// HW configuration event trigger type
    pub mod EVENT_TRG {
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

/// Hardware configuration registers
pub mod HWCFGR3 {
    pub use super::HWCFGR2::EVENT_TRG;
}

/// Hardware configuration registers
pub mod HWCFGR4 {
    pub use super::HWCFGR2::EVENT_TRG;
}

/// Hardware configuration register 1
pub mod HWCFGR1 {

    /// HW configuration number of event
    pub mod NBEVENTS {
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

    /// HW configuration number of CPUs
    pub mod NBCPUS {
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

    /// HW configuration of CPU(m) event output enable
    pub mod CPUEVTEN {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (4 bits: 0b1111 << 12)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// EXTI IP Version register
pub mod VERR {

    /// Minor Revision number
    pub mod MINREV {
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

    /// Major Revision number
    pub mod MAJREV {
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
}

/// Identification register
pub mod IPIDR {

    /// IP Identification
    pub mod IPID {
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

/// Size ID register
pub mod SIDR {

    /// Size Identification
    pub mod SID {
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
#[repr(C)]
pub struct RegisterBlock {
    /// rising trigger selection register
    pub RTSR1: RWRegister<u32>,

    /// falling trigger selection register
    pub FTSR1: RWRegister<u32>,

    /// software interrupt event register
    pub SWIER1: RWRegister<u32>,

    /// EXTI pending register
    pub PR1: RWRegister<u32>,

    _reserved1: [u32; 4],

    /// rising trigger selection register
    pub RTSR2: RWRegister<u32>,

    /// falling trigger selection register
    pub FTSR2: RWRegister<u32>,

    /// software interrupt event register
    pub SWIER2: RWRegister<u32>,

    /// pending register
    pub PR2: RWRegister<u32>,

    _reserved2: [u32; 20],

    /// CPUm wakeup with interrupt mask register
    pub C1IMR1: RWRegister<u32>,

    /// CPUm wakeup with event mask register
    pub C1EMR1: RWRegister<u32>,

    _reserved3: [u32; 2],

    /// CPUm wakeup with interrupt mask register
    pub C1IMR2: RWRegister<u32>,

    /// CPUm wakeup with event mask register
    pub C1EMR2: RWRegister<u32>,

    _reserved4: [u32; 10],

    /// CPUm wakeup with interrupt mask register
    pub C2IMR1: RWRegister<u32>,

    /// CPUm wakeup with event mask register
    pub C2EMR1: RWRegister<u32>,

    _reserved5: [u32; 2],

    /// CPUm wakeup with interrupt mask register
    pub C2IMR2: RWRegister<u32>,

    /// CPUm wakeup with event mask register
    pub C2EMR2: RWRegister<u32>,

    _reserved6: [u32; 192],

    /// EXTI Hardware configuration registers
    pub HWCFGR7: RORegister<u32>,

    /// Hardware configuration registers
    pub HWCFGR6: RORegister<u32>,

    /// Hardware configuration registers
    pub HWCFGR5: RORegister<u32>,

    /// Hardware configuration registers
    pub HWCFGR4: RORegister<u32>,

    /// Hardware configuration registers
    pub HWCFGR3: RORegister<u32>,

    /// Hardware configuration registers
    pub HWCFGR2: RORegister<u32>,

    /// Hardware configuration register 1
    pub HWCFGR1: RORegister<u32>,

    /// EXTI IP Version register
    pub VERR: RORegister<u32>,

    /// Identification register
    pub IPIDR: RORegister<u32>,

    /// Size ID register
    pub SIDR: RORegister<u32>,
}
pub struct ResetValues {
    pub RTSR1: u32,
    pub FTSR1: u32,
    pub SWIER1: u32,
    pub PR1: u32,
    pub RTSR2: u32,
    pub FTSR2: u32,
    pub SWIER2: u32,
    pub PR2: u32,
    pub C1IMR1: u32,
    pub C1EMR1: u32,
    pub C1IMR2: u32,
    pub C1EMR2: u32,
    pub C2IMR1: u32,
    pub C2EMR1: u32,
    pub C2IMR2: u32,
    pub C2EMR2: u32,
    pub HWCFGR7: u32,
    pub HWCFGR6: u32,
    pub HWCFGR5: u32,
    pub HWCFGR4: u32,
    pub HWCFGR3: u32,
    pub HWCFGR2: u32,
    pub HWCFGR1: u32,
    pub VERR: u32,
    pub IPIDR: u32,
    pub SIDR: u32,
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

/// Access functions for the EXTI peripheral instance
pub mod EXTI {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x58000800,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in EXTI
    pub const reset: ResetValues = ResetValues {
        RTSR1: 0x00000000,
        FTSR1: 0x00000000,
        SWIER1: 0x00000000,
        PR1: 0x00000000,
        RTSR2: 0x00000000,
        FTSR2: 0x00000000,
        SWIER2: 0x00000000,
        PR2: 0x00000000,
        C1IMR1: 0x7FC00000,
        C2IMR1: 0x7FC00000,
        C1EMR1: 0x00000000,
        C2EMR1: 0x00000000,
        C1IMR2: 0x0001FCFD,
        C2IMR2: 0x0001FCFD,
        C1EMR2: 0x00000000,
        C2EMR2: 0x00000000,
        HWCFGR5: 0x003EFFFF,
        HWCFGR6: 0x00000300,
        HWCFGR7: 0x00000000,
        HWCFGR2: 0x803FFFFF,
        HWCFGR3: 0x00000302,
        HWCFGR4: 0x00000000,
        HWCFGR1: 0x00003130,
        VERR: 0x00000020,
        IPIDR: 0x000E0001,
        SIDR: 0xA3C5DD01,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut EXTI_TAKEN: bool = false;

    /// Safe access to EXTI
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
            if EXTI_TAKEN {
                None
            } else {
                EXTI_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to EXTI
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if EXTI_TAKEN && inst.addr == INSTANCE.addr {
                EXTI_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal EXTI
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        EXTI_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to EXTI
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const EXTI: *const RegisterBlock = 0x58000800 as *const _;
