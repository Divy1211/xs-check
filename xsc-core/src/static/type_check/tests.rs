use std::path::PathBuf;

use crate::r#static::info::{gen_errs_from_src, AstMap, SrcCache, TypeEnv, XsError};

fn chk(src: &str) -> Vec<XsError> {
    let path = PathBuf::from("loop_param_test.xs");
    let mut type_env = TypeEnv::new(vec![]);
    let mut ast_cache = AstMap::new();
    let src_cache = SrcCache::new();

    let result = gen_errs_from_src(&path, src, &mut type_env, &mut ast_cache, &src_cache);
    assert!(result.is_ok(), "unexpected parse errors: {:?}", result.err());

    type_env.errs.remove(&path).unwrap_or_default()
}

fn assert_const_loop_error(errs: Vec<XsError>) {
    assert_eq!(errs.len(), 1, "unexpected errors: {errs:?}");
    assert!(matches!(
        &errs[0],
        XsError::Syntax { msg, keywords, .. }
            if msg == "Cannot re-assign a value to a {0} variable"
                && keywords == &vec!["const".to_string()]
    ));
}

fn assert_non_numeric_loop_error(errs: Vec<XsError>, actual: &str) {
    assert_eq!(errs.len(), 1, "unexpected errors: {errs:?}");
    assert!(matches!(
        &errs[0],
        XsError::TypeMismatch { actual: err_actual, expected, .. }
            if err_actual == actual && expected == "int | float"
    ));
}

fn assert_redefined_name_error(errs: Vec<XsError>, name: &str, note: Option<&str>) {
    assert_eq!(errs.len(), 1, "unexpected errors: {errs:?}");
    assert!(matches!(
        &errs[0],
        XsError::RedefinedName { name: err_name, note: err_note, .. }
            if err_name == name && err_note.as_deref() == note
    ));
}

#[test]
fn allows_reusing_predeclared_loop_param_across_loops() {
    let errs = chk(r#"
        void main() {
            int i = 1;
            for (i = 0; < 2) {}
            for (i = 0; < 2) {}
        }
        "#);

    assert!(errs.is_empty(), "unexpected errors: {errs:?}");
}

#[test]
fn rejects_nested_reuse_of_the_same_loop_param() {
    let errs = chk(r#"
        void main() {
            for (i = 0; < 2) {
                for (i = 0; < 2) {}
            }
        }
        "#);

    assert_redefined_name_error(errs, "i", Some("Nested loops cannot reuse the same loop parameter"));
}

#[test]
fn rejects_defining_a_loop_param_after_the_loop() {
    let errs = chk(r#"
        void main() {
            for (i = 0; < 2) {}
            int i = 1;
        }
        "#);

    assert_redefined_name_error(errs, "i", None);
}

#[test]
fn allows_reusing_an_implicit_loop_param_in_later_loops() {
    let errs = chk(r#"
        void main() {
            for (i = 0; < 2) {}
            for (i = 0; < 2) {}
        }
        "#);

    assert!(errs.is_empty(), "unexpected errors: {errs:?}");
}

#[test]
fn rejects_nested_reuse_of_the_same_loop_param_through_non_loop_nesting() {
    let errs = chk(r#"
        void main() {
            for (i = 0; < 2) {
                if (true) {
                    while (true) {
                        for (i = 0; < 2) {}
                    }
                }
            }
        }
        "#);

    assert_redefined_name_error(errs, "i", Some("Nested loops cannot reuse the same loop parameter"));
}

#[test]
fn rejects_const_local_loop_param() {
    let errs = chk(r#"
        void main() {
            const int i = 0;
            for (i = 0; < 2) {}
        }
        "#);

    assert_const_loop_error(errs);
}

#[test]
fn allows_global_int_loop_param_inside_function() {
    let errs = chk(r#"
        int i = 0;
        void foo() {
            for (i = 0; < 2) {}
        }
        "#);

    assert!(errs.is_empty(), "unexpected errors: {errs:?}");
}

#[test]
fn rejects_global_const_loop_param_inside_function() {
    let errs = chk(r#"
        const int i = 0;
        void foo() {
            for (i = 0; < 2) {}
        }
        "#);

    assert_const_loop_error(errs);
}

#[test]
fn allows_global_static_loop_param_inside_function() {
    let errs = chk(r#"
        static int i = 0;
        void foo() {
            for (i = 0; < 2) {}
        }
        "#);

    assert!(errs.is_empty(), "unexpected errors: {errs:?}");
}

#[test]
fn allows_local_static_loop_param_inside_function() {
    let errs = chk(r#"
        void foo() {
            static int i = 0;
            for (i = 0; < 2) {}
        }
        "#);

    assert!(errs.is_empty(), "unexpected errors: {errs:?}");
}

#[test]
fn allows_function_parameter_loop_param() {
    let errs = chk(r#"
        void foo(int i = 0) {
            for (i = 0; < 2) {}
        }
        "#);

    assert!(errs.is_empty(), "unexpected errors: {errs:?}");
}

#[test]
fn allows_float_local_loop_param() {
    let errs = chk(r#"
        void main() {
            float i = 0;
            for (i = 0; < 2) {}
        }
        "#);

    assert!(errs.is_empty(), "unexpected errors: {errs:?}");
}

#[test]
fn allows_float_global_loop_param_inside_function() {
    let errs = chk(r#"
        float i = 0;
        void foo() {
            for (i = 0; < 2) {}
        }
        "#);

    assert!(errs.is_empty(), "unexpected errors: {errs:?}");
}

#[test]
fn allows_float_function_parameter_loop_param() {
    let errs = chk(r#"
        void foo(float i = 0) {
            for (i = 0; < 2) {}
        }
        "#);

    assert!(errs.is_empty(), "unexpected errors: {errs:?}");
}

#[test]
fn rejects_bool_loop_param() {
    let errs = chk(r#"
        void main() {
            bool i = true;
            for (i = 0; < 2) {}
        }
        "#);

    assert_non_numeric_loop_error(errs, "bool");
}

#[test]
fn rejects_string_loop_param() {
    let errs = chk(r#"
        void main() {
            string i = "";
            for (i = 0; < 2) {}
        }
        "#);

    assert_non_numeric_loop_error(errs, "string");
}

#[test]
fn rejects_vector_loop_param() {
    let errs = chk(r#"
        void main() {
            vector i = vector(0, 0, 0);
            for (i = 0; < 2) {}
        }
        "#);

    assert_non_numeric_loop_error(errs, "vector");
}
