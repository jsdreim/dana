pub trait Exp: Copy + Default + Eq + Ord {
    const VALUE: i32;
}

pub trait CanMul2: Exp { type Mul2: Exp + CanDiv2<Div2=Self>; }
pub trait CanDiv2: Exp { type Div2: Exp + CanMul2<Mul2=Self>; }

pub trait CanMul3: Exp { type Mul3: Exp + CanDiv3<Div3=Self>; }
pub trait CanDiv3: Exp { type Div3: Exp + CanMul3<Mul3=Self>; }

pub trait CanMul4: Exp { type Mul4: Exp + CanDiv4<Div4=Self>; }
pub trait CanDiv4: Exp { type Div4: Exp + CanMul4<Mul4=Self>; }

pub trait CanMul5: Exp { type Mul5: Exp + CanDiv5<Div5=Self>; }
pub trait CanDiv5: Exp { type Div5: Exp + CanMul5<Mul5=Self>; }

pub trait CanMul6: Exp { type Mul6: Exp + CanDiv6<Div6=Self>; }
pub trait CanDiv6: Exp { type Div6: Exp + CanMul6<Mul6=Self>; }


//region Generic division.
pub trait CanDiv<E>: Exp { type Quotient: Exp; }

impl<E: Exp> CanDiv<E1> for E { type Quotient = E; }
impl<E: CanDiv2> CanDiv<E2> for E { type Quotient = E::Div2; }
impl<E: CanDiv3> CanDiv<E3> for E { type Quotient = E::Div3; }
impl<E: CanDiv4> CanDiv<E4> for E { type Quotient = E::Div4; }
impl<E: CanDiv5> CanDiv<E5> for E { type Quotient = E::Div5; }
impl<E: CanDiv6> CanDiv<E6> for E { type Quotient = E::Div6; }
//endregion


//region Generic multiplication.
pub trait CanMul<E>: Exp { type Product: Exp; }

impl<E: Exp> CanMul<E1> for E { type Product = E; }
impl<E: CanMul2> CanMul<E2> for E { type Product = E::Mul2; }
impl<E: CanMul3> CanMul<E3> for E { type Product = E::Mul3; }
impl<E: CanMul4> CanMul<E4> for E { type Product = E::Mul4; }
impl<E: CanMul5> CanMul<E5> for E { type Product = E::Mul5; }
impl<E: CanMul6> CanMul<E6> for E { type Product = E::Mul6; }
//endregion


macro_rules! def_exp {
    { $(
    type $name:ident = $val:literal
    $(($($op:tt $fac:tt = $mul:ident),*))?
    ;
    )* } => {
        $(
        #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
        pub struct $name;
        impl Exp for $name { const VALUE: i32 = $val; }

        $($(
        def_exp!($name $op $fac = $mul);
        )*)?

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(stringify!($val))
            }
        }
        )*
    };
    //region Operation graph.
    ($base:ident * 2 = $mul2:ident) => {
        impl CanMul2 for $base { type Mul2 = $mul2; }
        impl CanDiv2 for $mul2 { type Div2 = $base; }
    };
    ($base:ident * 3 = $mul3:ident) => {
        impl CanMul3 for $base { type Mul3 = $mul3; }
        impl CanDiv3 for $mul3 { type Div3 = $base; }
    };
    ($base:ident * 4 = $mul4:ident) => {
        impl CanMul4 for $base { type Mul4 = $mul4; }
        impl CanDiv4 for $mul4 { type Div4 = $base; }
    };
    ($base:ident * 5 = $mul5:ident) => {
        impl CanMul5 for $base { type Mul5 = $mul5; }
        impl CanDiv5 for $mul5 { type Div5 = $base; }
    };
    ($base:ident * 6 = $mul6:ident) => {
        impl CanMul6 for $base { type Mul6 = $mul6; }
        impl CanDiv6 for $mul6 { type Div6 = $base; }
    };
    //endregion
}

def_exp! {
    type E0  =  0 (*2 = E0, *3 = E0, *4 = E0, *5 = E0, *6 = E0);
    type E1  =  1 (*2 = E2, *3 = E3, *4 = E4, *5 = E5, *6 = E6);
    type E2  =  2 (*2 = E4, *3 = E6, *4 = E8, *5 = E10, *6 = E12);
    type E3  =  3 (*2 = E6, *3 = E9, *4 = E12, *5 = E15);
    type E4  =  4 (*2 = E8, *3 = E12, *4 = E16);
    type E5  =  5 (*2 = E10, *3 = E15);
    type E6  =  6 (*2 = E12);
    type E7  =  7 (*2 = E14);
    type E8  =  8 (*2 = E16);
    type E9  =  9;
    type E10 = 10;
    type E11 = 11;
    type E12 = 12;
    type E13 = 13;
    type E14 = 14;
    type E15 = 15;
    type E16 = 16;
}


/// Struct used to implement a very cool hack ([`crate::Quantity::pow`]).
pub struct Num<const N: i32>;

pub trait ExpImplemented { type Exp: Exp; }

impl ExpImplemented for Num<2> { type Exp = E2; }
impl ExpImplemented for Num<3> { type Exp = E3; }
impl ExpImplemented for Num<4> { type Exp = E4; }
impl ExpImplemented for Num<5> { type Exp = E5; }
impl ExpImplemented for Num<6> { type Exp = E6; }
impl ExpImplemented for Num<7> { type Exp = E7; }
impl ExpImplemented for Num<8> { type Exp = E8; }
impl ExpImplemented for Num<9> { type Exp = E9; }
impl ExpImplemented for Num<10> { type Exp = E10; }
impl ExpImplemented for Num<11> { type Exp = E11; }
impl ExpImplemented for Num<12> { type Exp = E12; }
impl ExpImplemented for Num<13> { type Exp = E13; }
impl ExpImplemented for Num<14> { type Exp = E14; }
impl ExpImplemented for Num<15> { type Exp = E15; }
impl ExpImplemented for Num<16> { type Exp = E16; }
