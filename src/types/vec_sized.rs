use serde::{
    de::{SeqAccess, Visitor},
    ser::SerializeSeq,
    Deserialize, Serialize, Serializer,
};
use std::marker::PhantomData;

use crate::error::Error;

pub fn deserialize<'de, D, T>(d: D) -> ::std::result::Result<Vec<T>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: Sized + Deserialize<'de>,
{
    d.deserialize_tuple(
        999,
        VecVisitorSized::<T> {
            phantom: PhantomData::default(),
        },
    )
}

struct VecVisitorSized<T> {
    phantom: PhantomData<Vec<T>>,
}

impl<'de, T> Visitor<'de> for VecVisitorSized<T>
where
    T: Deserialize<'de>,
{
    type Value = Vec<T>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a VarInt followed by data")
    }

    fn visit_seq<A>(self, mut seq: A) -> ::std::result::Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
        T: Deserialize<'de>,
    {
        let mut count = 0;
        let mut result = 0u32;
        let mut read;
        while {
            read = seq.next_element::<u8>()?.unwrap();
            let value = read & 0x7F;
            result |= (value as u32) << (7 * count);
            count += 1;
            if count > 5 {
                panic!("Error not implemented")
            }
            (read & 0x80) != 0
        } {}

        let length = unsafe { std::mem::transmute::<u32, i32>(result) };

        let mut data = vec![];
        for _ in 0..length {
            data.push(seq.next_element::<T>()?.unwrap())
        }
        Ok(data)
    }
}

pub fn serialize<T, S>(vec: &Vec<T>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: Serialize,
{
    let mut sequence = serializer.serialize_seq(Some(vec.len() + 1))?;
    sequence.serialize_element(&crate::types::varint::to_bytes(vec.len() as i32))?;
    for v in vec {
        sequence.serialize_element(v)?;
    }
    sequence.end()
}
