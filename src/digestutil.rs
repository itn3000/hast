use super::error::ApplicationError;
use std::io::Read;

pub fn update_digest<D, R>(d: &mut D, in_f: &mut R) -> Result<(), ApplicationError> where D: digest::Update, R: Read {
    let mut buf: Vec<u8> = Vec::new();
    buf.resize(1024, 0u8);
    loop {
        let n = match in_f.read(&mut buf) {
            Ok(v) => Ok(v),
            Err(e) => Err(ApplicationError::Io(e))
        }?;
        d.update(&buf[0..n]);
        if n == 0 || n < 1024 {
            break;
        }
    }
    Ok(())
}
