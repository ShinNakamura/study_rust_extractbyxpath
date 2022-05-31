fn main() {
    if let Err(err) = extractbyxpath::run() {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}
