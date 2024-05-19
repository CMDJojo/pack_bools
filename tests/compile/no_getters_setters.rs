use pack_bools::pack_bools;

#[pack_bools(no_get, no_set)]
struct MyBools {
    a: bool,
    b: bool,
}

fn main() {
    let mut x = MyBools {
        packed_bools: 0
    };
    let y = x.get_a();
    x.set_b(true);
}
