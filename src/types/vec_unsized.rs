use std::marker::PhantomData;
use serde::{
    de::{SeqAccess, Visitor},
    ser::SerializeSeq,
    Serializer,
    Serialize,
    Deserialize
};

pub fn deserialize<'de, D, T>(d: D) -> ::std::result::Result<Vec<T>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: Sized + Deserialize<'de>,
{
    d.deserialize_tuple(
        usize::MAX,
        VecVisitor {
            phantom: PhantomData::default(),
        },
    )
}

struct VecVisitor<T> {
    phantom: PhantomData<Vec<T>>,
}

impl<'de, T> Visitor<'de> for VecVisitor<T>
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
        let mut data = vec![];
        loop {
            match seq.next_element::<T>() {
                Ok(v) => data.push(v.unwrap()),
                Err(_) => break,
            }
        }
        Ok(data)
    }
}

pub fn serialize<T, S>(vec: &Vec<T>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: Serialize
{
    let mut sequence = serializer.serialize_seq(Some(vec.len()))?;
    for v in vec {
	sequence.serialize_element(v)?;
    }
    sequence.end()
}
