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
