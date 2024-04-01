use alloc::sync::Arc;
use axerrno::AxResult;
use axfs::api::{FileIO, FileIOType};
use axsync::Mutex;

pub struct EventFd {
    value: Arc<Mutex<u64>>,
    flags: i32,
}

impl EventFd {
    pub fn new(initval: u64, flags: i32) -> EventFd {
        EventFd {
            value: Arc::new(Mutex::new(initval)),
            flags,
        }
    }
}

impl FileIO for EventFd {
    fn read(&self, buf: &mut [u8]) -> AxResult<usize> {
        let len: usize = core::mem::size_of::<u64>();
        assert!(buf.len() == len);

        buf[0..len].copy_from_slice(&self.value.lock().to_ne_bytes());
        Ok(len)
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
    fn test_write() {
        let event_fd = EventFd::new(42, 0);
        let val = 12u64;
        event_fd.write(&val.to_ne_bytes()[0..core::mem::size_of::<u64>()]);

        let event_fd_val = 0u64;
        event_fd.read(&mut event_fd_val.to_ne_bytes()).unwrap();
        assert_eq!(54, event_fd_val);
    }
}
