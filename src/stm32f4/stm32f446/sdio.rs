#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Secure digital input/output SD/SDIO MMC card host interface

use crate::{RORegister, RWRegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// SDIO power control register
pub mod POWER {

    /// Power supply control bits. These bits are used to define the current functional state of the card clock
    pub mod PWRCTRL {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (2 bits: 0b11 << 0)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Power off
            pub const PowerOff: u32 = 0b00;

            /// 0b11: Power on
            pub const PowerOn: u32 = 0b11;
        }
    }
}

/// CLKCR register controls the SDIO_CK output clock.
pub mod CLKCR {

    /// Clock divide factor. This field defines the divide factor between the input clock (SDIOCLK) and the output clock (SDIO_CK): SDIO_CK frequency = SDIOCLK / \[CLKDIV + 2\]. While the SD/SDIO card or MultiMediaCard is in identification mode, the SDIO_CK frequency must be less than 400 kHz. The clock frequency can be changed to the maximum card bus frequency when relative card addresses are assigned to all cards
    pub mod CLKDIV {
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

    /// Power saving configuration bit. For power saving, the SDIO_CK clock output can be disabled when the bus is idle by setting PWRSAV
    pub mod PWRSAV {
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

            /// 0b0: SDIO_CK clock is always enabled
            pub const Enabled: u32 = 0b0;

            /// 0b1: SDIO_CK is only enabled when the bus is active
            pub const Disabled: u32 = 0b1;
        }
    }

    /// Wide bus mode enable bit This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0)
    pub mod WIDBUS {
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

            /// 0b00: 1 lane wide bus
            pub const BusWidth1: u32 = 0b00;

            /// 0b01: 4 lane wide bus
            pub const BusWidth4: u32 = 0b01;

            /// 0b10: 8 lane wide bus
            pub const BusWidth8: u32 = 0b10;
        }
    }

    /// SDIO_CK dephasing selection bit. When BYPASS is active, the data and the command change on SDIOCLK falling edge whatever NEGEDGE value
    pub mod NEGEDGE {
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

            /// 0b0: SDIO_CK generated on the rising edge
            pub const Rising: u32 = 0b0;

            /// 0b1: SDIO_CK generated on the falling edge
            pub const Falling: u32 = 0b1;
        }
    }

    /// HW Flow Control enable. When HW Flow Control is enabled, the meaning of the TXFIFOE and RXFIFOF interrupt signals, see SDIO Status register definition in Section 29.8.11
    pub mod HWFC_EN {
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

            /// 0b0: HW Flow Control is disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: HW Flow Control is enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Clock enable bit
    pub mod CLKEN {
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

            /// 0b0: Disable clock
            pub const Disabled: u32 = 0b0;

            /// 0b1: Enable clock
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Clock divider bypass enable bit
    pub mod BYPASS {
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

            /// 0b0: SDIOCLK is divided according to the CLKDIV value before driving the SDIO_CK output signal.
            pub const Disabled: u32 = 0b0;

            /// 0b1: SDIOCLK directly drives the SDIO_CK output signal
            pub const Enabled: u32 = 0b1;
        }
    }
}

/// The SDMMC_ARGR register contains a 32-bit command argument, which is sent to a card as part of a command message.
pub mod ARG {

    /// Command argument. These bits can only be written by firmware when CPSM is disabled (CPSMEN = 0). Command argument sent to a card as part of a command message. If a command contains an argument, it must be loaded into this register before writing a command to the command register.
    pub mod CMDARG {
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

/// The SDMMC_CMDR register contains the command index and command type bits. The command index is sent to a card as part of a command message. The command type bits control the command path state machine (CPSM).
pub mod CMD {

    /// Command index. This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). The command index is sent to the card as part of a command message.
    pub mod CMDINDEX {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (6 bits: 0x3f << 0)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Wait for response bits. This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). They are used to configure whether the CPSM is to wait for a response, and if yes, which kind of response.
    pub mod WAITRESP {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (2 bits: 0b11 << 6)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: No response
            pub const NoResponse: u32 = 0b00;

            /// 0b01: Short response
            pub const ShortResponse: u32 = 0b01;

            /// 0b10: No reponse
            pub const NoResponse2: u32 = 0b10;

            /// 0b11: Long reponse
            pub const LongResponse: u32 = 0b11;
        }
    }

