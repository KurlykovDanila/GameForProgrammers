use std::{
    collections::VecDeque,
    net::{TcpListener, TcpStream},
    time::{Duration, Instant},
};

use std::thread::sleep;

use super::client::{Client, DynClient};
use core::{
    direction::Direction,
    game::{GameBuilder, GameState, LikeGame},
    player::{Action, DynPlayer, Player},
    pos::Pos,
};
use serde::Deserialize;
use tungstenite::accept;
use tungstenite::Message;

#[derive(Deserialize)]
struct Req {
    pub actions: Vec<Action>,
}

pub struct Server<const PlayersOnGame: usize> {
    players_without_games: VecDeque<Box<dyn DynClient>>,
    games: VecDeque<(Vec<Box<dyn DynClient>>, Box<dyn LikeGame>, Duration)>,
    instant: Instant,
}

impl<const PlayersOnGame: usize> Server<PlayersOnGame> {
    pub fn new() -> Self {
        Self {
            players_without_games: VecDeque::new(),
            games: VecDeque::new(),
            instant: Instant::now(),
        }
    }

    fn new_connections_handler(&mut self, connection: TcpStream) {
        // Need refactor this shit!!!
        sleep(Duration::from_millis(1));
        match accept(connection) {
            Ok(mut websocket) => {
                if websocket.can_write() {
                    if let Ok(..) =
                        websocket.write_message(Message::Text(String::from("Accept connection")))
                    {
                        self.players_without_games
                            .push_back(Box::new(Client::new(websocket)));
                    }
                }
            }
            Err(err) => {
                println!("{}", err);
            }
        }
    }

    fn get_action_from_client(
        &mut self,
        client: &mut dyn DynClient,
        game: &Box<dyn LikeGame>,
    ) -> Vec<Action> {
        if let Ok(Message::Text(text)) = client.get_websocket().read_message() {
            let req: Result<Req, serde_json::Error> = serde_json::from_str(text.as_str());
            if let Ok(actions) = req {
                let mut actions = actions.actions;
                game.validate_actions(&mut actions);
                return actions;
            }
        }
        return vec![Action::Nothing; game.get_actions_count()];
    }

    fn games_update(&mut self) {
        if let Some((mut clients, mut game, timeout)) = self.games.pop_front() {
            if timeout < self.instant.elapsed() {
                if let GameState::TimeIsOver { winners } | GameState::End { winners } = game.state()
                {
                    for client in clients.iter_mut() {
                        if winners.contains(&client.get_hero_id().unwrap()) {
                            let _ = client
                                .get_websocket()
                                .write_message(Message::Text(String::from("Win")));
                        } else {
                            let _ = client
                                .get_websocket()
                                .write_message(Message::Text(String::from("Lose")));
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
                clients.iter_mut().for_each(|client| {
                    actions.push(self.get_action_from_client(client.as_mut(), &game))
                });
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

    fn create_new_game(&self, clients: &mut Vec<Box<dyn DynClient>>) -> Box<dyn LikeGame> {
        let mut players = Vec::new();
        let mut pos: Pos = (0, 0).into();
        let mut id = 0;
        for ind in 0..PlayersOnGame {
            pos = pos.shift(Direction::Right).unwrap();
            let pl: Box<dyn DynPlayer> = Box::new(Player::new_default_player(id, pos));
            players.push(pl);
            clients[ind].add_hero_id(id);
            id += 1;
        }
        return GameBuilder::default()
            .add_actions_count(2)
            .add_map_size(5)
            .add_time_limit(1000)
            .add_players(players)
            .build();
    }

    fn game_selection(&mut self) {
        if self.players_without_games.len() >= PlayersOnGame {
            let mut clients = Vec::with_capacity(PlayersOnGame);
            for _ in 0..PlayersOnGame {
                clients.push(self.players_without_games.pop_front().unwrap());
            }
            let game = self.create_new_game(&mut clients);
            clients.sort_by(|f, s| f.get_hero_id().unwrap().cmp(&s.get_hero_id().unwrap()));
            self.games.push_back((
                clients,
                game,
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
                    panic!("{:?}", e);
                }
            }
        }
    }
}
