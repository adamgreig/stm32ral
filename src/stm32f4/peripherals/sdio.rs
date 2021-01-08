#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Secure digital input/output interface
//!
//! Used by: stm32f401, stm32f405, stm32f407, stm32f411, stm32f412, stm32f413, stm32f427, stm32f429, stm32f469

use crate::{RORegister, RWRegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// power control register
pub mod POWER {

    /// PWRCTRL
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

/// SDI clock control register
pub mod CLKCR {

    /// HW Flow Control enable
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

    /// SDIO_CK dephasing selection bit
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

    /// Wide bus mode enable bit
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

    /// Power saving configuration bit
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

            /// 0b1: SDIO_CK is only enabled when the bus is active
            pub const Disabled: u32 = 0b1;

            /// 0b0: SDIO_CK clock is always enabled
            pub const Enabled: u32 = 0b0;
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

    /// Clock divide factor
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
}

/// argument register
pub mod ARG {

    /// Command argument
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

/// command register
pub mod CMD {

    /// CE-ATA command
    pub mod CE_ATACMD {
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

            /// 0b0: CE-ATA command disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: CE-ATA command enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// not Interrupt Enable
    pub mod nIEN {
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

            /// 0b0: Interrupts to the CE-ATA not disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Interrupt to the CE-ATA are disabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Enable CMD completion
    pub mod ENCMDcompl {
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

            /// 0b0: Command complete signal disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Command complete signal enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// SD I/O suspend command
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

    /// Command path state machine (CPSM) Enable bit
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

    /// CPSM Waits for ends of data transfer (CmdPend internal signal).
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

    /// CPSM waits for interrupt request
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

    /// Wait for response bits
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

    /// Command index
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
}

/// command response register
pub mod RESPCMD {

    /// Response command index
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

/// response 1..4 register
pub mod RESP1 {

    /// Card Status
    pub mod CARDSTATUS1 {
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

/// response 1..4 register
pub mod RESP2 {

    /// Card Status
    pub mod CARDSTATUS2 {
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

/// response 1..4 register
pub mod RESP3 {

    /// Card Status
    pub mod CARDSTATUS3 {
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

/// response 1..4 register
pub mod RESP4 {

    /// Card Status
    pub mod CARDSTATUS4 {
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

/// data timer register
pub mod DTIMER {

    /// Data timeout period
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

/// data length register
pub mod DLEN {

    /// Data length value
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

/// data control register
pub mod DCTRL {

    /// SD I/O enable functions
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

    /// Read wait mode
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

    /// Read wait stop
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

    /// Read wait start
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

    /// Data block size
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

    /// DMA enable bit
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

    /// Data transfer mode selection 1: Stream or SDIO multibyte data transfer.
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

    /// Data transfer direction selection
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

    /// DTEN
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
}

/// data counter register
pub mod DCOUNT {

    /// Data count value
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

/// status register
pub mod STA {

    /// CE-ATA command completion signal received for CMD61
    pub mod CEATAEND {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Completion signal not received
            pub const NotReceived: u32 = 0b0;

            /// 0b1: CE-ATA command completion signal received for CMD61
            pub const Received: u32 = 0b1;
        }
    }

    /// SDIO interrupt received
    pub mod SDIOIT {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: SDIO interrupt not receieved
            pub const NotReceived: u32 = 0b0;

            /// 0b1: SDIO interrupt received
            pub const Received: u32 = 0b1;
        }
    }

    /// Data available in receive FIFO
    pub mod RXDAVL {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Data not available in receive FIFO
            pub const NotAvailable: u32 = 0b0;

            /// 0b1: Data available in receive FIFO
            pub const Available: u32 = 0b1;
        }
    }

    /// Data available in transmit FIFO
    pub mod TXDAVL {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Data not available in transmit FIFO
            pub const NotAvailable: u32 = 0b0;

            /// 0b1: Data available in transmit FIFO
            pub const Available: u32 = 0b1;
        }
    }

    /// Receive FIFO empty
    pub mod RXFIFOE {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Receive FIFO not empty
            pub const NotEmpty: u32 = 0b0;

            /// 0b1: Receive FIFO empty
            pub const Empty: u32 = 0b1;
        }
    }

    /// Transmit FIFO empty
    pub mod TXFIFOE {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Transmit FIFO not empty
            pub const NotEmpty: u32 = 0b0;

            /// 0b1: Transmit FIFO empty. When HW Flow Control is enabled, TXFIFOE signals becomes activated when the FIFO contains 2 words.
            pub const Empty: u32 = 0b1;
        }
    }

    /// Receive FIFO full
    pub mod RXFIFOF {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Transmit FIFO not full
            pub const NotFull: u32 = 0b0;

            /// 0b1: Receive FIFO full. When HW Flow Control is enabled, RXFIFOF signals becomes activated 2 words before the FIFO is full.
            pub const Full: u32 = 0b1;
        }
    }

    /// Transmit FIFO full
    pub mod TXFIFOF {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Transmit FIFO not full
            pub const NotFull: u32 = 0b0;

            /// 0b1: Transmit FIFO full
            pub const Full: u32 = 0b1;
        }
    }

    /// Receive FIFO half full: there are at least 8 words in the FIFO
    pub mod RXFIFOHF {
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

            /// 0b0: Receive FIFO not half full
            pub const NotHalfFull: u32 = 0b0;

            /// 0b1: Receive FIFO half full. At least 8 words in the FIFO
            pub const HalfFull: u32 = 0b1;
        }
    }

    /// Transmit FIFO half empty: at least 8 words can be written into the FIFO
    pub mod TXFIFOHE {
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

            /// 0b0: Transmit FIFO not half empty
            pub const NotHalfEmpty: u32 = 0b0;

            /// 0b1: Transmit FIFO half empty. At least 8 words can be written into the FIFO
            pub const HalfEmpty: u32 = 0b1;
        }
    }

    /// Data receive in progress
    pub mod RXACT {
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

            /// 0b0: Data receive not in progress
            pub const NotInProgress: u32 = 0b0;

            /// 0b1: Data receive in progress
            pub const InProgress: u32 = 0b1;
        }
    }

    /// Data transmit in progress
    pub mod TXACT {
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

            /// 0b0: Data transmit is not in progress
            pub const NotInProgress: u32 = 0b0;

            /// 0b1: Data transmit in progress
            pub const InProgress: u32 = 0b1;
        }
    }

    /// Command transfer in progress
    pub mod CMDACT {
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

            /// 0b0: Command transfer not in progress
            pub const NotInProgress: u32 = 0b0;

            /// 0b1: Command tranfer in progress
            pub const InProgress: u32 = 0b1;
        }
    }

    /// Data block sent/received (CRC check passed)
    pub mod DBCKEND {
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

            /// 0b0: Data block not sent/received (CRC check failed)
            pub const NotTransferred: u32 = 0b0;

            /// 0b1: Data block sent/received (CRC check passed)
            pub const Transferred: u32 = 0b1;
        }
    }

    /// Start bit not detected on all data signals in wide bus mode
    pub mod STBITERR {
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

            /// 0b0: No start bit detected error
            pub const Detected: u32 = 0b0;

            /// 0b1: Start bit not detected error
            pub const NotDetected: u32 = 0b1;
        }
    }

    /// Data end (data counter, SDIDCOUNT, is zero)
    pub mod DATAEND {
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

            /// 0b1: Data end (DCOUNT, is zero)
            pub const Done: u32 = 0b1;

            /// 0b0: Not done
            pub const NotDone: u32 = 0b0;
        }
    }

    /// Command sent (no response required)
    pub mod CMDSENT {
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

            /// 0b0: Command not sent
            pub const NotSent: u32 = 0b0;

            /// 0b1: Command sent (no response required)
            pub const Sent: u32 = 0b1;
        }
    }

    /// Command response received (CRC check passed)
    pub mod CMDREND {
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

            /// 0b0: Command not done
            pub const NotDone: u32 = 0b0;

            /// 0b1: Command response received (CRC check passed)
            pub const Done: u32 = 0b1;
        }
    }

    /// Received FIFO overrun error
    pub mod RXOVERR {
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

            /// 0b0: No FIFO overrun error
            pub const NoOverrun: u32 = 0b0;

            /// 0b1: Receive FIFO overrun error
            pub const Overrun: u32 = 0b1;
        }
    }

    /// Transmit FIFO underrun error
    pub mod TXUNDERR {
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

            /// 0b0: No transmit FIFO underrun error
            pub const NoUnderrun: u32 = 0b0;

            /// 0b1: Transmit FIFO underrun error
            pub const Underrun: u32 = 0b1;
        }
    }

    /// Data timeout
    pub mod DTIMEOUT {
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

            /// 0b0: No data timeout
            pub const NoTimeout: u32 = 0b0;

            /// 0b1: Data timeout
            pub const Timeout: u32 = 0b1;
        }
    }

    /// Command response timeout
    pub mod CTIMEOUT {
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

            /// 0b0: No Command timeout
            pub const NoTimeout: u32 = 0b0;

            /// 0b1: Command timeout
            pub const Timeout: u32 = 0b1;
        }
    }

    /// Data block sent/received (CRC check failed)
    pub mod DCRCFAIL {
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

            /// 0b0: No Data block sent/received crc check fail
            pub const NotFailed: u32 = 0b0;

            /// 0b1: Data block sent/received crc failed
            pub const Failed: u32 = 0b1;
        }
    }

    /// Command response received (CRC check failed)
    pub mod CCRCFAIL {
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

            /// 0b0: Command response received, crc check passed
            pub const NotFailed: u32 = 0b0;

            /// 0b1: Command response received, crc check failed
            pub const Failed: u32 = 0b1;
        }
    }
}