    /// CPSM waits for interrupt request. If this bit is set, the CPSM disables command timeout and waits for an card interrupt request (Response). If this bit is cleared in the CPSM Wait state, will cause the abort of the interrupt mode.
    pub mod WAITINT {
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

            /// 0b0: Don't wait for interrupt request
            pub const Disabled: u32 = 0b0;

            /// 0b1: Wait for interrupt request
            pub const Enabled: u32 = 0b1;
        }
    }

    /// PSM Waits for ends of data transfer (CmdPend internal signal). If this bit is set, the CPSM waits for the end of data transfer before it starts sending a command. This feature is available only with Stream data transfer mode SDIO_DCTRL\[2\] = 1.
    pub mod WAITPEND {
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

            /// 0b0: Don't wait for data end
            pub const Disabled: u32 = 0b0;

            /// 0b1: Wait for end of data transfer signal before sending command
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Command path state machine (CPSM) Enable bit This bit is written 1 by firmware, and cleared by hardware when the CPSM enters the Idle state. If this bit is set, the CPSM is enabled. When DTEN = 1, no command will be transfered nor boot procedure will be started. CPSMEN is cleared to 0.
    pub mod CPSMEN {
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

            /// 0b0: Command path state machine disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Command path state machine enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// SD I/O suspend command. If this bit is set, the command to be sent is a suspend command (to be used only with SDIO card)
    pub mod SDIOSuspend {
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

            /// 0b0: Next command is not a SDIO suspend command
            pub const Disabled: u32 = 0b0;

            /// 0b1: Next command send is a SDIO suspend command
            pub const Enabled: u32 = 0b1;
        }
    }
}

/// The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response.
pub mod RESP1 {

    /// see Table404.
    pub mod CARDSTATUS {
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

/// The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response.
pub mod RESP2 {
    pub use super::RESP1::CARDSTATUS;
}

/// The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response.
pub mod RESP3 {
    pub use super::RESP1::CARDSTATUS;
}

/// The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response.
pub mod RESP4 {
    pub use super::RESP1::CARDSTATUS;
}

/// The SDMMC_DTIMER register contains the data timeout period, in card bus clock periods. A counter loads the value from the SDMMC_DTIMER register, and starts decrementing when the data path state machine (DPSM) enters the Wait_R or Busy state. If the timer reaches 0 while the DPSM is in either of these states, the timeout status flag is set.
pub mod DTIMER {

    /// Data and R1b busy timeout period This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0). Data and R1b busy timeout period expressed in card bus clock periods.
    pub mod DATATIME {
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

/// The SDMMC_DLENR register contains the number of data bytes to be transferred. The value is loaded into the data counter when data transfer starts.
pub mod DLEN {

    /// Data length value This register can only be written by firmware when DPSM is inactive (DPSMACT = 0). Number of data bytes to be transferred. When DDR = 1 DATALENGTH is truncated to a multiple of 2. (The last odd byte is not transfered) When DATALENGTH = 0 no data will be transfered, when requested by a CPSMEN and CMDTRANS = 1 also no command will be transfered. DTEN and CPSMEN are cleared to 0.
    pub mod DATALENGTH {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (25 bits: 0x1ffffff << 0)
        pub const mask: u32 = 0x1ffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// The SDMMC_DCTRL register control the data path state machine (DPSM).
pub mod DCTRL {

    /// Data transfer enable bit This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0). This bit is cleared by Hardware when data transfer completes. This bit shall only be used to transfer data when no associated data transfer command is used, i.e. shall not be used with SD or eMMC cards.
    pub mod DTEN {
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

            /// 0b0: Disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Start transfer
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Data transfer direction selection This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0).
    pub mod DTDIR {
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

            /// 0b0: From controller to card
            pub const ControllerToCard: u32 = 0b0;

            /// 0b1: From card to controller
            pub const CardToController: u32 = 0b1;
        }
    }

    /// Data transfer mode selection. This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0).
    pub mod DTMODE {
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

            /// 0b0: Bloack data transfer
            pub const BlockMode: u32 = 0b0;

            /// 0b1: Stream or SDIO multibyte data transfer
            pub const StreamMode: u32 = 0b1;
        }
    }

    /// Data block size. Define the data block length when the block data transfer mode is selected, block length = 2^(DBLOCKSIZE) bytes
    pub mod DBLOCKSIZE {
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

