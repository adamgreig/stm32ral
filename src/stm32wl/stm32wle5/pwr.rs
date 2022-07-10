#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Power control

use crate::{RORegister, RWRegister, WORegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// Power control register 1
pub mod CR1 {

    /// Low-power run
    pub mod LPR {
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
            pub const MainMode: u32 = 0b0;

            /// 0b1: Voltage regulator in low-power mode in Low-power run mode
            pub const LowPowerMode: u32 = 0b1;
        }
    }

    /// Voltage scaling range selection
    pub mod VOS {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (2 bits: 0b11 << 9)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b01: 1.2 V (range 1)
            pub const V1_2: u32 = 0b01;

            /// 0b10: 1.0 V (range 2)
            pub const V1_0: u32 = 0b10;
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

            /// 0b0: Access to RTC and backup registers disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Access to RTC and backup registers enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Flash memory power down mode during LPSleep for CPU1
    pub mod FPDS {
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

            /// 0b0: Flash memory in Idle mode when system is in LPSleep mode
            pub const Idle: u32 = 0b0;

            /// 0b1: Flash memory in Power-down mode when system is in LPSleep mode
            pub const PowerDown: u32 = 0b1;
        }
    }

    /// Flash memory power down mode during LPRun for CPU1
    pub mod FPDR {
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

            /// 0b0: Flash memory in Idle mode when system is in LPRun mode
            pub const Idle: u32 = 0b0;

            /// 0b1: Flash memory in Power-down mode when system is in LPRun mode
            pub const PowerDown: u32 = 0b1;
        }
    }

    /// sub-GHz SPI NSS source select
    pub mod SUBGHZSPINSSSEL {
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

            /// 0b0: sub-GHz SPI NSS signal driven from PWR_SUBGHZSPICR.NSS (RFBUSYMS functionality enabled)
            pub const SUBGHZSPICR: u32 = 0b0;

            /// 0b1: sub-GHz SPI NSS signal driven from LPTIM3_OUT (RFBUSYMS functionality disabled)
            pub const LPTIM3: u32 = 0b1;
        }
    }

    /// Low-power mode selection for CPU1
    pub mod LPMS {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Stop 0 mode
            pub const Stop0: u32 = 0b000;

            /// 0b001: Stop 1 mode
            pub const Stop1: u32 = 0b001;

            /// 0b010: Stop 2 mode
            pub const Stop2: u32 = 0b010;

            /// 0b011: Standby mode
            pub const Standby: u32 = 0b011;

            /// 0b100: Shutdown mode
            pub const Shutdown: u32 = 0b100;
        }
    }
}

/// Power control register 2
pub mod CR2 {

    /// Peripheral voltage monitoring 3 enable: VDDA vs. 1.62V
    pub mod PVME3 {
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

            /// 0b0: PVM3 (VDDA monitoring versus 1.62 V threshold) disable
            pub const Disabled: u32 = 0b0;

            /// 0b1: PVM3 (VDDA monitoring versus 1.62 V threshold) enable
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Power voltage detector level selection.
    pub mod PLS {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (3 bits: 0b111 << 1)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: 2.0V
            pub const V2_0: u32 = 0b000;

            /// 0b001: 2.2V
            pub const V2_2: u32 = 0b001;

            /// 0b010: 2.4V
            pub const V2_4: u32 = 0b010;

            /// 0b011: 2.5V
            pub const V2_5: u32 = 0b011;

            /// 0b100: 2.6V
            pub const V2_6: u32 = 0b100;

            /// 0b101: 2.8V
            pub const V2_8: u32 = 0b101;

            /// 0b110: 2.9V
            pub const V2_9: u32 = 0b110;

            /// 0b111: External input analog voltage PVD_IN (compared internally to VREFINT)
            pub const External: u32 = 0b111;
        }
    }

    /// Power voltage detector enable
    pub mod PVDE {
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

