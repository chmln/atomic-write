use nix::{
    fcntl::{open, OFlag},
    sys::stat::Mode,
    unistd::{close, fsync, unlink, write},
};
use std::path::Path;

mod error;
pub use self::error::Error;

pub fn atomic_write(path: impl AsRef<str>, contents: impl AsRef<[u8]>) -> Result<(), Error> {
    let original_path = path.as_ref();
    let mut tmp_path = original_path.to_string();
    tmp_path.push_str(".tmp~");

    let tmp_path = Path::new(&tmp_path);

    match unlink(tmp_path) {
        Ok(()) | Err(nix::Error::Sys(nix::errno::Errno::ENOENT)) => {}
        Err(e) => return Err(e.into()),
    }

    let fd = open(
        tmp_path,
        OFlag::O_RDWR | OFlag::O_CREAT | OFlag::O_TRUNC,
        Mode::S_IRUSR
            | Mode::S_IWUSR
            | Mode::S_IRGRP
            | Mode::S_IWGRP
            | Mode::S_IROTH
            | Mode::S_IWOTH,
    )?;

    write(fd, contents.as_ref())?;

    fsync(fd)?;
    close(fd)?;
    std::fs::rename(tmp_path, Path::new(&original_path))?;

    Ok(())
}
