use std::error::Error;
use utils::*;
use vercel_lambda::lambda;

fn main() -> Result<(), Box<dyn Error>> {
    Ok(lambda!(convert_handler))
}