/// interrupt clear register
pub mod ICR {

    /// CEATAEND flag clear bit
    pub mod CEATAENDC {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
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

    /// SDIOIT flag clear bit
    pub mod SDIOITC {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::CEATAENDC::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DBCKEND flag clear bit
    pub mod DBCKENDC {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::CEATAENDC::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// STBITERR flag clear bit
    pub mod STBITERRC {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::CEATAENDC::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DATAEND flag clear bit
    pub mod DATAENDC {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::CEATAENDC::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CMDSENT flag clear bit
    pub mod CMDSENTC {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::CEATAENDC::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CMDREND flag clear bit
    pub mod CMDRENDC {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::CEATAENDC::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// RXOVERR flag clear bit
    pub mod RXOVERRC {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::CEATAENDC::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// TXUNDERR flag clear bit
    pub mod TXUNDERRC {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::CEATAENDC::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DTIMEOUT flag clear bit
    pub mod DTIMEOUTC {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::CEATAENDC::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CTIMEOUT flag clear bit
    pub mod CTIMEOUTC {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::CEATAENDC::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DCRCFAIL flag clear bit
    pub mod DCRCFAILC {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::CEATAENDC::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CCRCFAIL flag clear bit
    pub mod CCRCFAILC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::CEATAENDC::W;
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// mask register
pub mod MASK {

