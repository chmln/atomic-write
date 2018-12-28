mod lib;
use self::lib::{atomic_write, Error};
pub fn main() -> Result<(), Error> {
    atomic_write("/tmp/test", "yo this works")
}
