use std::io::Error;
use memcache::MemcacheError;

#[derive(Debug)]
pub struct EitherError(Option<MemcacheError>, Option<Error>);

impl EitherError {
    pub fn from_memcache(e: MemcacheError) -> Self {
        EitherError(Some(e), None)
    }

    pub fn from_str(s: &str) -> Self {
        EitherError(None, Some(Error::new(std::io::ErrorKind::Other, s)))
    }

    pub fn to_string(&self) -> String {
        match &self.1 {
            Some(e) => e.to_string(),
            None => match &self.0 {
                Some(e) => e.to_string(),
                None => "Unknown error".to_string(),
            }
        }
    }
}