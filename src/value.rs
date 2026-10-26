//! JVM values with raw floating-point bits and UTF-16 code units preserved.
use derive_deftly::{Deftly, define_derive_deftly};
define_derive_deftly! {
    ValueTypeNames:
    impl $ttype {
        pub const fn name(self) -> &'static str { match self { $(Self::$vname => stringify!($vname),) } }
        pub fn from_name(name: &str) -> Option<Self> { match name { $(stringify!($vname) => Some(Self::$vname),) _ => None } }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Deftly)]
#[derive_deftly(ValueTypeNames)]
pub enum ValueType {
    Boolean,
    String,
    Byte,
    Short,
    Int,
    Long,
    Float,
    Double,
    List,
    Char,
    NodeRef,
    Unknown,
}
/// A JVM string, including isolated surrogate code units.
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct JavaString(pub Vec<u16>);
impl JavaString {
    pub fn from_units(units: Vec<u16>) -> Self {
        Self(units)
    }
    pub fn units(&self) -> &[u16] {
        &self.0
    }
    pub fn to_utf8(&self) -> std::result::Result<String, std::string::FromUtf16Error> {
        String::from_utf16(&self.0)
    }
    pub fn substring(&self, start: i32, end: i32) -> Option<Self> {
        let (start, end) = (usize::try_from(start).ok()?, usize::try_from(end).ok()?);
        Some(Self(self.0.get(start..end)?.to_vec()))
    }
}
impl From<&str> for JavaString {
    fn from(value: &str) -> Self {
        Self(value.encode_utf16().collect())
    }
}
impl From<String> for JavaString {
    fn from(value: String) -> Self {
        Self::from(value.as_str())
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Scalar {
    Boolean(bool),
    String(JavaString),
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(u32),
    Double(u64),
    Char(u16),
    NodeRef(String),
    List(Vec<Scalar>),
}
impl Scalar {
    pub fn value_type(&self) -> ValueType {
        match self {
            Self::Boolean(_) => ValueType::Boolean,
            Self::String(_) => ValueType::String,
            Self::Byte(_) => ValueType::Byte,
            Self::Short(_) => ValueType::Short,
            Self::Int(_) => ValueType::Int,
            Self::Long(_) => ValueType::Long,
            Self::Float(_) => ValueType::Float,
            Self::Double(_) => ValueType::Double,
            Self::Char(_) => ValueType::Char,
            Self::NodeRef(_) => ValueType::NodeRef,
            Self::List(_) => ValueType::List,
        }
    }
    pub fn string(value: &str) -> Self {
        Self::String(value.into())
    }
    pub fn float(value: f32) -> Self {
        Self::Float(value.to_bits())
    }
    pub fn double(value: f64) -> Self {
        Self::Double(value.to_bits())
    }
}
/// Property cardinality lists differ from scalars whose ValueType itself is List.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PropertyValue {
    Scalar(Scalar),
    List(Vec<Scalar>),
}
impl PropertyValue {
    pub fn scalars(&self) -> &[Scalar] {
        match self {
            Self::Scalar(v) => std::slice::from_ref(v),
            Self::List(v) => v,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Cardinality {
    ZeroOrOne,
    List,
    One(Option<Scalar>),
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum EdgeCardinality {
    One,
    ZeroOrOne,
    #[default]
    List,
}
impl EdgeCardinality {
    pub fn accepts(self, count: usize) -> bool {
        match self {
            Self::One => count == 1,
            Self::ZeroOrOne => count <= 1,
            Self::List => true,
        }
    }
    pub fn union(self, other: Self) -> Self {
        self.max(other)
    }
}