    /// CE-ATA command completion signal received interrupt enable
    pub mod CEATAENDIE {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
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

    /// SDIO mode interrupt received interrupt enable
    pub mod SDIOITIE {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CEATAENDIE::RW;
    }

    /// Data available in Rx FIFO interrupt enable
    pub mod RXDAVLIE {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CEATAENDIE::RW;
    }

    /// Data available in Tx FIFO interrupt enable
    pub mod TXDAVLIE {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CEATAENDIE::RW;
    }

    /// Rx FIFO empty interrupt enable
    pub mod RXFIFOEIE {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CEATAENDIE::RW;
    }

    /// Tx FIFO empty interrupt enable
    pub mod TXFIFOEIE {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CEATAENDIE::RW;
    }

    /// Rx FIFO full interrupt enable
    pub mod RXFIFOFIE {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CEATAENDIE::RW;
    }

    /// Tx FIFO full interrupt enable
    pub mod TXFIFOFIE {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CEATAENDIE::RW;
    }

    /// Rx FIFO half full interrupt enable
    pub mod RXFIFOHFIE {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CEATAENDIE::RW;
    }

    /// Tx FIFO half empty interrupt enable
    pub mod TXFIFOHEIE {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CEATAENDIE::RW;
    }

    /// Data receive acting interrupt enable
    pub mod RXACTIE {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CEATAENDIE::RW;
    }

