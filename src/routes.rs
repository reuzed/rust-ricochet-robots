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

// We need routes for solving arbitrary boards, and the default boards that come with the game - these default ones cann be handled by frontend.
// We don't want to force our solver structs to be constant or implement serialisation traits, so just reimplement API for Position etc. here

#[derive(Deserialize)]
pub struct SolveBoardRequest {
    width: u32,
    height: u32,
    yellow_pos: (u32, u32),
    red_pos: (u32, u32),
    green_pos: (u32, u32),
    blue_pos: (u32, u32),
    silver_pos: Option<(u32, u32)>,
    target_pos: (u32, u32),
    horizontal_walls: Vec<(u32, u32)>,
    vertical_walls: Vec<(u32, u32)>,
}

#[derive(Serialize)]
pub struct SolveBoardResponse {
    // maybe vec of moves 
}
