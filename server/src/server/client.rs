use std::net::TcpStream;

use tungstenite::WebSocket;

pub trait DynClient: WithHero + WithWS {
    fn add_hero_id(&mut self, hero_id: u8);
}


impl DynClient for Client {
    fn add_hero_id(&mut self, hero_id: u8) {
        self.hero_id = Some(hero_id);
    }
}

pub struct Client {
    ws: WebSocket<TcpStream>,
    hero_id: Option<u8>,
}

impl Client {
    pub fn new(ws: WebSocket<TcpStream>) -> Self {
        Self { ws, hero_id: None }
    }
}

pub trait WithWS {
    fn get_websocket(&mut self) -> &mut WebSocket<TcpStream>;
}

pub trait WithHero {
    fn get_hero_id(&self) -> Option<u8>;
}

impl WithWS for Client {
    fn get_websocket(&mut self) -> &mut WebSocket<TcpStream> {
        &mut self.ws
    }
}
impl WithHero for Client {
    fn get_hero_id(&self) -> Option<u8> {
        self.hero_id
    }
}