            /// 0b0: PVD Disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: PVD Enabled
            pub const Enabled: u32 = 0b1;
        }
    }
}

/// Power control register 3
pub mod CR3 {

    /// Enable internal wakeup line for CPU1
    pub mod EIWUL {
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

            /// 0b0: Internal wakeup line interrupt to CPU1 disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Internal wakeup line interrupt to CPU1 enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// akeup for CPU1
    pub mod EWRFIRQ {
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

            /// 0b0: Radio IRQ\[2:0\] is disabled and does not trigger a wakeup from Standby event to CPU1.
            pub const Disabled: u32 = 0b0;

            /// 0b1: Radio IRQ\[2:0\] is enabled and triggers a wakeup from Standby event to CPU1.
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Enable Radio BUSY Wakeup from Standby for CPU1
    pub mod EWRFBUSY {
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

            /// 0b0: Radio Busy is disabled and does not trigger a wakeup from Standby event to CPU1 when a rising or a falling edge occurs
            pub const Disabled: u32 = 0b0;

            /// 0b1: Radio Busy is enabled and triggers a wakeup from Standby event to CPU1 when a rising or a falling edge occurs. The active edge is configured via the WRFBUSYP bit in PWR_CR4
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Apply pull-up and pull-down configuration from CPU1
    pub mod APC {
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

            /// 0b0: I/O pull-up and pull-down configurations defined in the PWR_PUCRx and PWR_PDCRx registers are applied
            pub const Disabled: u32 = 0b0;

            /// 0b1: PWR_PUCRx and PWR_PDCRx registers are NOT applied to the I/Os
            pub const Enabled: u32 = 0b1;
        }
    }

    /// SRAM2 retention in Standby mode
    pub mod RRS {
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

            /// 0b0: SRAM2 powered off in Standby mode (SRAM2 content lost)
            pub const PowerOff: u32 = 0b0;

            /// 0b1: SRAM2 powered by the low-power regulator in Standby mode (SRAM2 content kept)
            pub const OnLPR: u32 = 0b1;
        }
    }

    /// Enable wakeup PVD for CPU1
    pub mod EWPVD {
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

            /// 0b0: PVD not enabled by the sub-GHz radio active state
            pub const Disabled: u32 = 0b0;

            /// 0b1: PVD enabled while the sub-GHz radio is active
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Ultra-low-power enable
    pub mod EULPEN {
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

            /// 0b0: Disable (the supply voltage is monitored continuously)
            pub const Disabled: u32 = 0b0;

            /// 0b1: Enable, when set, the supply voltage is sampled for PDR/BOR reset condition only periodically
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Enable Wakeup pin WKUP3 for CPU1
    pub mod EWUP3 {
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

            /// 0b0: WKUP pin 3 is used for general purpose I/Os. An event on the WKUP pin 3 does not wakeup the device from Standby mode
            pub const Disabled: u32 = 0b0;

            /// 0b1: WKUP pin 3 is used for wakeup from Standby mode and forced in input pull down configuration (rising edge on WKUP pin 3wakes-up the system from Standby mode)
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Enable Wakeup pin WKUP2 for CPU1
    pub mod EWUP2 {
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

            /// 0b0: WKUP pin 2 is used for general purpose I/Os. An event on the WKUP pin 2 does not wakeup the device from Standby mode
            pub const Disabled: u32 = 0b0;

            /// 0b1: WKUP pin 2 is used for wakeup from Standby mode and forced in input pull down configuration (rising edge on WKUP pin 2 wakes-up the system from Standby mode)
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Enable Wakeup pin WKUP1 for CPU1
    pub mod EWUP1 {
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

            /// 0b0: WKUP pin 1 is used for general purpose I/Os. An event on the WKUP pin 1 does not wakeup the device from Standby mode
            pub const Disabled: u32 = 0b0;

