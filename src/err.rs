use zip::result::ZipError;

pub type TempResult<T> = Result<T, TempErr>;

#[derive(Debug)]
pub enum TempErr {
    Connection(reqwest::Error),
    Serialization(usiem::serde_json::Error),
    Io(std::io::Error),
    Zip(ZipError),
    Base(&'static str),
}

impl From<reqwest::Error> for TempErr {
    fn from(err: reqwest::Error) -> Self {
        Self::Connection(err)
    }
}
impl From<usiem::serde_json::Error> for TempErr {
    fn from(err: usiem::serde_json::Error) -> Self {
        Self::Serialization(err)
    }
}
/*
impl From<surf::Error> for TempErr {
    fn from(err: surf::Error) -> Self {
        Self::Connection(err)
    }
} */

impl From<ZipError> for TempErr {
    fn from(err: ZipError) -> Self {
        Self::Zip(err)
    }
}
impl From<std::io::Error> for TempErr {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}
