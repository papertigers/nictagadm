#[derive(Debug)]
pub enum SdcConfigError {
    BadExitStatus(String, std::process::ExitStatus),
    IOError(std::io::Error),
    Utf8Error(std::str::Utf8Error),
}

impl From<std::io::Error> for SdcConfigError {
    fn from(err: std::io::Error) -> SdcConfigError {
        SdcConfigError::IOError(err)
    }
}

impl From<std::str::Utf8Error> for SdcConfigError {
    fn from(err: std::str::Utf8Error) -> SdcConfigError {
        SdcConfigError::Utf8Error(err)
    }
}
