fn main() {
    if let Err(error) = minigrep::run() {
        println!("Error: \"{}\"", error);
    }
}
