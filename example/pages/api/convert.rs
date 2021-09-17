use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    api_rs::convert::main()
}
