mod my_module {
    pub fn print_alphabets() {
        for c in b'a'..=b'z' {
            println!("{}", c as char);
        }
        for c in b'A'..=b'Z' {
            println!("{}", c as char);
        }
    }
}

fn main() {
    my_module::print_alphabets();
}
