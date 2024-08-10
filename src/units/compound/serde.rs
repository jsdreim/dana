#![cfg(feature = "serde")]

use serde::{
    de::{DeserializeOwned, Unexpected},
    Deserialize, Deserializer,
    Serialize, Serializer,
};
use typenum::Integer;
use crate::{
    dimension::{CanDimDiv, CanDimInv, CanDimMul, CanDimPowType, DimType},
    prelude::*,
};


/// Helper enum for de/serialization of unit div/mul.
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
enum SerdeBinary<A: Unit, B: Unit> {
    Div(A, B),
    Mul(A, B),
}


//region Division.
impl<A: Unit, B: Unit> Serialize for UnitDiv<A, B> where
    A::Dim: CanDimDiv<B::Dim>,
    A: Serialize, B: Serialize,
{
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        SerdeBinary::Div(self.0, self.1).serialize(s)
    }
}

impl<'de, A: Unit, B: Unit> Deserialize<'de> for UnitDiv<A, B> where
    A::Dim: CanDimDiv<B::Dim>,
    A: DeserializeOwned, B: DeserializeOwned,
{
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        match Deserialize::deserialize(d)? {
            SerdeBinary::Div(a, b) => Ok(Self::new(a, b)),
            SerdeBinary::Mul(_, _) => Err(serde::de::Error::invalid_value(
                Unexpected::Str("mul"),
                &r#"string "div""#,
                //  TODO: Determine whether this makes sense outside of JSON.
            )),
        }
    }
}
//endregion


//region Multiplication.
impl<A: Unit, B: Unit> Serialize for UnitMul<A, B> where
    A::Dim: CanDimMul<B::Dim>,
    A: Serialize, B: Serialize,
{
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        SerdeBinary::Mul(self.0, self.1).serialize(s)
    }
}

impl<'de, A: Unit, B: Unit> Deserialize<'de> for UnitMul<A, B> where
    A::Dim: CanDimMul<B::Dim>,
    A: DeserializeOwned, B: DeserializeOwned,
{
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        match Deserialize::deserialize(d)? {
            SerdeBinary::Mul(a, b) => Ok(Self::new(a, b)),
            SerdeBinary::Div(_, _) => Err(serde::de::Error::invalid_value(
                Unexpected::Str("div"),
                &r#"string "mul""#,
            )),
        }
    }
}
//endregion


//region Inversion
/// Helper struct for de/serialization of inverted units.
#[derive(Deserialize, Serialize)]
struct SerdeInv<U: Unit> {
    inv: U,
}

impl<U: Unit> Serialize for PerUnit<U> where
    U::Dim: CanDimInv,
    U: Serialize,
{
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        SerdeInv { inv: self.0 }.serialize(s)
    }
}

impl<'de, U: Unit> Deserialize<'de> for PerUnit<U> where
    U::Dim: CanDimInv,
    U: DeserializeOwned,
{
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let SerdeInv { inv } = Deserialize::deserialize(d)?;
        Ok(Self::new(inv))
    }
}
//endregion


//region Exponentiation.
/// Helper struct for de/serialization of exponentiated units.
#[derive(Deserialize, Serialize)]
struct SerdePow<U: Unit> {
    base: U,
    exp: i32,
}

impl<U: Unit, E: Integer> Serialize for UnitPow<U, E> where
    U::Dim: CanDimPowType<E>,
    U: Serialize,
{
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        SerdePow { base: self.0, exp: E::I32 }.serialize(s)
    }
}

struct ExpectedInt(i32);

impl serde::de::Expected for ExpectedInt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "integer `{}`", self.0)
    }
}

impl<'de, U: Unit, E: Integer> Deserialize<'de> for UnitPow<U, E> where
    U::Dim: CanDimPowType<E>,
    U: DeserializeOwned,
{
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let SerdePow { base, exp } = Deserialize::deserialize(d)?;

        if exp == E::I32 {
            Ok(Self::new(base))
        } else {
            Err(serde::de::Error::invalid_value(
                Unexpected::Signed(exp as _),
                &ExpectedInt(E::I32),
            ))
        }
    }
}
//endregion


//region Anonymous unit.
impl<Dim: DimType> Serialize for UnitAnon<Dim> {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.0.serialize(s)
    }
}

impl<'de, Dim: DimType> Deserialize<'de> for UnitAnon<Dim> {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(Self::new(f64::deserialize(d)?))
    }
}
//endregion
