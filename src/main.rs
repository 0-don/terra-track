fn main() {
    let _ = std::thread::spawn(|| scanner::main()).join();
    let _ = std::thread::spawn(|| api::main()).join();
}
