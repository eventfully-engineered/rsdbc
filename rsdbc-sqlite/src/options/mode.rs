
// TODO: should this be file open modes?
// https://www.sqlite.org/c3ref/c_open_autoproxy.html

#[derive(Debug, Clone)]
pub enum Mode {
    RO,
    RW,
    RWC,
    MEMORY,

}

impl Mode {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            Mode::RO => "ro",
            Mode::RW => "rw",
            Mode::RWC => "rwc", // read write create
            Mode::MEMORY => "memory"
        }
    }
}

// TODO: what is the default?
impl Default for Mode {
    fn default() -> Self {
        Mode::RW
    }
}
