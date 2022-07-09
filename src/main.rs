mod core;

fn main() {
    print!("\x1B[2J\x1B[1;1H");
    println!("STARTING MAAI | VERSION {}", env!("CARGO_PKG_VERSION"));

    let data: core::input::Data = core::input::Data { payload: "" };
    data.start()
}
