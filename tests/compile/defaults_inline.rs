use pack_bools::pack_bools;

#[pack_bools(inline)]
struct MyBools {
    #[pack_bools(default = true)]
    a: bool,
    b: bool,
}

fn main() {}
