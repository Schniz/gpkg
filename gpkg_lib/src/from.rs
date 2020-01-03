/// Makes it easy to migrate between errors and an enum of errors
///
/// ```rust
/// #[macro_use]
/// extern crate gpkg_lib;
///
/// #[derive(Debug)]
/// enum Errors {
///     IoError(std::io::Error),
///     SerdeError(serde_json::Error),
///     ErrorCode(i32),
/// }
///
/// from!(Errors, {
///     std::io::Error => IoError,
///     serde_json::Error => SerdeError,
///     i32 => ErrorCode
/// });
///
/// # fn main() {
/// match Errors::from(666) {
///   Errors::ErrorCode(666) => {},
///   err => panic!(format!("Got {:?}, but needed ErrorCode(666)", err)),
/// }
/// # }
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
