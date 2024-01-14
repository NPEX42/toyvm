mod utils;

fn main() {
    println!("WasmRT - V0.1");

    println!("{:?}", utils::read_bytes("fib.wasm".into()))
    
}
