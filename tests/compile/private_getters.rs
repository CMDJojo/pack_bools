mod inner {
    use pack_bools::pack_bools;

    #[pack_bools(get =)]
    pub struct MyBools {
        pub a: bool,
        #[pack_bools(get = get_b)]
        pub(super) b: bool,
    }
}

fn main() {
    use inner::MyBools;
    let mut x = MyBools {
        packed_bools: 0
    };
    let y = x.get_a();
    let z = x.get_b();
}
