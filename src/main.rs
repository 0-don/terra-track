use scanner::db;

fn main() {
    db::get_db_connection();
    // scanner::main();
    // api::main();

    std::thread::spawn(|| scanner::main());
    let _ = std::thread::spawn(|| api::main()).join();
}
