macro_rules! define_groups {
    //  Define a group module, containing symbols from other modules.
    ($(#[$attr:meta])*
    $vis:vis mod $module:ident ($($import:tt),* $(,)?)) => {
        $(#[$attr])*
        $vis mod $module {
            $(define_groups!(@ $import);)*
        }
    };

    ($($(#[$attr:meta])*
    $vis:vis mod $module:ident ($($import:tt),* $(,)?);)*) => {
        $(define_groups!($(#[$attr])* $vis mod $module ($($import),*));)*
    };

    //  Internal: Imports.
    (@ [$($utype:ident),* $(,)?]) => { pub use super::types::{$($utype),*}; };
    (@ ($group:ident)) => { pub use super::$group::*; };
    (@ $module:ident) => { pub use super::$module::units::*; };
}


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

                $(define_symbols!(@ super::super::$unit; $alias $(: $atype)? = $val);)*
            }
        }
        )*

        /// All type aliases.
        pub mod types {
            $($(pub use super::$module::types::{$($uname),+};)?)*
        }
    };

    //  Internal: Definitions for consts.
    (@ $utype:ty; $alias:ident            = $variant:ident) => {
        pub const $alias: $utype = <$utype>::$variant;
    };
    (@ $utype:ty; $alias:ident            = [$($unit:tt)*]) => {
        pub const $alias: $utype = $crate::unit!($($unit)*);
    };
    (@ $utype:ty; $alias:ident            = $value:expr) => {
        pub const $alias: $utype = $value;
    };
    (@ $utype:ty; $alias:ident: $atype:tt = $($unit:tt)*) => {
        pub const $alias: $crate::utype!($atype) = $crate::unit!($($unit)*);
    };
}