            /// 0b1: WKUP pin 1 is used for wakeup from Standby mode and forced in input pull down configuration (rising edge on WKUP pin 1 wakes-up the system from Standby mode)
            pub const Enabled: u32 = 0b1;
        }
    }
}

/// Power control register 4
pub mod CR4 {

    /// Wakeup Radio BUSY polarity
    pub mod WRFBUSYP {
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

            /// 0b0: Detection on high level (rising edge)
            pub const RisingEdge: u32 = 0b0;

            /// 0b1: Detection on low level (falling edge)
            pub const FallingEdge: u32 = 0b1;
        }
    }

    /// VBAT battery charging resistor selection
    pub mod VBRS {
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

            /// 0b0: VBAT charging through a 5 kΩ resistor
            pub const R5k: u32 = 0b0;

            /// 0b1: VBAT charging through a 1.5 kΩ resistor
            pub const R1_5k: u32 = 0b1;
        }
    }

    /// VBAT battery charging enable
    pub mod VBE {
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

            /// 0b0: VBAT battery charging disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: VBAT battery charging enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Wakeup pin WKUP3 polarity
    pub mod WP3 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::WRFBUSYP::RW;
    }

    /// Wakeup pin WKUP2 polarity
    pub mod WP2 {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::WRFBUSYP::RW;
    }

    /// Wakeup pin WKUP1 polarity
    pub mod WP1 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::WRFBUSYP::RW;
    }
}

/// Power status register 1
pub mod SR1 {

    /// Internal wakeup interrupt flag
    pub mod WUFI {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: All internal wakeup sources are cleared
            pub const Clear: u32 = 0b0;

            /// 0b1: wakeup is detected on the internal wakeup line
            pub const Wakeup: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Radio BUSY wakeup flag
    pub mod WRFBUSYF {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No wakeup event detected on radio busy
            pub const Clear: u32 = 0b0;

            /// 0b1: Wakeup event detected on radio busy
            pub const Wakeup: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Wakeup PVD flag
    pub mod WPVDF {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No wakeup event detected on PVD
            pub const Clear: u32 = 0b0;

            /// 0b1: Wakeup event detected on PVD
            pub const Wakeup: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Wakeup flag 3
    pub mod WUF3 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No wakeup event detected on WKUP3
            pub const Clear: u32 = 0b0;

            /// 0b1: Wakeup event detected on WKUP3
            pub const Wakeup: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Wakeup flag 2
    pub mod WUF2 {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No wakeup event detected on WKUP2
            pub const Clear: u32 = 0b0;

            /// 0b1: Wakeup event detected on WKUP2
            pub const Wakeup: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Wakeup flag 1
    pub mod WUF1 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No wakeup event detected on WKUP1
            pub const Clear: u32 = 0b0;

            /// 0b1: Wakeup event detected on WKUP1
            pub const Wakeup: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Power status register 2
pub mod SR2 {

    /// Peripheral voltage monitoring output: VDDA vs. 1.62 V
    pub mod PVMO3 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: VDDA voltage above PVM3 threshold (around 1.62 V)
            pub const Above: u32 = 0b0;

            /// 0b1: VDDA voltage below PVM3 threshold (around 1.62 V)
            pub const Below: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Power voltage detector output
    pub mod PVDO {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: VDD or voltage level on PVD_IN above the selected PVD threshold
            pub const Above: u32 = 0b0;

            /// 0b1: VDD or voltage level on PVD_IN below the selected PVD threshold
            pub const Below: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Voltage scaling flag
    pub mod VOSF {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Regulator ready in the selected voltage range
            pub const Ready: u32 = 0b0;

            /// 0b1: Regulator output voltage changed to the required voltage level
            pub const Change: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// regulator1 low power flag
    pub mod REGLPF {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Main regulator (MR) ready and used
            pub const Main: u32 = 0b0;

            /// 0b1: Low-power regulator (LPR) used
            pub const LowPower: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// regulator1 started
    pub mod REGLPS {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: LPR not ready
            pub const NotReady: u32 = 0b0;

