use once_cell::sync::OnceCell;

pub const CONFIG: OnceCell<()> = OnceCell::new();

pub fn load_cell() {
    println!("Setting");
    CONFIG.set(()).unwrap();
    println!("Waiting");
    CONFIG.wait();
}


fn main() {
    load_cell();
    println!("Finished");
}
