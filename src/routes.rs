use serde::{Deserialize, Serialize};

pub fn health() -> String {
    "Hello from rrr.".to_string()
}

#[derive(Deserialize)]
pub struct TestRequest {
    a: Vec<i64>,
    b: String,
}

#[derive(Serialize)]
pub struct TestResponse {
    s: i64,
}

pub fn echo(req: TestRequest) -> TestResponse {
    println!("{}", req.b);
    TestResponse {
        s: req.a.iter().sum(),
    }
}
