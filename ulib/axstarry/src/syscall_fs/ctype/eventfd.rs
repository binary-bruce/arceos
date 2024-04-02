use alloc::sync::Arc;
use axerrno::{AxError, AxResult};
use axfs::api::{FileIO, FileIOType};
use axsync::Mutex;
use axtask::yield_now;
use bitflags::{bitflags, Flags};

bitflags! {
    // https://sites.uclouvain.be/SystInfo/usr/include/sys/eventfd.h.html
    #[derive(Clone, Copy, Debug)]
    pub struct EventFdFlag: u32 {
        const EFD_SEMAPHORE = 1;
        const EFD_NONBLOCK = 2048;
    }
}

// https://man7.org/linux/man-pages/man2/eventfd2.2.html
pub struct EventFd {
    value: Arc<Mutex<u64>>,
    flags: u32,
}

impl EventFd {
    pub fn new(initval: u64, flags: u32) -> EventFd {
        EventFd {
            value: Arc::new(Mutex::new(initval)),
            flags,
        }
    }
}

impl FileIO for EventFd {
    fn read(&self, buf: &mut [u8]) -> AxResult<usize> {
        let len: usize = core::mem::size_of::<u64>();
        if buf.len() < len {
            return Err(AxError::InvalidInput);
        }

        // If EFD_SEMAPHORE was not specified and the eventfd counter has a nonzero value, then a read(2) returns 8 bytes containing that value,
        // and the counter's value is reset to zero.
        if self.flags & EventFdFlag::EFD_SEMAPHORE.bits() == 0 && *self.value.lock() != 0 {
            buf[0..len].copy_from_slice(&self.value.lock().to_ne_bytes());
            *self.value.lock() = 0;
            return Ok(len);
        }

        // If EFD_SEMAPHORE was specified and the eventfd counter has a nonzero value, then a read(2) returns 8 bytes containing the value,
        // and the counter's value is decremented by 1.
        if self.flags & EventFdFlag::EFD_SEMAPHORE.bits() != 0 && *self.value.lock() != 0 {
            buf[0..len].copy_from_slice(&self.value.lock().to_ne_bytes());
            let _ = self.value.lock().checked_add_signed(-1);
            return Ok(len);
        }

        // If the eventfd counter is zero at the time of the call to read(2),
        // then the call either blocks until the counter becomes nonzero (at which time, the read(2) proceeds as described above)
        // or fails with the error EAGAIN if the file descriptor has been made nonblocking.
        loop {
            if *self.value.lock() != 0 {
                buf[0..len].copy_from_slice(&self.value.lock().to_ne_bytes());
                return Ok(len);
            }

            if self.flags & EventFdFlag::EFD_NONBLOCK.bits() != 0 {
                yield_now()
            } else {
                return Err(AxError::WouldBlock);
            }
        }
    }

    fn write(&self, buf: &[u8]) -> AxResult<usize> {
        let len: usize = core::mem::size_of::<u64>();
        assert!(buf.len() == len);

        let val = u64::from_ne_bytes(buf[0..len].try_into().unwrap());
        let mut value_guard = self.value.lock();
        match value_guard.checked_add(val) {
            Some(new_value) => {
                *value_guard = new_value;
                Ok(len)
            }
            None => {
                panic!("overflow, to be handled in the future")
            }
        }
    }

    fn readable(&self) -> bool {
        true
    }

    fn writable(&self) -> bool {
        true
    }

    fn executable(&self) -> bool {
        false
    }

    fn get_type(&self) -> FileIOType {
        FileIOType::Other
    }
}

#[cfg(test)]
mod tests {
    use super::EventFd;
    use axfs::api::FileIO;

    #[test]
    fn test_read() {
        let event_fd = EventFd::new(42, 0);
        let event_fd_val = 0u64;
        let len = event_fd.read(&mut event_fd_val.to_ne_bytes()).unwrap();

        assert_eq!(42, event_fd_val);
        assert_eq!(4, len);
    }

    #[test]
    fn test_read_with_bad_input() {
        let event_fd = EventFd::new(42, 0);
        let event_fd_val = 0u32;
        let result = event_fd.read(&mut event_fd_val.to_ne_bytes());
        assert_eq!(Err(AxError::InvalidInput), result);
    }

    #[test]
    fn test_write() {
        let event_fd = EventFd::new(42, 0);
        let val = 12u64;
        event_fd.write(&val.to_ne_bytes()[0..core::mem::size_of::<u64>()]);

        let event_fd_val = 0u64;
        event_fd.read(&mut event_fd_val.to_ne_bytes()).unwrap();
        assert_eq!(54, event_fd_val);
    }
}
