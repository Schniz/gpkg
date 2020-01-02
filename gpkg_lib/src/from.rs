/// Makes it easy to migrate between errors and an enum of errors
///
/// ```rust
/// use crate::from;
///
/// enum Errors {
///     IoError(std::io::Error),
///     SerdeError(serde_json::Error),
/// }
///
/// from!(Errors, {
///     std::io::Error => IoError,
///     serde_json::Error => SerdeError
/// });
/// ```
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