            /// 0b1: LPR ready
            pub const Ready: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Flash ready
    pub mod FLASHRDY {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Flash memory not ready to be accessed
            pub const NotReady: u32 = 0b0;

            /// 0b1: Flash memory ready to be accessed
            pub const Ready: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// regulator2 low power flag
    pub mod REGMRS {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Main regulator supplied directly from VDD
            pub const V_DD: u32 = 0b0;

            /// 0b1: Main regulator supplied through LDO or SMPS
            pub const LDO_SMPS: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Radio end of life flag
    pub mod RFEOLF {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Supply voltage above radio end-of-life operating low level
            pub const Above: u32 = 0b0;

            /// 0b1: Supply voltage below radio end-of-life operating low level
            pub const Below: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// LDO ready flag
    pub mod LDORDY {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: LDO not ready or off
            pub const NotReady: u32 = 0b0;

            /// 0b1: LDO ready
            pub const Ready: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SMPS ready flag
    pub mod SMPSRDY {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: SMPS step-down converter not ready or off
            pub const NotReady: u32 = 0b0;

            /// 0b1: SMPS step-down converter ready
            pub const Ready: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Radio BUSY masked signal status
    pub mod RFBUSYMS {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: radio busy masked signal low (not busy)
            pub const NotBusy: u32 = 0b0;

            /// 0b1: radio busy masked signal high (busy)
            pub const Busy: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Radio BUSY signal status
    pub mod RFBUSYS {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: radio busy signal low (not busy)
            pub const NotBusy: u32 = 0b0;

            /// 0b1: radio busy signal high (busy)
            pub const Busy: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Power status clear register
pub mod SCR {

    /// Clear wakeup Radio BUSY flag
    pub mod CWRFBUSYF {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Setting this bit clears the WRFBUSYF flag in the PWR_SR1. This bit is always read 0.
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Clear wakeup PVD interrupt flag
    pub mod CWPVDF {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Setting this bit clears the WPVDF flag in the PWR_SR1. This bit is always read as 0.
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Clear wakeup flag 3
    pub mod CWUF3 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Setting this bit clears the WUF3 flag in the PWR_SR1 register. This bit is always read as 0.
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Clear wakeup flag 2
    pub mod CWUF2 {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Setting this bit clears the WUF2 flag in the PWR_SR1 register. This bit is always read as 0.
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Clear wakeup flag 1
    pub mod CWUF1 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Setting this bit clears the WUF1 flag in the PWR_SR1 register. This bit is always read as 0.
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Power control register 5
pub mod CR5 {

    /// Enable SMPS Step Down converter SMPS mode enabled.
    pub mod SMPSEN {
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

            /// 0b0: SMPS step-down converter SMPS mode disabled (LDO mode enabled)
            pub const Disabled: u32 = 0b0;

            /// 0b1: SMPS step-down converter SMPS mode enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Enable Radio End Of Life detector enabled
    pub mod RFEOLEN {
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

            /// 0b0: Radio end-of-life detector disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Radio end-of-life detector enabled
            pub const Enabled: u32 = 0b1;
        }
    }
}

/// Power Port A pull-up control register
pub mod PUCRA {

    /// Port PA15 pull-up
    pub mod PU15 {
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

            /// 0b0: Disable pull-up on PA\[y\] when both APC bits are set in PWR control register 3 (PWR_CR3)
            pub const Disabled: u32 = 0b0;

            /// 0b1: Enable pull-up on PA\[y\] when both APC bits are set in PWR control register 3 (PWR_CR3). The pull-up is not activated if the corresponding PA\[y\] bit is also set
            pub const Enabled: u32 = 0b1;
        }
    }

    /// PU14
    pub mod PU14 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PU15::RW;
    }

    /// Port PA\[y\] pull-up bit y (y=0 to 13)
    pub mod PU13 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PU15::RW;
    }

    /// PU12
    pub mod PU12 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PU15::RW;
    }

