use crate::runtime as rt;
use crate::runtime::RawStr;
use crate::Hash;
use std::cmp;
use std::hash;
use std::vec;

/// Static type information.
#[derive(Debug)]
#[repr(C)]
pub struct StaticType {
    /// The name of the static type.
    pub name: RawStr,
    /// The hash of the static type.
    pub hash: Hash,
}

impl cmp::PartialEq for &'static StaticType {
    fn eq(&self, other: &Self) -> bool {
        self.hash == other.hash
    }
}

impl cmp::Eq for &'static StaticType {}

impl hash::Hash for &'static StaticType {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.hash.hash(state)
    }
}

/// The specialized type information for a unit.
pub static UNIT_TYPE: &StaticType = &StaticType {
    name: RawStr::from_str("unit"),
    hash: Hash::new(0x9de148b05752dbb3),
};

impl_static_type!(() => UNIT_TYPE);
impl_static_type!(rt::UnitStruct => UNIT_TYPE);

/// The specialized type information for a byte type.
pub static BYTE_TYPE: &StaticType = &StaticType {
    name: RawStr::from_str("byte"),
    hash: Hash::new(0x190cacf7c7187189),
};

impl_static_type!(u8 => BYTE_TYPE);

/// The specialized type information for a bool type.
pub static BOOL_TYPE: &StaticType = &StaticType {
    name: RawStr::from_str("bool"),
    hash: Hash::new(0xbe6bff4422d0c759),
};

impl_static_type!(bool => BOOL_TYPE);

/// The specialized type information for a char type.
pub static CHAR_TYPE: &StaticType = &StaticType {
    name: RawStr::from_str("char"),
    hash: Hash::new(0xc56a31d061187c8b),
};

impl_static_type!(char => CHAR_TYPE);

/// The specialized type information for a integer type.
pub static INTEGER_TYPE: &StaticType = &StaticType {
    name: RawStr::from_str("integer"),
    hash: Hash::new(0xbb378867da3981e2),
};

impl_static_type!(i8 => INTEGER_TYPE);
impl_static_type!(u16 => INTEGER_TYPE);
impl_static_type!(i16 => INTEGER_TYPE);
impl_static_type!(u32 => INTEGER_TYPE);
impl_static_type!(i32 => INTEGER_TYPE);
impl_static_type!(u64 => INTEGER_TYPE);
impl_static_type!(i64 => INTEGER_TYPE);
impl_static_type!(u128 => INTEGER_TYPE);
impl_static_type!(i128 => INTEGER_TYPE);

/// The specialized type information for a float type.
pub static FLOAT_TYPE: &StaticType = &StaticType {
    name: RawStr::from_str("float"),
    hash: Hash::new(0x13e40c27462ed8fc),
};

impl_static_type!(f32 => FLOAT_TYPE);
impl_static_type!(f64 => FLOAT_TYPE);

/// The specialized type information for a string type.
pub static STRING_TYPE: &StaticType = &StaticType {
    name: RawStr::from_str("String"),
    hash: Hash::new(0x823ede4114ff8de6),
};

impl_static_type!(String => STRING_TYPE);
impl_static_type!(str => STRING_TYPE);

/// The specialized type information for a bytes type.
pub static BYTES_TYPE: &StaticType = &StaticType {
    name: RawStr::from_str("Bytes"),
    hash: Hash::new(0x957fa73126817683),
};

impl_static_type!(rt::Bytes => BYTES_TYPE);
impl_static_type!([u8] => BYTES_TYPE);

/// The specialized type information for a vector type.
pub static VEC_TYPE: &StaticType = &StaticType {
    name: RawStr::from_str("Vec"),
    hash: Hash::new(0x6c129752545b4223),
};

impl_static_type!(rt::Vec => VEC_TYPE);
impl_static_type!(impl<T> vec::Vec<T> => VEC_TYPE);
impl_static_type!([rt::Value] => VEC_TYPE);
impl_static_type!(impl<T> rt::VecTuple<T> => VEC_TYPE);

