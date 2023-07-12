use std::collections::HashMap;

use serde_json::Value;
use anyhow::Result;
use spin_sdk::{
    http_component,
    http::{
        Request, 
        Response, 
        Router, 
        Params,
    },
};

mod logic;

/// A Spin HTTP component that internally routes requests.
#[http_component]
fn handle_route(req: Request) -> Result<Response> {
    let mut router = Router::new();
    router.get("/info", api::info);
    router.any("/*", api::echo_wildcard);
    router.handle(req)
}

mod api {
    use super::*;

    // /hello/:planet
    pub fn info(_req: Request, params: Params) -> Result<Response> {
        //let planet = params.get("planet").expect("PLANET");

        Ok(http::Response::builder()
            .status(http::StatusCode::OK)
            .body(Some(logic::info()))?)
    }

    // /*
    pub fn echo_wildcard(_req: Request, params: Params) -> Result<Response> {
        let capture = params.wildcard().unwrap_or_default();
        Ok(http::Response::builder()
            .status(http::StatusCode::OK)
            .body(Some(format!("{capture}").into()))?)
    }
}

pub struct Game {
    id: String,
    ruleset: HashMap<String, Value>,
    timeout: u32,
}

pub struct Board {
    height: u32,
    width: u32,
    food: Vec<Coord>,
    snakes: Vec<Battlesnake>,
    hazards: Vec<Coord>,
}

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

pub struct Coord {
    x: u32,
    y: u32,
}

pub struct GameState {
    game: Game,
    turn: u32,
    board: Board,
    you: Battlesnake,
}

