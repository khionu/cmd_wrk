use std::any::TypeId;
use std::error::Error as StdError;
use std::marker::PhantomData;
use std::num::ParseIntError;

use typemap::{ShareMap, Key};

pub mod type_parsers;
pub mod utils;

pub type TypeParseResult<T> = Result<(T, usize), Error>;

pub trait TypeParser {
    type Type;
    type Context;

    fn parse(&self, ctx: &Self::Context, src: &str) -> TypeParseResult<Self::Type>;
}

pub struct TypeParserKey<P: TypeParser + 'static> {
    __phantom: PhantomData<P>,
}

impl<P> Key for TypeParserKey<P> where
    P: TypeParser + 'static,
{
    type Value = P;
}

pub enum Error {
    InvalidFormat(String),
    UnexpectedToken(usize),
    Other(Box<dyn StdError>),
}

pub struct ParserStorage(ShareMap);

pub struct Parser {
    storage: ParserStorage,
}

impl ParserStorage {
    pub fn empty() -> Self {
        ParserStorage(ShareMap::custom())
    }

    pub fn new() -> Self {
        let mut map = ShareMap::custom();

        macro_rules! insert {
            ($parser:ident) => {
                map.insert::<TypeParserKey<$parser>>($parser);
            };
        }

        #[cfg(feature = "bundle-primitive")]
        {
            use type_parsers::primitives::*;

            insert!(U8Parser);
            insert!(U16Parser);
            insert!(U32Parser);
            insert!(U64Parser);
            insert!(I8Parser);
            insert!(I16Parser);
            insert!(I32Parser);
            insert!(I64Parser);
        }

        #[cfg(feature = "bundle-std")]
        {
            use type_parsers::std::*;

            insert!(IpParser);
            insert!(DurationParser);
            insert!(StringParser);
        }

        ParserStorage(map)
    }
}

impl<T: StdError + 'static + Sized> From<T> for Error {
    fn from(err: T) -> Self {
        let ty = TypeId::of::<T>();

        match ty {
            t if t == TypeId::of::<ParseIntError>() => {
                Error::InvalidFormat(err.to_string())
            }
            _ => Error::Other(Box::new(err))
        }
    }
}

impl Parser {
    pub fn from_clap_app(app: clap::App) -> Parser {

    }
}
