#[cfg(test)]
mod tests {
    use mlir_rust_stuff::{unroll};

    #[test]
    fn it_works() {
        unroll! {
            for i in 1..10 {
                println!("a");
            }
        }
    }
}