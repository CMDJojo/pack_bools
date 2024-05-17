use pack_bools::pack_bools;

#[pack_bools(newtype)]
#[derive(Default)]
struct WithDefaults {
    _string: String,
    default_false: bool,
    #[pack_bools(default = false)]
    default_false_2: bool,
    #[pack_bools(default = true)]
    default_true: bool
}

#[test]
fn test_defaults() {
    let defaults = WithDefaults::default();
    assert!(!defaults.get_default_false());
    assert!(!defaults.get_default_false());
    assert!(defaults.get_default_true());
}

#[pack_bools(get = %, set = s%)]
#[derive(Default)]
struct CustomNames {
    bool_a: bool,
    bool_b: bool,
    #[pack_bools(get = pub get_custom)]
    custom: bool
}

#[test]
fn test_custom_names() {
    let mut cn = CustomNames::default();
    assert!(!cn.bool_a());
    assert!(!cn.bool_b());
    assert!(!cn.get_custom());
    cn.sbool_a(true);
    assert!(cn.bool_a());
    assert!(!cn.bool_b());
    assert!(!cn.get_custom());
    cn.sbool_b(true);
    cn.sbool_a(false);
    assert!(!cn.bool_a());
    assert!(cn.bool_b());
    assert!(!cn.get_custom());
}

#[pack_bools(no_getters)]
#[derive(Default)]
struct NoGetters {
    a: bool,
    #[pack_bools(get = get_me)]
    b: bool
}

#[test]
fn test_no_getters() {
    let x = NoGetters::default();
    assert!(!x.get_me());
}