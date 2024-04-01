use alloc::sync::Arc;
use axlog::error;
use axprocess::current_process;

use crate::syscall_fs::ctype::eventfd::EventFd;
use crate::{SyscallError, SyscallResult};

pub fn syscall_eventfd(args: [usize; 6]) -> SyscallResult {
    let initval = args[0] as u64;
    let flags = args[1] as i32;
    error!(
        "[syscall_eventfd] is called: initval={}, flags={}",
        initval, flags
    );

    let process = current_process();
    let mut fd_table = process.fd_manager.fd_table.lock();
    let fd_num = if let Ok(fd) = process.alloc_fd(&mut fd_table) {
        fd
    } else {
        return Err(SyscallError::EPERM);
    };

    error!("[syscall_eventfd] allocated available fd: {}", fd_num);
    fd_table[fd_num] = Some(Arc::new(EventFd::new(initval, flags)));

    Ok(fd_num as isize)
}