    /// Data transmit acting interrupt enable
    pub mod TXACTIE {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CEATAENDIE::RW;
    }

    /// Command acting interrupt enable
    pub mod CMDACTIE {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CEATAENDIE::RW;
    }

    /// Data block end interrupt enable
    pub mod DBCKENDIE {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CEATAENDIE::RW;
    }

    /// Start bit error interrupt enable
    pub mod STBITERRIE {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CEATAENDIE::RW;
    }

    /// Data end interrupt enable
    pub mod DATAENDIE {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CEATAENDIE::RW;
    }

    /// Command sent interrupt enable
    pub mod CMDSENTIE {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CEATAENDIE::RW;
    }

    /// Command response received interrupt enable
    pub mod CMDRENDIE {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CEATAENDIE::RW;
    }

    /// Rx FIFO overrun error interrupt enable
    pub mod RXOVERRIE {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CEATAENDIE::RW;
    }

    /// Tx FIFO underrun error interrupt enable
    pub mod TXUNDERRIE {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CEATAENDIE::RW;
    }

    /// Data timeout interrupt enable
    pub mod DTIMEOUTIE {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CEATAENDIE::RW;
    }

    /// Command timeout interrupt enable
    pub mod CTIMEOUTIE {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CEATAENDIE::RW;
    }

    /// Data CRC fail interrupt enable
    pub mod DCRCFAILIE {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CEATAENDIE::RW;
    }

    /// Command CRC fail interrupt enable
    pub mod CCRCFAILIE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CEATAENDIE::RW;
    }
}

/// FIFO counter register
pub mod FIFOCNT {

    /// Remaining number of words to be written to or read from the FIFO.
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

/// data FIFO register
pub mod FIFO {

    /// Receive and transmit FIFO data
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
#[repr(C)]
pub struct RegisterBlock {
    /// power control register
    pub POWER: RWRegister<u32>,

    /// SDI clock control register
    pub CLKCR: RWRegister<u32>,

    /// argument register
    pub ARG: RWRegister<u32>,

    /// command register
    pub CMD: RWRegister<u32>,

    /// command response register
    pub RESPCMD: RORegister<u32>,

    /// response 1..4 register
    pub RESP1: RORegister<u32>,

    /// response 1..4 register
    pub RESP2: RORegister<u32>,

    /// response 1..4 register
    pub RESP3: RORegister<u32>,

    /// response 1..4 register
    pub RESP4: RORegister<u32>,

    /// data timer register
    pub DTIMER: RWRegister<u32>,

    /// data length register
    pub DLEN: RWRegister<u32>,

    /// data control register
    pub DCTRL: RWRegister<u32>,

    /// data counter register
    pub DCOUNT: RORegister<u32>,

    /// status register
    pub STA: RORegister<u32>,

    /// interrupt clear register
    pub ICR: RWRegister<u32>,

    /// mask register
    pub MASK: RWRegister<u32>,

    _reserved1: [u32; 2],

    /// FIFO counter register
    pub FIFOCNT: RORegister<u32>,

    _reserved2: [u32; 13],

    /// data FIFO register
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
