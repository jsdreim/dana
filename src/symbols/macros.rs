/// Define groups of related symbols.
macro_rules! define_groups {
    //  Define a group module, containing symbols from other modules.
    ($($(#[$attr:meta])*
    $vis:vis mod $module:ident ($($import:tt),* $(,)?);)*) => {
        $($(#[$attr])*
        $vis mod $module {
            $(define_groups!(@ $import);)*
        })*
    };

    //region Internal.
    //  Export specific type aliases.
    (@ [$($utype:ident),* $(,)?]) => {
        pub use super::types::{$($utype),*};
    };

    //  Export all unit consts from a symbols submodule.
    (@ $module:ident) => {
        pub use super::$module::units::*;
    };

    //  Export everything from another group.
    (@ ($group:ident)) => {
        pub use super::$group::*;
    };
    //endregion
}


/// Define symbols for units and types. This macro should only be invoked once.
macro_rules! define_symbols {
    //  Define type modules, with symbols for types and their variants.
    ($($vis:vis mod $module:ident for type $unit:tt $(as $($uname:ident),+)? {
        $(const $alias:ident $(: $atype:tt)? = $val:tt);*   $(;)?
    })*) => {
        $(
        #[allow(unused_imports)] $vis use self::$module::types::*;
        #[allow(unused_imports)] $vis use self::$module::units::*;
        #[allow(unused_imports)]
        $vis mod $module {
            pub use types::*;
            pub use units::*;

            /// Type aliases.
            pub mod types {
                $($(pub type $uname = super::super::$unit;)+)?
            }

            /// Unit consts.
            pub mod units {
                use super::super::*;

                $(define_symbols!(
                    @ super::super::$unit;
                    $alias $(: $atype)? = $val
                );)*
            }
        }
        )*

        /// All type aliases.
        pub mod types {
            $($(pub use super::$module::types::{$($uname),+};)?)*
        }
    };

    //region Internal.
    /*//  Debug.
    (@ $($t:tt)*) => {compile_error!(stringify!($($t)*));};*/

    //  Define an alias as an enum variant.
    (@ $utype:ty; $alias:ident            = $variant:ident) => {
        pub const $alias: $utype = <$utype>::$variant;
    };
    //  Define an alias with a unit expression.
    (@ $utype:ty; $alias:ident            = ($($unit:tt)*)) => {
        pub const $alias: $utype = unit!($($unit)*);
    };
    //  Define an alias as an arbitrary expression.
    (@ $utype:ty; $alias:ident            = $value:expr) => {
        pub const $alias: $utype = $value;
    };
    //  Define an alias with a type expression AND a unit expression.
    (@ $utype:ty; $alias:ident: $atype:tt = $($unit:tt)*) => {
        pub const $alias: utype!($atype) = unit!($($unit)*);
    };
    //endregion
}
