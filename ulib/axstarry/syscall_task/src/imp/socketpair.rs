use crate::make_pipe;

use axlog::info;
use axprocess::current_process;
use syscall_utils::{SyscallError, SyscallResult};

pub fn syscall_socketpair(
    _domain: usize,
    _sock_type: usize,
    _protocol: usize,
    sv: *mut [u32; 2],
) -> SyscallResult {
    let flags = 0;
    let fd = sv as *mut u32;
    axlog::info!("Into syscall_pipe2. fd: {} flags: {}", fd as usize, flags);
    let process = current_process();
    if process.manual_alloc_for_lazy((fd as usize).into()).is_err() {
        return Err(SyscallError::EINVAL);
    }
    let non_block = (flags & 0x800) != 0;
    let (read, write) = make_pipe(non_block);
    let mut fd_table = process.fd_manager.fd_table.lock();
    let fd_num = if let Ok(fd) = process.alloc_fd(&mut fd_table) {
        fd
    } else {
        return Err(SyscallError::EPERM);
    };
    fd_table[fd_num] = Some(read);
    let fd_num2 = if let Ok(fd) = process.alloc_fd(&mut fd_table) {
        fd
    } else {
        return Err(SyscallError::EPERM);
    };
    fd_table[fd_num2] = Some(write);
    info!("read end: {} write: end: {}", fd_num, fd_num2);
    unsafe {
        core::ptr::write(fd, fd_num as u32);
        core::ptr::write(fd.offset(1), fd_num2 as u32);
    }
    Ok(0)
}
