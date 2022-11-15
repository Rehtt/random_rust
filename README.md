```rust
#[cfg(test)]
mod tests {
    use chrono::{Duration, Local};
    use super::*;

    #[test]
    fn it_works() {
        let a: DateTime<Local> = Local::now();
        println!("{}", rand_time(a.sub(Duration::days(180)),a));
        println!("{}", rand_id_no(a));
        println!("{}", rand_phone());
        println!("{}",rand_name());
    }
}
```