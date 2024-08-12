/// A trait for builder pattern.
///
/// This trait is implemented by the `Builder` derive macro. It provides a common interface for
/// setting fields of a struct and building the struct.
///
/// This purely works on build time and tries
/// to provide a compile time safety for building a struct, while allowing to set fields in any
/// order.
///
///
/// # Example
///
/// This is a simple example of how to use the `Builder` trait.
///
/// ```rust
/// use buco::Builder;
///
/// #[derive(Builder)]
/// struct Elements {
///     fire: u8,
///     water: String,
/// }
///
/// let elements = Elements::builder()
///    .set_water("water".to_string())
///    .set_fire(1)
///    .build();
/// ```
/// Similarly, if you miss setting a field, it will give a compile time error.
///
/// ```rust,compile_fail
/// use buco::Builder;
///
/// #[derive(Builder)]
/// struct Elements {
///     fire: u8,
///     water: String,
/// }
///
/// let elements = Elements::builder()
///    .set_water("water".to_string())
///    .build();
/// ```
///
///
pub trait Builder {}

pub use buco_derive::Builder;

#[cfg(test)]
mod tests {
    use super::*;
    use trybuild::TestCases;

    #[test]
    fn sanity_tests() {
        let t = TestCases::new();

        t.compile_fail("build_tests/01-builder-enum.rs");
        t.compile_fail("build_tests/02-builder-union.rs");
        t.pass("build_tests/03-builder-struct.rs");
        t.compile_fail("build_tests/04-builder-struct-partial.rs");
        t.pass("build_tests/05-builder-struct-complete.rs");
        t.compile_fail("build_tests/06-builder-struct-overwrite.rs");
    }

    #[test]
    fn test_common_builder() {
        #[derive(Builder)]
        struct Data {
            v1: u8,
            v2: String,
            v3: f64,
        }

        let data = Data::builder()
            .set_v1(1)
            .set_v2("hello".to_string())
            .set_v3(1.414)
            .build();

        assert_eq!(data.v1, 1);
        assert_eq!(data.v2, "hello");
        assert_eq!(data.v3, 1.414);
    }
}
