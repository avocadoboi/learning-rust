
fn main() {
    if let Err(error) = multithreaded_web_server::run() {
        println!("Error: \"{}\"", error)
    }
}
