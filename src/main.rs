extern crate redis;

use redis::{Client, Commands};

fn main() {
    let client = Client::open("redis://127.0.0.1/").unwrap();
    let conn = client.get_connection().unwrap();
    let _: () = conn.set("answer", 42i32).unwrap();
    let answer: i32 = conn.get("answer").unwrap();
    let result: redis::RedisResult<bool> = conn.expire("answer", 1);
    println!("Answer: {} {:?}", answer, result);
}