/// The specialized type information for an anonymous tuple type.
pub static TUPLE_TYPE: &StaticType = &StaticType {
    name: RawStr::from_str("Tuple"),
    hash: Hash::new(0x6da74f62cfa5cc1f),
};

impl_static_type!(rt::Tuple => TUPLE_TYPE);

/// The specialized type information for an anonymous object type.
pub static OBJECT_TYPE: &StaticType = &StaticType {
    name: RawStr::from_str("Object"),
    hash: Hash::new(0x65f4e1cf10b1f34c),
};

impl_static_type!(rt::Object => OBJECT_TYPE);
impl_static_type!(rt::Struct => OBJECT_TYPE);

/// The specialized type information for the range type.
pub static RANGE_TYPE: &StaticType = &StaticType {
    name: RawStr::from_str("Range"),
    hash: Hash::new(0xde6d8aadf191516b),
};

impl_static_type!(rt::Range => RANGE_TYPE);

/// The specialized type information for a future type.
pub static FUTURE_TYPE: &StaticType = &StaticType {
    name: RawStr::from_str("Future"),
    hash: Hash::new(0xafab4a2797436aee),
};

impl_static_type!(rt::Future => FUTURE_TYPE);

/// The specialized type information for a generator type.
pub static GENERATOR_TYPE: &StaticType = &StaticType {
    name: RawStr::from_str("Generator"),
    hash: Hash::new(0x50deff8c6ef7532c),
};

impl_static_type!(rt::Generator<rt::Vm> => GENERATOR_TYPE);

/// The specialized type information for a generator state type.
pub static GENERATOR_STATE_TYPE: &StaticType = &StaticType {
    name: RawStr::from_str("GeneratorState"),
    hash: Hash::new(0xdd4141d4d8a3ac31),
};

impl_static_type!(rt::GeneratorState => GENERATOR_STATE_TYPE);

/// The specialized type information for the `Stream` type.
pub static STREAM_TYPE: &StaticType = &StaticType {
    name: RawStr::from_str("Stream"),
    hash: Hash::new(0xd94133730d02c3ea),
};

impl_static_type!(rt::Stream<rt::Vm> => STREAM_TYPE);

/// The specialized type information for a result type.
pub static RESULT_TYPE: &StaticType = &StaticType {
    name: RawStr::from_str("Result"),
    hash: Hash::new(0xecec15e1363240ac),
};

impl_static_type!(impl<T, E> Result<T, E> => RESULT_TYPE);

/// The specialized type information for a option type.
pub static OPTION_TYPE: &StaticType = &StaticType {
    name: RawStr::from_str("Option"),
    hash: Hash::new(0x5e08dc3f663c72db),
};

impl_static_type!(impl<T> Option<T> => OPTION_TYPE);

/// The specialized type information for a function pointer type.
pub static FUNCTION_TYPE: &StaticType = &StaticType {
    name: RawStr::from_str("Function"),
    hash: Hash::new(0x45b788b02e7f231c),
};

impl_static_type!(rt::Function => FUNCTION_TYPE);
impl_static_type!(rt::Shared<rt::Function> => FUNCTION_TYPE);
impl_static_type!(impl<T> std::collections::HashMap<String, T> => OBJECT_TYPE);

/// The specialized type information for a fmt spec types.
pub static FORMAT_TYPE: &StaticType = &StaticType {
    name: RawStr::from_str("Format"),
    hash: Hash::new(0x8d6bddd19f58e97a),
};

impl_static_type!(rt::Format => FORMAT_TYPE);

/// The specialized type information for the iterator type.
pub static ITERATOR_TYPE: &StaticType = &StaticType {
    name: RawStr::from_str("Iterator"),
    hash: Hash::new(0xe08fbd4d99f308e9),
};

impl_static_type!(rt::Iterator => ITERATOR_TYPE);

/// The specialized type information for type objects.
pub static TYPE: &StaticType = &StaticType {
    name: RawStr::from_str("Type"),
    hash: Hash::new(0x3cb9320f24bf56f0),
};
