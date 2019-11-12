use crate::{TypeParser, Error, TypeParseResult, utils};
use std::str::FromStr;

macro_rules! numeric_parser {
    ($parser:ident, $numtype:ty) => {
        pub struct $parser;

        impl TypeParser for $parser {
            type Type = $numtype;
            type Context = ();

            fn parse(&self, _ctx: &Self::Context, src: &str) -> TypeParseResult<Self::Type> {
                let s = match utils::take_until_whitespace(src) {
                    Some(s) => s,
                    None => return Err(Error::UnexpectedToken(0)),
                };

                let val = <$numtype>::from_str(s)?;

                Ok((val, s.len()))
            }
        }
    };
}

numeric_parser!(U8Parser,  u8);
numeric_parser!(U16Parser, u16);
numeric_parser!(U32Parser, u32);
numeric_parser!(U64Parser, u64);
numeric_parser!(I8Parser,  i8);
numeric_parser!(I16Parser, i16);
numeric_parser!(I32Parser, i32);
numeric_parser!(I64Parser, i64);
