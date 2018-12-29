use nix::{
    fcntl::{open, OFlag},
    sys::stat::Mode,
    unistd::{close, fsync, unlink, write},
};
use std::{fmt, path::Path};

#[derive(Debug)]
pub struct Error {
    message: String,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl std::error::Error for Error {}

impl From<nix::Error> for Error {
    fn from(e: nix::Error) -> Self {
        Error {
            message: format!("{}", e),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error {
            message: format!("{}", e),
        }
    }
}


pub fn atomic_write(path: impl Into<String>, contents: impl Into<Vec<u8>>) -> Result<(), Error> {
    let original_path = path.into();
    let mut tmp_path = original_path.clone();
    tmp_path.push_str(".tmp");

    let tmp_path_s = Path::new(&tmp_path);
    let original_path_s = Path::new(&original_path);

    match unlink(tmp_path_s) {
        Ok(()) | Err(nix::Error::Sys(nix::errno::Errno::ENOENT)) => {}
        Err(e) => return Err(e.into()),
    }

    let fd = open(
        tmp_path_s,
        OFlag::O_RDWR | OFlag::O_CREAT | OFlag::O_TRUNC,
        Mode::S_IRUSR
            | Mode::S_IWUSR
            | Mode::S_IRGRP
            | Mode::S_IWGRP
            | Mode::S_IROTH
            | Mode::S_IWOTH,
    )?;

    write(fd, &contents.into())?;

    fsync(fd)?;
    close(fd)?;
    std::fs::rename(tmp_path_s, original_path_s)?;

    Ok(())
}