    /// Read wait start. If this bit is set, read wait operation starts.
    pub mod RWSTART {
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

            /// 0b0: Don't start read wait operation
            pub const Disabled: u32 = 0b0;

            /// 0b1: Read wait operation starts
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Read wait stop This bit is written by firmware and auto cleared by hardware when the DPSM moves from the READ_WAIT state to the WAIT_R or IDLE state.
    pub mod RWSTOP {
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

            /// 0b0: Read wait in progress if RWSTART is enabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Enable for read wait stop if RWSTART is enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Read wait mode. This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0).
    pub mod RWMOD {
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

            /// 0b0: Read wait control stopping using SDIO_D2
            pub const D2: u32 = 0b0;

            /// 0b1: Read wait control using SDIO_CK
            pub const Ck: u32 = 0b1;
        }
    }

    /// SD I/O interrupt enable functions This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0). If this bit is set, the DPSM enables the SD I/O card specific interrupt operation.
    pub mod SDIOEN {
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

            /// 0b0: SDIO operations disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: SDIO operations enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// DMA enable
    pub mod DMAEN {
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

            /// 0b0: Dma disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Dma enabled
            pub const Enabled: u32 = 0b1;
        }
    }
}

/// The SDMMC_DCNTR register loads the value from the data length register (see SDMMC_DLENR) when the DPSM moves from the Idle state to the Wait_R or Wait_S state. As data is transferred, the counter decrements the value until it reaches 0. The DPSM then moves to the Idle state and when there has been no error, the data status end flag (DATAEND) is set.
pub mod DCOUNT {

    /// Data count value When read, the number of remaining data bytes to be transferred is returned. Write has no effect.
    pub mod DATACOUNT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (25 bits: 0x1ffffff << 0)
        pub const mask: u32 = 0x1ffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// The SDMMC_STAR register is a read-only register. It contains two types of flag:Static flags (bits \[29,21,11:0\]): these bits remain asserted until they are cleared by writing to the SDMMC interrupt Clear register (see SDMMC_ICR)Dynamic flags (bits \[20:12\]): these bits change state depending on the state of the underlying logic (for example, FIFO full and empty flags are asserted and de-asserted as data while written to the FIFO)
pub mod STA {

    /// Command response received (CRC check failed). Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
    pub mod CCRCFAIL {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Command response received, crc check passed
            pub const NotFailed: u32 = 0b0;

            /// 0b1: Command response received, crc check failed
            pub const Failed: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Data block sent/received (CRC check failed). Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
    pub mod DCRCFAIL {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No Data block sent/received crc check fail
            pub const NotFailed: u32 = 0b0;

            /// 0b1: Data block sent/received crc failed
            pub const Failed: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Command response timeout. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR. The Command Timeout period has a fixed value of 64 SDMMC_CK clock periods.
    pub mod CTIMEOUT {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No Command timeout
            pub const NoTimeout: u32 = 0b0;

            /// 0b1: Command timeout
            pub const Timeout: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Data timeout. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
    pub mod DTIMEOUT {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No data timeout
            pub const NoTimeout: u32 = 0b0;

            /// 0b1: Data timeout
            pub const Timeout: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Transmit FIFO underrun error or IDMA read transfer error. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
    pub mod TXUNDERR {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No transmit FIFO underrun error
            pub const NoUnderrun: u32 = 0b0;

            /// 0b1: Transmit FIFO underrun error
            pub const Underrun: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Received FIFO overrun error or IDMA write transfer error. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
    pub mod RXOVERR {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No FIFO overrun error
            pub const NoOverrun: u32 = 0b0;

            /// 0b1: Receive FIFO overrun error
            pub const Overrun: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Command response received (CRC check passed, or no CRC). Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
    pub mod CMDREND {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Command not done
            pub const NotDone: u32 = 0b0;

            /// 0b1: Command response received (CRC check passed)
            pub const Done: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Command sent (no response required). Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
    pub mod CMDSENT {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Command not sent
            pub const NotSent: u32 = 0b0;

            /// 0b1: Command sent (no response required)
            pub const Sent: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Data transfer ended correctly. (data counter, DATACOUNT is zero and no errors occur). Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
    pub mod DATAEND {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Not done
            pub const NotDone: u32 = 0b0;

