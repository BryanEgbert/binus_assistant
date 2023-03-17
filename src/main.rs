
mod api;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    println!("{}", add(1, 2));

    // Ok(())
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }
}