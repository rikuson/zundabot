fn main() {
    if let Err(e) = zundabot::run() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
