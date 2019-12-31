#[macro_export]
macro_rules! from {
    ($enum_name:path, { $($from:path => $enum_variant:ident),* }) => {
        $(
            impl From<$from> for $enum_name {
                fn from(err: $from) -> Self {
                    Self::$enum_variant(err)
                }
            }
        )*
    };
}
