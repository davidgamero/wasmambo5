use std::collections::HashMap;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
// use spin_sdk::{
//     http_component,
//     http::{
//         Request,
//         Response,
//         Router,
//         Params,
//     },
// };
use spin_sdk::{
    http::{Request, Response},
    http_component,
};

/// A simple Spin HTTP component.
#[http_component]
fn handle_wasmambo5(req: Request) -> Result<Response> {
    println!("{:?}", req.headers());

    let mut router = ImTheRouterNow::new();
    router.add_route("/move", api::move_);
    router.add_route("/somethingelse", api::echo_wildcard);
    router.add_route("/", api::info);

    let routed = router.handle(req);
    match routed {
        Ok(r) => {
            println!("routed");
            return Ok(r);
        }
        Err(e) => {
            println!("error");
        }
    }

    Ok(http::Response::builder()
        .status(200)
        .header("foo", "bar")
        .body(Some("Hello, Fermyon".into()))?)
}

// uSe WeB AsSeMbLy iTs tHe FuTuRe hErE I Am mAkiNg a RoUtEr iN 2023
pub struct ImTheRouterNow {
    routes: Vec<(&'static str, fn(Request) -> Result<Response>)>,
}

impl ImTheRouterNow {
    pub fn new() -> Self {
        Self { routes: Vec::new() }
    }

    pub fn add_route(&mut self, path: &'static str, handler: fn(Request) -> Result<Response>) {
        self.routes.push((path, handler));
    }

    pub fn handle(&self, req: Request) -> Result<Response> {
        let u = req.uri().to_string();
        println!("uri: {}", u);
        for (path, handler) in &self.routes {
            if u.starts_with(path) {
                return handler(req);
            }
        }
        Ok(http::Response::builder()
            .status(http::StatusCode::NOT_FOUND)
            .body(None)?)
    }
}

mod logic;

mod api {
    use super::*;

    // /hello/:planet
    pub fn info(_req: Request) -> Result<Response> {

        Ok(http::Response::builder()
            .status(http::StatusCode::OK)
            .body(Some(logic::info()))?)
    }

    pub fn move_(_req: Request) -> Result<Response> {
        let b = _req.body().as_ref().unwrap();
        //turn b into GAmestate
        let r: std::result::Result<GameState, serde_json::Error> = serde_json::from_slice(&b);
        match r {
            Ok(gs) => Ok(http::Response::builder()
                .status(http::StatusCode::OK)
                .body(Some(logic::get_move(
                    &gs.game, &gs.turn, &gs.board, &gs.you,
                )))?),
            Err(e) => {
                println!("error: {:?}", e);
                Ok(http::Response::builder()
                    .status(http::StatusCode::BAD_REQUEST)
                    .body(Some("error".into()))?)
            }
        }
    }

    // /*
    pub fn echo_wildcard(_req: Request) -> Result<Response> {
        // let capture = params.wildcard().unwrap_or_default();
        Ok(http::Response::builder()
            .status(http::StatusCode::OK)
            .body(Some(
                format!("my patience is exhaused. go make this method yourself.").into(),
            ))?)
    }
}

#[derive(Serialize, Deserialize)]
pub struct Game {
    id: String,
    ruleset: HashMap<String, Value>,
    timeout: u32,
}

#[derive(Serialize, Deserialize)]
pub struct Board {
    height: u32,
    width: u32,
    food: Vec<Coord>,
    snakes: Vec<Battlesnake>,
    hazards: Vec<Coord>,
}

#[derive(Serialize, Deserialize)]
pub struct Battlesnake {
    id: String,
    name: String,
    health: u32,
    body: Vec<Coord>,
    head: Coord,
    length: u32,
    latency: String,
    shout: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Coord {
    x: u32,
    y: u32,
}

#[derive(Serialize, Deserialize)]
pub struct GameState {
    game: Game,
    turn: u32,
    board: Board,
    you: Battlesnake,
}
