use std::{
    collections::VecDeque,
    net::{TcpListener, TcpStream},
    time::{Duration, Instant},
};

use super::client::{Client, WithHero, WithWS};
use crate::game::{
    direction::Direction,
    game::{Game, GameState},
    map::Map,
    player::{Action, Player},
    pos::Pos,
};
use serde::Deserialize;
use tungstenite::accept;
use tungstenite::Message;

#[derive(Deserialize)]
struct Req {
    pub actions: Vec<Action>,
}

pub struct Server<const ActionOnStep: usize, const PlayersOnGame: usize> {
    players_without_games: Vec<Box<Client>>,
    games: VecDeque<(Vec<Box<Client>>, Box<Game<ActionOnStep>>, Duration)>,
    instant: Instant,
}

impl<const ActionOnStep: usize, const PlayersOnGame: usize> Server<ActionOnStep, PlayersOnGame> {
    pub fn new() -> Self {
        Self {
            players_without_games: Vec::new(),
            games: VecDeque::new(),
            instant: Instant::now(),
        }
    }

    fn new_connections_handler(&mut self, connection: TcpStream) {
        println!("New");
        if let Ok(mut websocket) = accept(connection) {
            println!("New connection: {:?}", websocket.get_ref());
            if websocket.can_write() {
                if let Ok(..) =
                    websocket.write_message(Message::Text(String::from("Accept connection")))
                {
                    self.players_without_games
                        .push(Box::new(Client::new(websocket)));
                }
            }
        }
    }

    fn get_action_from_client(
        &mut self,
        client: &mut Client,
        game: &Game<ActionOnStep>,
    ) -> Vec<Action> {
        if let Ok(Message::Text(text)) = client.get_websocket().read_message() {
            let req: Result<Req, serde_json::Error> = serde_json::from_str(text.as_str());
            if let Ok(actions) = req {
                let mut actions = actions.actions;
                println!("Parsed: {:?}", actions);
                client
                    .get_websocket()
                    .write_message(Message::Text(String::from("Parsed correctly")))
                    .unwrap();
                game.validate_actions(&mut actions);
                return actions;
            }
        }
        return vec![Action::Nothing; ActionOnStep];
    }

    fn games_update(&mut self) {
        if let Some((mut clients, mut game, timeout)) = self.games.pop_front() {
            if timeout < self.instant.elapsed() {
                if let GameState::TimeIsOver { .. } = game.state() {
                    for client in clients.iter_mut() {
                        println!("End of game");
                        if let Ok(..) = client
                            .get_websocket()
                            .write_message(Message::Text(String::from("Game is over")))
                        {
                        }
                    }
                    return;
                }
                if let GameState::Continue { info } | GameState::NotStarted { info } = game.state()
                {
                    for client in clients.iter_mut() {
                        if let Ok(..) = client
                            .get_websocket()
                            .write_message(Message::Text(serde_json::to_string(info).unwrap()))
                        {
                        }
                    }
                }
                let mut actions = Vec::new();
                for client in clients.iter_mut() {
                    actions.push(self.get_action_from_client(client, &game));
                }
                game.update(actions);
                self.games.push_back((
                    clients,
                    game,
                    self.instant.elapsed() + Duration::from_secs(2),
                ));
            } else {
                self.games.push_back((clients, game, timeout));
            }
        }
    }

    fn create_new_game(&self, clients: &mut Vec<Box<Client>>) -> Game<ActionOnStep> {
        let map = Map::new_empty(4);
        let mut players = Vec::new();
        let mut pos: Pos = (0, 0).into();
        let mut id = 0;
        for ind in 0..PlayersOnGame {
            pos = pos.shift(Direction::Right).unwrap();
            players.push(Player::new_default_player(id, pos));
            clients[ind].add_hero(id);
            id += 1;
        }
        let game: Game<ActionOnStep> = Game::new(map, players, 1000);
        return game;
    }

    fn game_selection(&mut self) {
        if self.players_without_games.len() >= PlayersOnGame {
            let mut clients = Vec::with_capacity(PlayersOnGame);
            for _ in 0..PlayersOnGame {
                clients.push(self.players_without_games.pop().unwrap());
            }
            let game = self.create_new_game(&mut clients);
            clients.sort_by(|f, s| f.get_hero_id().unwrap().cmp(&s.get_hero_id().unwrap()));
            self.games.push_back((
                clients,
                Box::new(game),
                self.instant.elapsed() + Duration::from_secs(2),
            ));
        }
    }

    pub fn run(&mut self) {
        let server = TcpListener::bind("127.0.0.1:8080").unwrap();
        server.set_nonblocking(true).unwrap();
        for stream in server.incoming() {
            match stream {
                Ok(s) => {
                    self.new_connections_handler(s);
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    self.games_update();
                    self.game_selection();
                }
                Err(ref e) => {
                    panic!("AAAAAA: {:?}", e);
                }
            }
        }
    }
}
