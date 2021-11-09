//! Hello Driver
//! Commands
//!     0 -> SUCCESS
//!     1 -> print Hello
//!

use core::cell::Cell;

use kernel::process::{Error, ProcessId};
use kernel::syscall::{CommandReturn, SyscallDriver};
use kernel::{debug, ErrorCode};

pub const DRIVER_NUM: usize = 0xa0000;

pub struct Hello {
    // interior mutability
    value: Cell<usize>,
}

impl Hello {
    pub const fn new() -> Hello {
        Hello {
            value: Cell::new(0),
        }
    }
}

impl SyscallDriver for Hello {
    fn command(
        &self,
        command_num: usize,
        _r2: usize,
        _r3: usize,
        _process_id: ProcessId,
    ) -> CommandReturn {
        match command_num {
            0 => CommandReturn::success(),
            1 => {
                debug!("Hello");
                CommandReturn::success()
            }
            2 => {
                self.value.set(self.value.get() + 1);
                CommandReturn::success()
            }
            _ => CommandReturn::failure(ErrorCode::NOSUPPORT),
        }
    }

    fn allocate_grant(&self, _process_id: ProcessId) -> Result<(), Error> {
        Ok(())
    }
}
