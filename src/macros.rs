macro_rules! dummy {
    ($(#[$attr:meta])* $vis:vis trait $name:ident: $($traits:tt)*) => {
        $(#[$attr])*
        $vis trait $name: $($traits)* {}
        impl<T: $($traits)*> $name for T {}
    };
}
