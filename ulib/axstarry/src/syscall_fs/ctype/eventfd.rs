use axerrno::AxResult;
use axfs::api::{FileIO, FileIOType};

pub struct EventFd {
    value: u32,
    flags: i32,
}

impl EventFd {
    pub fn new(initval: u32, flags: i32) -> EventFd {
        EventFd {
            value: initval,
            flags,
        }
    }
}

impl FileIO for EventFd {
    fn read(&self, buf: &mut [u8]) -> AxResult<usize> {
        let len: usize = core::mem::size_of::<u32>();
        assert!(buf.len() == len);

        buf[0..len].copy_from_slice(&self.value.to_ne_bytes());
        Ok(len)
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
        let event_fd_val = 0u32;
        let len = event_fd.read(&mut event_fd_val.to_ne_bytes()).unwrap();

        assert_eq!(42, event_fd_val);
        assert_eq!(4, len);
    }
}
