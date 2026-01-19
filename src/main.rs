fn main() {
    let result = toon_rust::cli::run();
    if let Err(err) = result {
        eprintln!("{err}");
        std::process::exit(1);
    }
}
