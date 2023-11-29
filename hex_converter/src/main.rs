fn main() {
    println!("{}", convert_hex("Supply Chain Security"));
}

fn convert_hex(input: &str) -> String {
   return const_hex::encode_upper(input.as_bytes());
}
