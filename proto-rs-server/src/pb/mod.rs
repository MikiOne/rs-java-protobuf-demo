use prost::Message;
use crate::student::Student;

pub mod student;

impl From<Vec<u8>> for Student {
    fn from(value: Vec<u8>) -> Self {
        let mut bytes = value.as_slice();
        Self::decode(&bytes[..]).unwrap()
    }
}
impl<const N: usize> From<&[u8; N]> for Student {
    fn from(value: &[u8; N]) -> Self {
        Self::decode(&value[..]).unwrap()
    }
}
