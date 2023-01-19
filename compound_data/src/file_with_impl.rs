#[derive(Debug)]  // Allows the println! macro to print File
struct File {
    name: String,
    data: Vec<u8>,
}

impl File {
    fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
        }
    }
}

fn main() {
    let f3 = File::new("f3.txt");

    lef f3_name = &f3.name;
    let f3_length = f3.data.len();

    println!(":?", f3);
    println!("{} is {} bytes long", f3_name, f3_length);
}
