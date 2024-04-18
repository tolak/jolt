pub fn main() {
    let (prove, verify) = guest::build_string_vecs();

    let (output, proof) = prove(81);
    let is_valid = verify(proof);

    println!("output: {:?}", output);
    println!("valid: {}", is_valid);
}
