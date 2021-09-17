use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    api::convert::main()
}