    /// PU11
    pub mod PU11 {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PU15::RW;
    }

    /// PU10
    pub mod PU10 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PU15::RW;
    }

    /// PU9
    pub mod PU9 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PU15::RW;
    }

    /// PU8
    pub mod PU8 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PU15::RW;
    }

    /// PU7
    pub mod PU7 {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PU15::RW;
    }

    /// PU6
    pub mod PU6 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PU15::RW;
    }

    /// PU5
    pub mod PU5 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PU15::RW;
    }

    /// PU4
    pub mod PU4 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PU15::RW;
    }

    /// PU3
    pub mod PU3 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PU15::RW;
    }

    /// PU2
    pub mod PU2 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PU15::RW;
    }

    /// PU1
    pub mod PU1 {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PU15::RW;
    }

    /// PU0
    pub mod PU0 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PU15::RW;
    }
}

/// Power Port A pull-down control register
pub mod PDCRA {

    /// PD15
    pub mod PD15 {
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

            /// 0b0: Disable the pull-down on PA\[y\] when both APC bits are set in PWR control register 3 (PWR_CR3)
            pub const Disabled: u32 = 0b0;

            /// 0b1: Enable the pull-down on PA\[y\] when both APC bits are set in PWR control register 3 (PWR_CR3)
            pub const Enabled: u32 = 0b1;
        }
    }

    /// ull-down
    pub mod PD14 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD15::RW;
    }

    /// PD13
    pub mod PD13 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD15::RW;
    }

    /// Port PA\[y\] pull-down (y=0 to 12)
    pub mod PD12 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD15::RW;
    }

    /// PD11
    pub mod PD11 {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD15::RW;
    }

    /// PD10
    pub mod PD10 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD15::RW;
    }

    /// PD9
    pub mod PD9 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD15::RW;
    }

    /// PD8
    pub mod PD8 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD15::RW;
    }

    /// PD7
    pub mod PD7 {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD15::RW;
    }

    /// PD6
    pub mod PD6 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD15::RW;
    }

    /// PD5
    pub mod PD5 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD15::RW;
    }

    /// PD4
    pub mod PD4 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD15::RW;
    }

    /// PD3
    pub mod PD3 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD15::RW;
    }

    /// PD2
    pub mod PD2 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD15::RW;
    }

    /// PD1
    pub mod PD1 {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD15::RW;
    }

    /// PD0
    pub mod PD0 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD15::RW;
    }
}

/// Power Port B pull-up control register
pub mod PUCRB {

    /// Port PB\[y\] pull-up (y=0 to 15)
    pub mod PU15 {
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

            /// 0b0: Disable pull-up on PA\[y\] when both APC bits are set in PWR control register 3 (PWR_CR3)
            pub const Disabled: u32 = 0b0;

            /// 0b1: Enable pull-up on PA\[y\] when both APC bits are set in PWR control register 3 (PWR_CR3). The pull-up is not activated if the corresponding PA\[y\] bit is also set
            pub const Enabled: u32 = 0b1;
        }
    }

    /// PU14
    pub mod PU14 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PU15::RW;
    }

    /// PU13
    pub mod PU13 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PU15::RW;
    }

    /// PU12
    pub mod PU12 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PU15::RW;
    }

    /// PU11
    pub mod PU11 {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PU15::RW;
    }

    /// PU10
    pub mod PU10 {
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

            /// 0b0: Disable pull-up on PB\[y\] when both APC bits are set in PWR control register 3 (PWR_CR3)
            pub const Disabled: u32 = 0b0;

            /// 0b1: Enable pull-up on PB\[y\] when both APC bits are set in PWR control register 3 (PWR_CR3). The pull-up is not activated if the corresponding PB\[y\] bit is also set
            pub const Enabled: u32 = 0b1;
        }
    }

    /// PU9
    pub mod PU9 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PU15::RW;
    }

    /// PU8
    pub mod PU8 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PU15::RW;
    }

    /// PU7
    pub mod PU7 {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PU15::RW;
    }

    /// PU6
    pub mod PU6 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PU15::RW;
    }

    /// PU5
    pub mod PU5 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PU15::RW;
    }

    /// PU4
    pub mod PU4 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PU15::RW;
    }

    /// PU3
    pub mod PU3 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PU15::RW;
    }

    /// PU2
    pub mod PU2 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PU15::RW;
    }

    /// PU1
    pub mod PU1 {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PU15::RW;
    }

    /// PU0
    pub mod PU0 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PU10::RW;
    }
}

