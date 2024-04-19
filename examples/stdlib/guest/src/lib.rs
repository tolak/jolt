#![no_main]

#[jolt::provable]
fn int_to_string(n: i32) -> String {
    n.to_string()
}

#[jolt::provable]
fn string_vecs(n: u32) -> String {
    let mut res = String::new();
    for i in 65..65+n {
        res += &i.to_string();
    }

    res
}
