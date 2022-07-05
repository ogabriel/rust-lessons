pub fn main() {
    let mut uma_string = String::from("Gabriel");

    println!("{}", uma_string);

    emprestar(&mut uma_string);

    println!("{}", uma_string);
}

fn roubar(string:String) {
    println!("{}", string);
}

fn emprestar(string:&mut String) {
    string.push_str(" Oliveira");
    println!("{}", string);
}
