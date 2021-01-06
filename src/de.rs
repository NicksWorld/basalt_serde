use std::io::Cursor;
use std::io::Read;
use std::marker::PhantomData;

use serde::{
    self,
    de::{IntoDeserializer, SeqAccess, Visitor},
    Deserialize,
};

use crate::error::{Error, Result};

pub struct Deserializer {
    pub(crate) reader: Cursor<Vec<u8>>,
}

impl Deserializer {
    fn read_byte(&mut self) -> Result<u8> {
        let mut buf = [0u8; 1];
        if self.reader.read_exact(&mut buf).is_ok() {
            Ok(buf[0])
        } else {
            Err(Error::Eof)
        }
    }
}

impl<'de, 'a> serde::Deserializer<'de> for &'a mut Deserializer {
    type Error = Error;

    #[inline]
    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Error::Message(
            "deserialize_any is not supported!".to_string(),
        ))
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.read_byte()? {
            1 => visitor.visit_bool(true),
            0 => visitor.visit_bool(false),
            _ => Err(Error::InvalidData),
        }
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_u8(self.read_byte()?)
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_i8(unsafe { std::mem::transmute::<u8, i8>(self.read_byte()?) })
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        let mut buf = [0u8; 2];
        if self.reader.read_exact(&mut buf).is_ok() {
            visitor.visit_u16(u16::from_be_bytes(buf))
        } else {
            Err(Error::Eof)
        }
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        let mut buf = [0u8; 2];
        if self.reader.read_exact(&mut buf).is_ok() {
            visitor.visit_i16(i16::from_be_bytes(buf))
        } else {
            Err(Error::Eof)
        }
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        let mut buf = [0u8; 4];
        if self.reader.read_exact(&mut buf).is_ok() {
            visitor.visit_u32(u32::from_be_bytes(buf))
        } else {
            Err(Error::Eof)
        }
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        let mut buf = [0u8; 4];
        if self.reader.read_exact(&mut buf).is_ok() {
            visitor.visit_i32(i32::from_be_bytes(buf))
        } else {
            Err(Error::Eof)
        }
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        let mut buf = [0u8; 8];
        if self.reader.read_exact(&mut buf).is_ok() {
            visitor.visit_u64(u64::from_be_bytes(buf))
        } else {
            Err(Error::Eof)
        }
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        let mut buf = [0u8; 8];
        if self.reader.read_exact(&mut buf).is_ok() {
            visitor.visit_i64(i64::from_be_bytes(buf))
        } else {
            Err(Error::Eof)
        }
    }

    fn deserialize_u128<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Error::Unimplemented)
    }

    fn deserialize_i128<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Error::Unimplemented)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        let mut buf = [0u8; 4];
        if self.reader.read_exact(&mut buf).is_ok() {
            visitor.visit_f32(f32::from_be_bytes(buf))
        } else {
            Err(Error::Eof)
        }
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        let mut buf = [0u8; 8];
        if self.reader.read_exact(&mut buf).is_ok() {
            visitor.visit_f64(f64::from_be_bytes(buf))
        } else {
            Err(Error::Eof)
        }
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_char(self.read_byte()? as char)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        let varint = self.deserialize_tuple(999, crate::types::varint::VarIntVisitor)?;
        let mut buf = vec![];
        self.reader
            .by_ref()
            .take(varint as u64)
            .read_to_end(&mut buf)
            .unwrap();

        visitor.visit_string(String::from_utf8_lossy(&buf).to_string())
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_option<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Error::Unimplemented)
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        println!("{:?}", name);
        println!("{:?}", variants);

        impl<'de, 'a> serde::de::EnumAccess<'de> for &'a mut Deserializer {
            type Error = Error;
            type Variant = Self;

            fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant)>
            where
                V: serde::de::DeserializeSeed<'de>,
            {
                use serde::Deserializer;

                let idx = self.deserialize_tuple(999, crate::types::varint::VarIntVisitor)? as u64;
                let val: Result<_> = seed.deserialize(idx.into_deserializer());
                Ok((val?, self))
            }
        }

        visitor.visit_enum(self)
        //Err(Error::Unimplemented)
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Error::Unimplemented)
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        struct Access<'a> {
            deserializer: &'a mut Deserializer,
            len: usize,
        }

        impl<'de, 'a, 'b: 'a> serde::de::SeqAccess<'de> for Access<'a> {
            type Error = Error;

            fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
            where
                T: serde::de::DeserializeSeed<'de>,
            {
                if self.len > 0 {
                    self.len -= 1;
                    let value =
                        serde::de::DeserializeSeed::deserialize(seed, &mut *self.deserializer)?;
                    Ok(Some(value))
                } else {
                    Ok(None)
                }
            }

            fn size_hint(&self) -> Option<usize> {
                Some(self.len)
            }
        }

        visitor.visit_seq(Access {
            deserializer: self,
            len,
        })
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_tuple(len, visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_tuple(fields.len(), visitor)
    }

    fn deserialize_str<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Error::Unimplemented)
    }

    fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Error::Unimplemented)
    }

    fn deserialize_identifier<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Error::Unimplemented)
    }

    fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Error::Unimplemented)
    }
}

pub fn from_bytes<'a, T>(data: &[u8]) -> Result<T>
where
    T: Deserialize<'a>,
{
    let mut deserializer = Deserializer {
        reader: ::std::io::Cursor::new(data.to_vec()),
    };
    T::deserialize(&mut deserializer)
}

impl<'de, 'a> serde::de::VariantAccess<'de> for &'a mut Deserializer {
    type Error = Error;

    fn unit_variant(self) -> Result<()> {
        Ok(())
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        serde::de::DeserializeSeed::deserialize(seed, self)
    }

    fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        serde::de::Deserializer::deserialize_tuple(self, len, visitor)
    }

    fn struct_variant<V>(self, fields: &'static [&'static str], visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        serde::de::Deserializer::deserialize_tuple(self, fields.len(), visitor)
    }
}