            /// 0b1: Data end (DCOUNT, is zero)
            pub const Done: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Data block sent/received. (CRC check passed) and DPSM moves to the READWAIT state. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
    pub mod DBCKEND {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Data block not sent/received (CRC check failed)
            pub const NotTransferred: u32 = 0b0;

            /// 0b1: Data block sent/received (CRC check passed)
            pub const Transferred: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Data transfer aborted by CMD12. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
    pub mod CMDACT {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Command transfer not in progress
            pub const NotInProgress: u32 = 0b0;

            /// 0b1: Command tranfer in progress
            pub const InProgress: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Data path state machine active, i.e. not in Idle state. This is a hardware status flag only, does not generate an interrupt.
    pub mod TXACT {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Data transmit is not in progress
            pub const NotInProgress: u32 = 0b0;

            /// 0b1: Data transmit in progress
            pub const InProgress: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Command path state machine active, i.e. not in Idle state. This is a hardware status flag only, does not generate an interrupt.
    pub mod RXACT {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Data receive not in progress
            pub const NotInProgress: u32 = 0b0;

            /// 0b1: Data receive in progress
            pub const InProgress: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Transmit FIFO half empty At least half the number of words can be written into the FIFO. This bit is cleared when the FIFO becomes half+1 full.
    pub mod TXFIFOHE {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Transmit FIFO not half empty
            pub const NotHalfEmpty: u32 = 0b0;

            /// 0b1: Transmit FIFO half empty. At least 8 words can be written into the FIFO
            pub const HalfEmpty: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Receive FIFO half full There are at least half the number of words in the FIFO. This bit is cleared when the FIFO becomes half+1 empty.
    pub mod RXFIFOHF {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Receive FIFO not half full
            pub const NotHalfFull: u32 = 0b0;

            /// 0b1: Receive FIFO half full. At least 8 words in the FIFO
            pub const HalfFull: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Transmit FIFO full This is a hardware status flag only, does not generate an interrupt. This bit is cleared when one FIFO location becomes empty.
    pub mod TXFIFOF {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Transmit FIFO not full
            pub const NotFull: u32 = 0b0;

            /// 0b1: Transmit FIFO full
            pub const Full: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Receive FIFO full This bit is cleared when one FIFO location becomes empty.
    pub mod RXFIFOF {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Transmit FIFO not full
            pub const NotFull: u32 = 0b0;

            /// 0b1: Receive FIFO full. When HW Flow Control is enabled, RXFIFOF signals becomes activated 2 words before the FIFO is full.
            pub const Full: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Transmit FIFO empty This bit is cleared when one FIFO location becomes full.
    pub mod TXFIFOE {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Transmit FIFO not empty
            pub const NotEmpty: u32 = 0b0;

            /// 0b1: Transmit FIFO empty. When HW Flow Control is enabled, TXFIFOE signals becomes activated when the FIFO contains 2 words.
            pub const Empty: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Receive FIFO empty This is a hardware status flag only, does not generate an interrupt. This bit is cleared when one FIFO location becomes full.
    pub mod RXFIFOE {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Receive FIFO not empty
            pub const NotEmpty: u32 = 0b0;

            /// 0b1: Receive FIFO empty
            pub const Empty: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Inverted value of SDMMC_D0 line (Busy), sampled at the end of a CMD response and a second time 2 SDMMC_CK cycles after the CMD response. This bit is reset to not busy when the SDMMCD0 line changes from busy to not busy. This bit does not signal busy due to data transfer. This is a hardware status flag only, it does not generate an interrupt.
    pub mod TXDAVL {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Data not available in transmit FIFO
            pub const NotAvailable: u32 = 0b0;

            /// 0b1: Data available in transmit FIFO
            pub const Available: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// end of SDMMC_D0 Busy following a CMD response detected. This indicates only end of busy following a CMD response. This bit does not signal busy due to data transfer. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
    pub mod RXDAVL {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: Data not available in receive FIFO
            pub const NotAvailable: u32 = 0b0;

            /// 0b1: Data available in receive FIFO
            pub const Available: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SDIO interrupt received. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
    pub mod SDIOIT {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: SDIO interrupt not receieved
            pub const NotReceived: u32 = 0b0;

            /// 0b1: SDIO interrupt received
            pub const Received: u32 = 0b1;
        }
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// The SDMMC_ICR register is a write-only register. Writing a bit with 1 clears the corresponding bit in the SDMMC_STAR status register.
pub mod ICR {

