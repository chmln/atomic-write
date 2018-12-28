use errno::{errno, Errno};
use libc;
use std::{ffi::CString, fmt};
use nix;

#[derive(Debug)]
pub enum Error {
    Libc(Errno),
    StrNul,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Error::Libc(errno) => format!("{}", errno),
                Error::StrNul => "An interior nul byte was found in the path".to_string(),
            }
        )
    }
}

fn result(data: i32) -> Result<i32, Error> {
    if data == -1 {
        return Err(Error::Libc(errno()));
    }
    return Ok(data);
}

pub fn c_str(s: String) -> Result<*const i8, Error> {
    CString::new(s)
        .map(|s| s.as_ptr())
        .map_err(|_| Error::StrNul)
}

pub fn atomic_write(path: impl Into<String>, contents: impl Into<Vec<u8>>) -> Result<(), Error> {
    let rename = |old_path, new_path| result(unsafe { libc::rename(old_path, new_path) });

    let original_path = path.into();
    let mut tmp_path = original_path.clone();
    tmp_path.push_str(".tmp");
    println!("{} {}", original_path, tmp_path);
    let (original_path, tmp_path) = (c_str(original_path)?, c_str(tmp_path)?);

    let fd = open(
        c_str(".".to_string())?,
               libc::O_CREAT | libc::O_TRUNC,

        libc::S_IRUSR
            | libc::S_IWUSR
            | libc::S_IRGRP
            | libc::S_IWGRP
            | libc::S_IROTH
            | libc::S_IWOTH,
    )?;
    println!("Opened!");


    let mut bytes = contents.into();
    let num_bytes = bytes.len();

    let bytes_written = write(
        fd,
        bytes.as_mut_ptr() as *mut core::ffi::c_void,
        num_bytes,
    )?;

    println!("Bytes: {}, Written: {}", num_bytes, bytes_written);

    close(fd)?;
    rename(tmp_path, original_path)?;

    Ok(())
}
