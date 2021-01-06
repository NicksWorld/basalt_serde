use serde::{
    de::{SeqAccess, Visitor},
    Serializer,
};

pub type VarLong = i64;

pub fn deserialize<'de, D>(d: D) -> ::std::result::Result<VarLong, D::Error>
where
    D: serde::Deserializer<'de>,
{
    d.deserialize_tuple(10, VarLongVisitor)
}

struct VarLongVisitor;

impl<'de> Visitor<'de> for VarLongVisitor {
    type Value = VarLong;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a VarLong encoded as bytes")
    }

    fn visit_seq<A>(self, mut seq: A) -> ::std::result::Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut count = 0;
        let mut result = 0u64;
        let mut read;
        while {
            read = seq.next_element::<u8>()?.unwrap();
            let value = read & 0x7F;
            result |= (value as u64) << (7 * count);
            count += 1;
            (read & 0x80) != 0
        } {}
        Ok(unsafe { std::mem::transmute(result) })
    }
}

pub fn serialize<S>(varlong: &VarLong, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut value = unsafe { std::mem::transmute::<i64, u64>(*varlong) };
    let mut out = vec![];
    while {
        let mut temp = (value & 0x7F) as u8;
        value >>= 7;
        if value != 0 {
            temp |= 0x80;
        }
        out.push(temp);
        value != 0
    } {}

    serializer.serialize_bytes(&out)
}
