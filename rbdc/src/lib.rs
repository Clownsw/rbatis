use std::collections::HashMap;
use indexmap::IndexMap;

pub mod encode;
pub mod decode;
pub mod db;


#[cfg(test)]
mod test{
    #[test]
    fn test_ser(){
        let buf = rmp_serde::to_vec(&(42, "the Answer")).unwrap();

        assert_eq!(
            vec![0x92, 0x2a, 0xaa, 0x74, 0x68, 0x65, 0x20, 0x41, 0x6e, 0x73, 0x77, 0x65, 0x72],
            buf
        );

        let (a,b): (i32, &str)=rmp_serde::from_slice(&buf).unwrap();
        println!("{}{}",a,b);
        // assert_eq!((42, "the Answer"), rmp_serde::from_slice(&buf).unwrap());
    }
}