    /// CCRCFAIL flag clear bit Set by software to clear the CCRCFAIL flag.
    pub mod CCRCFAILC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Clear flag
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DCRCFAIL flag clear bit Set by software to clear the DCRCFAIL flag.
    pub mod DCRCFAILC {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::CCRCFAILC::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CTIMEOUT flag clear bit Set by software to clear the CTIMEOUT flag.
    pub mod CTIMEOUTC {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::CCRCFAILC::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DTIMEOUT flag clear bit Set by software to clear the DTIMEOUT flag.
    pub mod DTIMEOUTC {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::CCRCFAILC::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// TXUNDERR flag clear bit Set by software to clear TXUNDERR flag.
    pub mod TXUNDERRC {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::CCRCFAILC::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// RXOVERR flag clear bit Set by software to clear the RXOVERR flag.
    pub mod RXOVERRC {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::CCRCFAILC::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CMDREND flag clear bit Set by software to clear the CMDREND flag.
    pub mod CMDRENDC {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::CCRCFAILC::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CMDSENT flag clear bit Set by software to clear the CMDSENT flag.
    pub mod CMDSENTC {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::CCRCFAILC::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DATAEND flag clear bit Set by software to clear the DATAEND flag.
    pub mod DATAENDC {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::CCRCFAILC::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DBCKEND flag clear bit Set by software to clear the DBCKEND flag.
    pub mod DBCKENDC {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::CCRCFAILC::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SDIOIT flag clear bit Set by software to clear the SDIOIT flag.
    pub mod SDIOITC {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::CCRCFAILC::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// IDMA transfer error clear bit Set by software to clear the IDMATE flag.
    pub mod IDMATEC {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::CCRCFAILC::W;
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// The interrupt mask register determines which status flags generate an interrupt request by setting the corresponding bit to 1.
pub mod MASK {

    /// Command CRC fail interrupt enable Set and cleared by software to enable/disable interrupt caused by command CRC failure.
    pub mod CCRCFAILIE {
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

            /// 0b0: Interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Data CRC fail interrupt enable Set and cleared by software to enable/disable interrupt caused by data CRC failure.
    pub mod DCRCFAILIE {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CCRCFAILIE::RW;
    }

    /// Command timeout interrupt enable Set and cleared by software to enable/disable interrupt caused by command timeout.
    pub mod CTIMEOUTIE {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CCRCFAILIE::RW;
    }

    /// Data timeout interrupt enable Set and cleared by software to enable/disable interrupt caused by data timeout.
    pub mod DTIMEOUTIE {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CCRCFAILIE::RW;
    }

    /// Tx FIFO underrun error interrupt enable Set and cleared by software to enable/disable interrupt caused by Tx FIFO underrun error.
    pub mod TXUNDERRIE {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CCRCFAILIE::RW;
    }

    /// Rx FIFO overrun error interrupt enable Set and cleared by software to enable/disable interrupt caused by Rx FIFO overrun error.
    pub mod RXOVERRIE {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CCRCFAILIE::RW;
    }

    /// Command response received interrupt enable Set and cleared by software to enable/disable interrupt caused by receiving command response.
    pub mod CMDRENDIE {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CCRCFAILIE::RW;
    }

    /// Command sent interrupt enable Set and cleared by software to enable/disable interrupt caused by sending command.
    pub mod CMDSENTIE {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CCRCFAILIE::RW;
    }

    /// Data end interrupt enable Set and cleared by software to enable/disable interrupt caused by data end.
    pub mod DATAENDIE {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CCRCFAILIE::RW;
    }

    /// Data block end interrupt enable Set and cleared by software to enable/disable interrupt caused by data block end.
    pub mod DBCKENDIE {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CCRCFAILIE::RW;
    }

    /// Data transfer aborted interrupt enable Set and cleared by software to enable/disable interrupt caused by a data transfer being aborted.
    pub mod CMDACTIE {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CCRCFAILIE::RW;
    }

    /// Tx FIFO half empty interrupt enable Set and cleared by software to enable/disable interrupt caused by Tx FIFO half empty.
    pub mod TXFIFOHEIE {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CCRCFAILIE::RW;
    }

