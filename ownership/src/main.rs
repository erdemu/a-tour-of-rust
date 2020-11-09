fn uppercase(input: &String) {
    input.to_uppercase();
}

fn main() {
    let output = String::from("hello world.");
    uppercase(&output);
    print!("{}", output);
}
