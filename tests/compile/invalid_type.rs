use pack_bools::pack_bools;

#[pack_bools(type = String)]
struct MyBools {
    a: bool,
    b: bool,
}

fn main() {}
