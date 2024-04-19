pub fn main() {
    let output = guest::string_vecs(20);
    println!("output: {:?}", output);
    let (prove, verify) = guest::build_string_vecs();

    let (output, proof) = prove(20);
    let is_valid = verify(proof);

    println!("output: {:?}", output);
    println!("valid: {}", is_valid);
}
