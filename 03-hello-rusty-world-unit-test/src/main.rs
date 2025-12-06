fn main() {
  
}



pub fn hello_rusty_world(param:&str) -> String {
    format!("Hello, {param}")
}


#[cfg(test)] 
    mod tests {
        use super::*;

        #[test]
        fn test_hello_rusty_world() {
            let result: String = hello_rusty_world("rusty world");
            assert_eq!(result,"Hello, rusty world", "should return Hello, rusty world got {result}")
        }

    }
