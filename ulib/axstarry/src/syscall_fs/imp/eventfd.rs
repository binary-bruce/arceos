use axlog::error;
use crate::{SyscallError, SyscallResult};

pub fn syscall_eventfd(args: [usize; 6]) -> SyscallResult {
    let initval = args[0] as u32;
    let flags = args[1] as i32;
    error!("syscall_eventfd not implemented: initval={}, flags={}", initval, flags);
    Err(SyscallError::EMFILE)
}
