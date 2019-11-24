#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Power control
//!
//! Used by: stm32l0x2, stm32l0x3

use crate::RWRegister;
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// power control register
pub mod CR {

    /// Low-power deep sleep
    pub mod LPDS {
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

    /// Power down deepsleep
    pub mod PDDS {
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

            /// 0b0: Enter Stop mode when the CPU enters deepsleep
            pub const STOP_MODE: u32 = 0b0;

            /// 0b1: Enter Standby mode when the CPU enters deepsleep
            pub const STANDBY_MODE: u32 = 0b1;
        }
    }

    /// Clear wakeup flag
    pub mod CWUF {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Clear the WUF Wakeup flag after 2 system clock cycles
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Clear standby flag
    pub mod CSBF {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Clear the SBF Standby flag
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Power voltage detector enable
    pub mod PVDE {
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

            /// 0b0: PVD Disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: PVD Enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// PVD level selection
    pub mod PLS {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (3 bits: 0b111 << 5)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: 1.9 V
            pub const V1_9: u32 = 0b000;

            /// 0b001: 2.1 V
            pub const V2_1: u32 = 0b001;

            /// 0b010: 2.3 V
            pub const V2_3: u32 = 0b010;

            /// 0b011: 2.5 V
            pub const V2_5: u32 = 0b011;

            /// 0b100: 2.7 V
            pub const V2_7: u32 = 0b100;

            /// 0b101: 2.9 V
            pub const V2_9: u32 = 0b101;

            /// 0b110: 3.1 V
            pub const V3_1: u32 = 0b110;

            /// 0b111: External input analog voltage (Compare internally to VREFINT)
            pub const External: u32 = 0b111;
        }
    }

    /// Disable backup domain write protection
    pub mod DBP {
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

            /// 0b0: Access to RTC, RTC Backup and RCC CSR registers disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Access to RTC, RTC Backup and RCC CSR registers enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Ultra-low-power mode
    pub mod ULP {
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

            /// 0b0: VREFINT is on in low-power mode
            pub const Enabled: u32 = 0b0;

            /// 0b1: VREFINT is off in low-power mode
            pub const Disabled: u32 = 0b1;
        }
    }

    /// Fast wakeup
    pub mod FWU {
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

            /// 0b0: Low-power modes exit occurs only when VREFINT is ready
            pub const Disabled: u32 = 0b0;

            /// 0b1: VREFINT start up time is ignored when exiting low-power modes
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Voltage scaling range selection
    pub mod VOS {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (2 bits: 0b11 << 11)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b01: 1.8 V (range 1)
            pub const V1_8: u32 = 0b01;

            /// 0b10: 1.5 V (range 2)
            pub const V1_5: u32 = 0b10;

            /// 0b11: 1.2 V (range 3)
            pub const V1_2: u32 = 0b11;
        }
    }

    /// Deep sleep mode with Flash memory kept off
    pub mod DS_EE_KOFF {
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

            /// 0b0: NVM woken up when exiting from Deepsleep mode even if the bit RUN_PD is set
            pub const NVMWakeUp: u32 = 0b0;

            /// 0b1: NVM not woken up when exiting from low-power mode (if the bit RUN_PD is set)
            pub const NVMSleep: u32 = 0b1;
        }
    }

    /// Low power run mode
    pub mod LPRUN {
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

            /// 0b0: Voltage regulator in Main mode in Low-power run mode
            pub const MAIN_MODE: u32 = 0b0;

            /// 0b1: Voltage regulator in low-power mode in Low-power run mode
            pub const LOW_POWER_MODE: u32 = 0b1;
        }
    }

    /// Low-power deepsleep/Sleep/Low-power run
    pub mod LPSDSR {
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

            /// 0b0: Voltage regulator on during Deepsleep/Sleep/Low-power run mode
            pub const MAIN_MODE: u32 = 0b0;

            /// 0b1: Voltage regulator in low-power mode during Deepsleep/Sleep/Low-power run mode
            pub const LOW_POWER_MODE: u32 = 0b1;
        }
    }
}

/// power control/status register
pub mod CSR {

    /// Enable WKUP pin 2
    pub mod EWUP2 {
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

            /// 0b0: WKUP pin 2 is used for general purpose I/Os. An event on the WKUP pin 2 does not wakeup the device from Standby mode
            pub const Disabled: u32 = 0b0;

            /// 0b1: WKUP pin 2 is used for wakeup from Standby mode and forced in input pull down configuration (rising edge on WKUP pin 2 wakes-up the system from Standby mode)
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Enable WKUP pin 1
    pub mod EWUP1 {
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

            /// 0b0: WKUP pin 1 is used for general purpose I/Os. An event on the WKUP pin 1 does not wakeup the device from Standby mode
            pub const Disabled: u32 = 0b0;

            /// 0b1: WKUP pin 1 is used for wakeup from Standby mode and forced in input pull down configuration (rising edge on WKUP pin 1 wakes-up the system from Standby mode)
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Internal voltage reference ready flag
    pub mod VREFINTRDYF {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: VREFINT is OFF
            pub const NotReady: u32 = 0b0;

            /// 0b1: VREFINT is ready
            pub const Ready: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PVD output
    pub mod PVDO {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: VDD is higher than the PVD threshold selected with the PLS\[2:0\] bits
            pub const AboveThreshold: u32 = 0b0;

            /// 0b1: VDD is lower than the PVD threshold selected with the PLS\[2:0\] bits
            pub const BelowThreshold: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Standby flag
    pub mod SBF {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Device has not been in Standby mode
            pub const NoStandbyEvent: u32 = 0b0;

            /// 0b1: Device has been in Standby mode
            pub const StandbyEvent: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Wakeup flag
    pub mod WUF {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No wakeup event occurred
            pub const NoWakeupEvent: u32 = 0b0;

            /// 0b1: A wakeup event was received from the WKUP pin or from the RTC alarm (Alarm A or Alarm B), RTC Tamper event, RTC TimeStamp event or RTC Wakeup)
            pub const WakeupEvent: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Voltage Scaling select flag
    pub mod VOSF {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Regulator is ready in the selected voltage range
            pub const Ready: u32 = 0b0;

            /// 0b1: Regulator voltage output is changing to the required VOS level
            pub const NotReady: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Regulator LP flag
    pub mod REGLPF {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Regulator is ready in Main mode
            pub const Ready: u32 = 0b0;

            /// 0b1: Regulator voltage is in low-power mode
            pub const NotReady: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Enable WKUP pin 3
    pub mod EWUP3 {
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

            /// 0b0: WKUP pin 3 is used for general purpose I/Os. An event on the WKUP pin 3 does not wakeup the device from Standby mode
            pub const Disabled: u32 = 0b0;

            /// 0b1: WKUP pin 3 is used for wakeup from Standby mode and forced in input pull down configuration (rising edge on WKUP pin 3wakes-up the system from Standby mode)
            pub const Enabled: u32 = 0b1;
        }
    }
}
pub struct RegisterBlock {
    /// power control register
    pub CR: RWRegister<u32>,

    /// power control/status register
    pub CSR: RWRegister<u32>,
}
pub struct ResetValues {
    pub CR: u32,
    pub CSR: u32,
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