/// Power Port B pull-down control register
pub mod PDCRB {

    /// Port PB\[y\] pull-down (y=5 to 15)
    pub mod PD15 {
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

            /// 0b0: Disable the pull-down on PA\[y\] when both APC bits are set in PWR control register 3 (PWR_CR3)
            pub const Disabled: u32 = 0b0;

            /// 0b1: Enable the pull-down on PA\[y\] when both APC bits are set in PWR control register 3 (PWR_CR3)
            pub const Enabled: u32 = 0b1;
        }
    }

    /// PD14
    pub mod PD14 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD15::RW;
    }

    /// PD13
    pub mod PD13 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD15::RW;
    }

    /// PD12
    pub mod PD12 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD15::RW;
    }

    /// PD11
    pub mod PD11 {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD15::RW;
    }

    /// PD10
    pub mod PD10 {
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

            /// 0b0: Disable the pull-down on PB\[y\] when both APC bits are set in PWR control register 3 (PWR_CR3)
            pub const Disabled: u32 = 0b0;

            /// 0b1: Enable the pull-down on PB\[y\] when both APC bits are set in PWR control register 3 (PWR_CR3)
            pub const Enabled: u32 = 0b1;
        }
    }

    /// PD9
    pub mod PD9 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD15::RW;
    }

    /// PD8
    pub mod PD8 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD15::RW;
    }

    /// PD7
    pub mod PD7 {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD15::RW;
    }

    /// PD6
    pub mod PD6 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD15::RW;
    }

    /// PD5
    pub mod PD5 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD15::RW;
    }

    /// PD4
    pub mod PD4 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD15::RW;
    }

    /// Port PB\[y\] pull-down (y=0 to 3)
    pub mod PD3 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD15::RW;
    }

    /// PD2
    pub mod PD2 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD15::RW;
    }

    /// PD1
    pub mod PD1 {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD15::RW;
    }

    /// PD0
    pub mod PD0 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD10::RW;
    }
}

/// Power Port C pull-up control register
pub mod PUCRC {

    /// Port PC\[y\] pull-up (y=13 to 15)
    pub mod PU15 {
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

            /// 0b0: Disable pull-up on PC\[y\] when both APC bits are set in PWR control register 3 (PWR_CR3)
            pub const Disabled: u32 = 0b0;

            /// 0b1: Enable pull-up on PC\[y\] when both APC bits are set in PWR control register 3 (PWR_CR3). The pull-up is not activated if the corresponding PC\[y\] bit is also set
            pub const Enabled: u32 = 0b1;
        }
    }

    /// PU14
    pub mod PU14 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PU15::RW;
    }

    /// PU13
    pub mod PU13 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PU15::RW;
    }

    /// PU2
    pub mod PU2 {
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

            /// 0b0: Disable pull-up on PA\[y\] when both APC bits are set in PWR control register 3 (PWR_CR3)
            pub const Disabled: u32 = 0b0;

            /// 0b1: Enable pull-up on PA\[y\] when both APC bits are set in PWR control register 3 (PWR_CR3). The pull-up is not activated if the corresponding PA\[y\] bit is also set
            pub const Enabled: u32 = 0b1;
        }
    }

    /// PU1
    pub mod PU1 {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PU2::RW;
    }

    /// PU0
    pub mod PU0 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PU15::RW;
    }

    /// PU3
    pub mod PU3 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PU2::RW;
    }

    /// PU4
    pub mod PU4 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PU2::RW;
    }

    /// PU5
    pub mod PU5 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PU2::RW;
    }

    /// PU6
    pub mod PU6 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PU2::RW;
    }
}

