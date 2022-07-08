use std::collections::HashMap;
use std::fmt::{Display, Formatter};

pub mod encode;
pub mod decode;
pub mod db;
pub mod ser;
pub use ser::serialize;

#[derive(Debug)]
pub enum Error {
    E(String)
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::E(e) => {
                e.fmt(f)
            }
        }
    }
}

impl std::error::Error for Error {}

impl serde::ser::Error for Error {
    fn custom<T>(msg: T) -> Self where T: Display {
        Self::E(msg.to_string())
    }
}

#[cfg(test)]
mod test {
    use std::borrow::Cow;
    use std::collections::HashMap;
    use std::time::Duration;
    use crate::ser::{Ser, serialize};

    #[test]
    fn test_ser_ref() {
        #[derive(serde::Serialize, serde::Deserialize, Debug)]
        pub struct A {
            pub name: String,
        }
        let a = A {
            name: "sss".to_string(),
        };
        let v = serialize(&a).unwrap();
        println!("{:?}", v);

        let mut m=HashMap::new();
        m.insert(1,2);
        let v = serialize(&m).unwrap();
        println!("{:?}", v);
    }

    #[test]
    fn test_ser() {
        #[derive(serde::Serialize, serde::Deserialize, Debug)]
        pub struct A {
            pub name: String,
            pub i32: i32,
            pub u32: u32,
            pub i64: i64,
            pub u64: u64,
        }
        let buf = rbmp::to_vec(&A {
            name: "s".to_string(),
            i32: i32::MAX,
            u32: u32::MAX,
            i64: i64::MAX,
            u64: u64::MAX,
        }).unwrap();
        let v: rbmp::Value = rbmp::read_value(&mut &buf[..]).unwrap();
        println!("{}", v);

        let v: A = rbmp::decode::from_slice(&buf).unwrap();
        println!("{:?}", v);
    }
}