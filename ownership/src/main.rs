fn uppercase(mut input: String) {
    input = input.to_uppercase();
}

fn main() {
    let mut output = String::from("hello world.");
    uppercase(output);
    print!("{}", output);
}