    /// Rx FIFO half full interrupt enable Set and cleared by software to enable/disable interrupt caused by Rx FIFO half full.
    pub mod RXFIFOHFIE {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CCRCFAILIE::RW;
    }

    /// Rx FIFO full interrupt enable Set and cleared by software to enable/disable interrupt caused by Rx FIFO full.
    pub mod RXFIFOFIE {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CCRCFAILIE::RW;
    }

    /// Tx FIFO empty interrupt enable Set and cleared by software to enable/disable interrupt caused by Tx FIFO empty.
    pub mod TXFIFOEIE {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CCRCFAILIE::RW;
    }

    /// BUSYD0END interrupt enable Set and cleared by software to enable/disable the interrupt generated when SDMMC_D0 signal changes from busy to NOT busy following a CMD response.
    pub mod RXDAVLIE {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CCRCFAILIE::RW;
    }

    /// SDIO mode interrupt received interrupt enable Set and cleared by software to enable/disable the interrupt generated when receiving the SDIO mode interrupt.
    pub mod SDIOITIE {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CCRCFAILIE::RW;
    }

    /// Data available in Tx FIFO interrupt enable. Set and cleared by software to enable/disable the interrupt generated by the presence of data available in Tx FIFO
    pub mod TXDAVLIE {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CCRCFAILIE::RW;
    }

    /// Rx FIFO empty interrupt enable. Set and cleared by software to enable/disable interrupt caused by Rx FIFO empty
    pub mod RXFIFOEIE {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CCRCFAILIE::RW;
    }

    /// Tx FIFO full interrupt enable. Set and cleared by software to enable/disable interrupt caused by Tx FIFO full
    pub mod TXFIFOFIE {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CCRCFAILIE::RW;
    }

    /// Data receive acting interrupt enable. Set and cleared by software to enable/disable interrupt caused by data being received (data receive acting)
    pub mod RXACTIE {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CCRCFAILIE::RW;
    }

    /// Data transmit acting interrupt enable. Set and cleared by software to enable/disable interrupt caused by data being transferred (data transmit acting)
    pub mod TXACTIE {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CCRCFAILIE::RW;
    }
}

/// The SDMMC_RESPCMDR register contains the command index field of the last command response received. If the command response transmission does not contain the command index field (long or OCR response), the RESPCMD field is unknown, although it must contain 111111b (the value of the reserved field from the response).
pub mod RESPCMD {

    /// Response command index Read-only bit field. Contains the command index of the last command response received.
    pub mod RESPCMD {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (6 bits: 0x3f << 0)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO.When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.
pub mod FIFO {

    /// Receive and transmit FIFO data This register can only be read or written by firmware when the DPSM is active (DPSMACT=1). The FIFO data occupies 16 entries of 32-bit words.
    pub mod FIFOData {
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

/// The SDIO_FIFOCNT register contains the remaining number of words to be written to or read from the FIFO. The FIFO counter loads the value from the data length register (see SDIO_DLEN) when the data transfer enable bit, DTEN, is set in the data control register (SDIO_DCTRL register) and the DPSM is at the Idle state. If the data length is not word-aligned (multiple of 4), the remaining 1 to 3 bytes are regarded as a word.
pub mod FIFOCNT {

    /// Remaining number of words to be written to or read from the FIFO
    pub mod FIFOCOUNT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (24 bits: 0xffffff << 0)
        pub const mask: u32 = 0xffffff << offset;
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
    /// SDIO power control register
    pub POWER: RWRegister<u32>,

    /// CLKCR register controls the SDIO_CK output clock.
    pub CLKCR: RWRegister<u32>,

    /// The SDMMC_ARGR register contains a 32-bit command argument, which is sent to a card as part of a command message.
    pub ARG: RWRegister<u32>,

    /// The SDMMC_CMDR register contains the command index and command type bits. The command index is sent to a card as part of a command message. The command type bits control the command path state machine (CPSM).
    pub CMD: RWRegister<u32>,

    /// The SDMMC_RESPCMDR register contains the command index field of the last command response received. If the command response transmission does not contain the command index field (long or OCR response), the RESPCMD field is unknown, although it must contain 111111b (the value of the reserved field from the response).
    pub RESPCMD: RORegister<u32>,

    /// The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response.
    pub RESP1: RORegister<u32>,

    /// The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response.
    pub RESP2: RORegister<u32>,

