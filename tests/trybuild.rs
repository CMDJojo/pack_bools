#[test]
fn trybuild() {
    let cases = trybuild::TestCases::new();
    cases.compile_fail("tests/compile/invalid_type.rs");
    cases.compile_fail("tests/compile/too_many_bools.rs");
    cases.pass("tests/compile/just_enough.rs");
    cases.compile_fail("tests/compile/super_many_bools.rs");
    cases.compile_fail("tests/compile/defaults_inline.rs");
    cases.compile_fail("tests/compile/no_getters.rs");
    cases.compile_fail("tests/compile/no_setters.rs");
    cases.compile_fail("tests/compile/no_getters_setters.rs");
    cases.compile_fail("tests/compile/private_getters.rs");
}
