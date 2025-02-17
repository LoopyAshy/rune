use rune::compile::CompileErrorKind::*;
use rune::span;
use rune_tests::*;

#[test]
fn test_bad_attributes() {
    assert_compile_error! {
        r#"pub fn main() { #[foo] #[bar] let x = 1; }"#,
        span, Custom { message } => {
            assert_eq!(message.as_ref(), "attributes are not supported");
            assert_eq!(span, span!(16, 29));
        }
    };
}