    /// The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response.
    pub RESP3: RORegister<u32>,

    /// The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response.
    pub RESP4: RORegister<u32>,

    /// The SDMMC_DTIMER register contains the data timeout period, in card bus clock periods. A counter loads the value from the SDMMC_DTIMER register, and starts decrementing when the data path state machine (DPSM) enters the Wait_R or Busy state. If the timer reaches 0 while the DPSM is in either of these states, the timeout status flag is set.
    pub DTIMER: RWRegister<u32>,

    /// The SDMMC_DLENR register contains the number of data bytes to be transferred. The value is loaded into the data counter when data transfer starts.
    pub DLEN: RWRegister<u32>,

    /// The SDMMC_DCTRL register control the data path state machine (DPSM).
    pub DCTRL: RWRegister<u32>,

    /// The SDMMC_DCNTR register loads the value from the data length register (see SDMMC_DLENR) when the DPSM moves from the Idle state to the Wait_R or Wait_S state. As data is transferred, the counter decrements the value until it reaches 0. The DPSM then moves to the Idle state and when there has been no error, the data status end flag (DATAEND) is set.
    pub DCOUNT: RORegister<u32>,

    /// The SDMMC_STAR register is a read-only register. It contains two types of flag:Static flags (bits \[29,21,11:0\]): these bits remain asserted until they are cleared by writing to the SDMMC interrupt Clear register (see SDMMC_ICR)Dynamic flags (bits \[20:12\]): these bits change state depending on the state of the underlying logic (for example, FIFO full and empty flags are asserted and de-asserted as data while written to the FIFO)
    pub STA: RORegister<u32>,

    /// The SDMMC_ICR register is a write-only register. Writing a bit with 1 clears the corresponding bit in the SDMMC_STAR status register.
    pub ICR: RWRegister<u32>,

    /// The interrupt mask register determines which status flags generate an interrupt request by setting the corresponding bit to 1.
    pub MASK: RWRegister<u32>,

    _reserved1: [u8; 8],

    /// The SDIO_FIFOCNT register contains the remaining number of words to be written to or read from the FIFO. The FIFO counter loads the value from the data length register (see SDIO_DLEN) when the data transfer enable bit, DTEN, is set in the data control register (SDIO_DCTRL register) and the DPSM is at the Idle state. If the data length is not word-aligned (multiple of 4), the remaining 1 to 3 bytes are regarded as a word.
    pub FIFOCNT: RORegister<u32>,

    _reserved2: [u8; 52],

    /// The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO.When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.
    pub FIFO: RWRegister<u32>,
}
pub struct ResetValues {
    pub POWER: u32,
    pub CLKCR: u32,
    pub ARG: u32,
    pub CMD: u32,
    pub RESPCMD: u32,
    pub RESP1: u32,
    pub RESP2: u32,
    pub RESP3: u32,
    pub RESP4: u32,
    pub DTIMER: u32,
    pub DLEN: u32,
    pub DCTRL: u32,
    pub DCOUNT: u32,
    pub STA: u32,
    pub ICR: u32,
    pub MASK: u32,
    pub FIFOCNT: u32,
    pub FIFO: u32,
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

/// Access functions for the SDIO peripheral instance
pub mod SDIO {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40012c00,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in SDIO
    pub const reset: ResetValues = ResetValues {
        POWER: 0x00000000,
        CLKCR: 0x00000000,
        ARG: 0x00000000,
        CMD: 0x00000000,
        RESP1: 0x00000000,
        RESP2: 0x00000000,
        RESP3: 0x00000000,
        RESP4: 0x00000000,
        DTIMER: 0x00000000,
        DLEN: 0x00000000,
        DCTRL: 0x00000000,
        DCOUNT: 0x00000000,
        STA: 0x00000000,
        ICR: 0x00000000,
        MASK: 0x00000000,
        RESPCMD: 0x00000000,
        FIFO: 0x00000000,
        FIFOCNT: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut SDIO_TAKEN: bool = false;

    /// Safe access to SDIO
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
            if SDIO_TAKEN {
                None
            } else {
                SDIO_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to SDIO
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if SDIO_TAKEN && inst.addr == INSTANCE.addr {
                SDIO_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal SDIO
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        SDIO_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to SDIO
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const SDIO: *const RegisterBlock = 0x40012c00 as *const _;
