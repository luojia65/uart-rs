use volatile_register::{RO, WO};

/// UART Lite register block.
///
/// It consists of a control register, a status register, and a pair of
/// transmit/receive FIFOs.
#[repr(C)]
pub struct Registers {
    /// Receive data first-in first-out (FIFO) queue register.
    pub rx: RO<u32>,
    /// Transmit data first-in first-out (FIFO) queue register.
    pub tx: WO<u32>,
    /// Status register.
    pub stat: RO<u32>,
    /// Control register.
    pub ctrl: WO<u32>,
}