/// Power Port C pull-down control register
pub mod PDCRC {

    /// Port PC\[y\] pull-down (y=13 to 15)
    pub mod PD15 {
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

            /// 0b0: Disable the pull-down on PC\[y\] when both APC bits are set in PWR control register 3 (PWR_CR3)
            pub const Disabled: u32 = 0b0;

            /// 0b1: Enable the pull-down on PC\[y\] when both APC bits are set in PWR control register 3 (PWR_CR3)
            pub const Enabled: u32 = 0b1;
        }
    }

    /// PD14
    pub mod PD14 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD15::RW;
    }

    /// PD13
    pub mod PD13 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD15::RW;
    }

    /// PD2
    pub mod PD2 {
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

            /// 0b0: Disable the pull-down on PA\[y\] when both APC bits are set in PWR control register 3 (PWR_CR3)
            pub const Disabled: u32 = 0b0;

            /// 0b1: Enable the pull-down on PA\[y\] when both APC bits are set in PWR control register 3 (PWR_CR3)
            pub const Enabled: u32 = 0b1;
        }
    }

    /// PD1
    pub mod PD1 {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD2::RW;
    }

    /// PD0
    pub mod PD0 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD15::RW;
    }

    /// PD3
    pub mod PD3 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD2::RW;
    }

    /// PD4
    pub mod PD4 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD2::RW;
    }

    /// PD5
    pub mod PD5 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD2::RW;
    }

    /// PD6
    pub mod PD6 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PD2::RW;
    }
}

/// Power Port H pull-up control register
pub mod PUCRH {

    /// pull-up
    pub mod PU3 {
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

            /// 0b0: Disable pull-up on PH\[y\] when both APC bits are set in PWR control register 3 (PWR_CR3)
            pub const Disabled: u32 = 0b0;

            /// 0b1: Enable pull-up on PH\[y\] when both APC bits are set in PWR control register 3 (PWR_CR3). The pull-up is not activated if the corresponding PH\[y\] bit is also set
            pub const Enabled: u32 = 0b1;
        }
    }
}

/// Power Port H pull-down control register
pub mod PDCRH {

    /// pull-down
    pub mod PD3 {
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

            /// 0b0: Disable the pull-down on PH\[y\] when both APC bits are set in PWR control register 3 (PWR_CR3)
            pub const Disabled: u32 = 0b0;

            /// 0b1: Enable the pull-down on PH\[y\] when both APC bits are set in PWR control register 3 (PWR_CR3)
            pub const Enabled: u32 = 0b1;
        }
    }
}

/// Power extended status and status clear register
pub mod EXTSCR {

    /// CPU1 deepsleep mode
    pub mod C1DS {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: CPU is running or in sleep
            pub const RunningOrSleep: u32 = 0b0;

            /// 0b1: CPU is in Deep-Sleep
            pub const DeepSleep: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// System Stop0, 1 flag for CPU1. (All core states retained)
    pub mod C1STOPF {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: System has not been in Stop 0 or 1 mode
            pub const NoStop: u32 = 0b0;

            /// 0b1: System has been in Stop 0 or 1 mode
            pub const Stop: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// System Stop2 flag for CPU1. (partial core states retained)
    pub mod C1STOP2F {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: System has not been in Stop 2 mode
            pub const NoStop: u32 = 0b0;

            /// 0b1: System has been in Stop 2 mode
            pub const Stop: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// System Standby flag for CPU1. (no core states retained)
    pub mod C1SBF {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: System has not been in Standby mode
            pub const NoStandby: u32 = 0b0;

