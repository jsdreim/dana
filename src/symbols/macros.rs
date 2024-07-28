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
    ($(
    $(#[$attr_mod:meta])*
    $vis:vis mod $module:ident // Define module name.
    for type $utype:tt // Specify unit type this module focuses on.
    $(in mod $parent:ident)? // Specify a "parent" to borrow type aliases from.
    $(as $($alias_type:ident),+)? // Define type aliases to go in this module.
    {
        $($(type $alias_priv:ident = $upriv:tt;)+)? // New non-global types.
        $($(use $import_from:ident;)+)? // Reuse consts from another module.
        $(const $alias_const:ident $(: $atype:tt)? = $val:tt;)* // New consts.
    }
    )*) => {
        $(
        #[allow(unused_imports)] $vis use self::$module::types::*;
        #[allow(unused_imports)] $vis use self::$module::units::*;
        #[allow(unused_imports)]
        $(#[$attr_mod])*
        $vis mod $module {
            pub use types::*;
            pub use units::*;

            /// Type aliases.
            pub mod types {
                $(pub use super::super::$parent::types::*;)*

                $($(pub type $alias_priv = super::super::$upriv;)+)?
                $($(pub type $alias_type = super::super::$utype;)+)?
            }

            /// Unit consts.
            pub mod units {
                $($(pub use super::super::$import_from::units::*;)+)?
                use super::super::*;

                $(define_symbols!(
                    @ super::super::$utype;
                    $alias_const $(: $atype)? = $val
                );)*
            }
        }
        )*

        /// All type aliases.
        pub mod types {
            $($(pub use super::$module::types::{$($alias_type),+};)?)*
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