            /// 0b1: System has been in Standby mode
            pub const Standby: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Clear CPU1 Stop Standby flags
    pub mod C1CSSF {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Setting this bit clears the C1STOPF and C1SBF bits
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Power SPI3 control register
pub mod SUBGHZSPICR {

    /// sub-GHz SPI NSS control
    pub mod NSS {
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

            /// 0b0: Sub-GHz SPI NSS signal at level low
            pub const Low: u32 = 0b0;

            /// 0b1: Sub-GHz SPI NSS signal is at level high
            pub const High: u32 = 0b1;
        }
    }
}
#[repr(C)]
pub struct RegisterBlock {
    /// Power control register 1
    pub CR1: RWRegister<u32>,

    /// Power control register 2
    pub CR2: RWRegister<u32>,

    /// Power control register 3
    pub CR3: RWRegister<u32>,

    /// Power control register 4
    pub CR4: RWRegister<u32>,

    /// Power status register 1
    pub SR1: RORegister<u32>,

    /// Power status register 2
    pub SR2: RORegister<u32>,

    /// Power status clear register
    pub SCR: WORegister<u32>,

    /// Power control register 5
    pub CR5: RWRegister<u32>,

    /// Power Port A pull-up control register
    pub PUCRA: RWRegister<u32>,

    /// Power Port A pull-down control register
    pub PDCRA: RWRegister<u32>,

    /// Power Port B pull-up control register
    pub PUCRB: RWRegister<u32>,

    /// Power Port B pull-down control register
    pub PDCRB: RWRegister<u32>,

    /// Power Port C pull-up control register
    pub PUCRC: RWRegister<u32>,

    /// Power Port C pull-down control register
    pub PDCRC: RWRegister<u32>,

    _reserved1: [u8; 32],

    /// Power Port H pull-up control register
    pub PUCRH: RWRegister<u32>,

    /// Power Port H pull-down control register
    pub PDCRH: RWRegister<u32>,

    _reserved2: [u8; 40],

    /// Power extended status and status clear register
    pub EXTSCR: RWRegister<u32>,

    _reserved3: [u8; 4],

    /// Power SPI3 control register
    pub SUBGHZSPICR: RWRegister<u32>,
}
pub struct ResetValues {
    pub CR1: u32,
    pub CR2: u32,
    pub CR3: u32,
    pub CR4: u32,
    pub SR1: u32,
    pub SR2: u32,
    pub SCR: u32,
    pub CR5: u32,
    pub PUCRA: u32,
    pub PDCRA: u32,
    pub PUCRB: u32,
    pub PDCRB: u32,
    pub PUCRC: u32,
    pub PDCRC: u32,
    pub PUCRH: u32,
    pub PDCRH: u32,
    pub EXTSCR: u32,
    pub SUBGHZSPICR: u32,
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

/// Access functions for the PWR peripheral instance
pub mod PWR {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x58000400,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in PWR
    pub const reset: ResetValues = ResetValues {
        CR1: 0x00000200,
        CR2: 0x00000000,
        CR3: 0x00008000,
        CR4: 0x00000000,
        SR1: 0x00000000,
        SR2: 0x00000000,
        SCR: 0x00000000,
        CR5: 0x00000000,
        PUCRA: 0x00000000,
        PDCRA: 0x00000000,
        PUCRB: 0x00000000,
        PDCRB: 0x00000000,
        PUCRC: 0x00000000,
        PDCRC: 0x00000000,
        PUCRH: 0x00000000,
        PDCRH: 0x00000000,
        EXTSCR: 0x00000000,
        SUBGHZSPICR: 0x00008000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut PWR_TAKEN: bool = false;

    /// Safe access to PWR
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
            if PWR_TAKEN {
                None
            } else {
                PWR_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to PWR
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if PWR_TAKEN && inst.addr == INSTANCE.addr {
                PWR_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal PWR
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        PWR_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to PWR
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const PWR: *const RegisterBlock = 0x58000400 as *const _;
