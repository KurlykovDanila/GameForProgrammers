use std::fmt::Display;

use super::{
    character::Bullet,
    map::{CanMove, Cell, Map},
    player::{Action, DynPlayer, PlayerInfo},
    traits::Movable,
};

use ez_colorize::ColorizeDisplay;
use serde::Serialize;

pub trait LikeGame {
    fn update(&mut self, actions: Vec<Vec<Action>>);
    fn validate_actions(&self, actions: &mut Vec<Action>);
    fn state(&self) -> &GameState;
    fn get_actions_count(&self) -> usize;
}

impl LikeGame for Game {
    fn validate_actions(&self, actions: &mut Vec<Action>) {
        if actions.len() == self.actions_count {
        } else if actions.len() < self.actions_count {
            for _ in actions.len()..self.actions_count {
                actions.push(Action::Nothing);
            }
        } else {
            for _ in self.actions_count..actions.len() {
                actions.pop();
            }
        }
    }

    fn get_actions_count(&self) -> usize {
        self.actions_count
    }

    fn state(&self) -> &GameState {
        &self.state
    }

    fn update(&mut self, actions: Vec<Vec<Action>>) {
        match self.state {
            GameState::TimeIsOver { .. } => return,
            GameState::End { .. } => return,
            GameState::Continue { .. } | GameState::NotStarted { .. } => {
                for action_ind in 0..self.actions_count {
                    for pl_ind in 0..self.players.len() {
                        self.execute_action(pl_ind, actions[pl_ind][action_ind]);
                    }
                }
                self.bullet_update();
                self.time_update();

                self.game_state_update();
            }
            GameState::Empty => return,
        }
    }
}

pub struct Game {
    state: GameState,
    map: Map,
    players: Vec<Box<dyn DynPlayer>>,
    time_limit: u16,
    bullets: Vec<Bullet>,
    actions_count: usize,
}

impl Game {
    pub fn new(
        map: Map,
        mut players: Vec<Box<dyn DynPlayer>>,
        time_limit: u16,
        actions_count: usize,
    ) -> Self {
        players.sort_by(|f, s| f.get_id().cmp(&s.get_id()));
        let mut game = Self {
            state: GameState::Empty,
            map,
            players,
            time_limit,
            bullets: Vec::new(),
            actions_count,
        };
        game.state = GameState::NotStarted {
            info: GameInfo::new(&game),
        };
        game
    }

    fn check_time_limit_over(&mut self) {
        if self.time_limit == 0 {
            self.state = GameState::TimeIsOver {
                winners: self.alives(),
            };
        }
    }

    fn time_update(&mut self) {
        self.time_limit -= 1;
    }

    fn game_state_update(&mut self) {
        if let GameState::Continue { .. } | GameState::NotStarted { .. } = self.state {
            self.state = GameState::Continue {
                info: GameInfo::new(self),
            };
        }
        self.have_winner();
        self.check_time_limit_over();
    }

    fn alives(&self) -> Vec<u8> {
        let mut alives = Vec::new();
        for pl in self.players.iter() {
            if pl.alive() {
                alives.push(pl.get_id());
            }
        }
        return alives;
    }

    fn have_winner(&mut self) {
        let alives = self.alives();
        if alives.len() <= 1 {
            self.state = GameState::End { winners: alives }
        }
    }

    fn bullet_update(&mut self) {
        let max_range = self.bullets.iter().max_by(|a, b| a.range.cmp(&b.range));
        let count;
        if let Some(bullet) = max_range {
            count = bullet.range;
        } else {
            return;
        }
        for _ in 0..count {
            '_move: for b in self.bullets.iter_mut() {
                if b.can_move() {
                    let bullet_pos = b.get_position();
                    for p in self.players.iter_mut() {
                        if p.get_position() == bullet_pos {
                            p.get_damage(b.use_up());
                            continue '_move;
                        }
                    }
                    let bullet_direction = b.get_direction();
                    match self.map.can_move(bullet_pos, bullet_direction) {
                        CanMove::Yes => {
                            b.shift(bullet_direction);
                        }
                        CanMove::No(..) => {
                            continue '_move;
                        }
                    }
                }
            }
        }
        self.bullets.clear();
    }

    fn execute_action(&mut self, player_ind: usize, action: Action) {
        let player = &mut self.players[player_ind];
        let pos = player.get_position();
        match action {
            Action::Attack { direction } => {
                if let Some(bullet) = player.attack(direction) {
                    self.bullets.push(bullet);
                }
            }
            Action::Move { direction, range } => {
                for _ in 0..player.get_speed().min(range) {
                    match self.map.can_move(pos, direction) {
                        CanMove::Yes => {
                            player.shift(direction);
                        }
                        CanMove::No(_) => return,
                    }
                }
            }
            Action::Reload => player.reloading(),
            Action::Nothing => {}
        }
    }
}

#[derive(Serialize)]
pub struct GameInfo {
    map: Map,
    players: Vec<PlayerInfo>,
}

impl Display for GameInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.map)
    }
}

impl Display for GameState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GameState::NotStarted { info } => {
                write!(f, "{}, state:\n{}", "Game not started".green(), info)
            }
            GameState::Continue { info } => {
                write!(f, "{}, state:\n{}", "Game continue".green(), info)
            }
            GameState::TimeIsOver { winners } => {
                write!(f, "{}, winners id: {:?}", "Time is over".green(), winners)
            }
            GameState::End { winners } => writeln!(
                f,
                "{}, winners id: {:?}",
                "End of game with winners".green(),
                winners
            ),
            GameState::Empty => write!(f, "No state, empty"),
        }
    }
}

#[derive(Default)]
pub struct GameBuilder {
    map_size: Option<u8>,
    players: Vec<Box<dyn DynPlayer>>,
    time_limit: Option<u16>,
    actions_count: Option<usize>,
}

impl GameBuilder {
    pub fn add_players(mut self, players: Vec<Box<dyn DynPlayer>>) -> Self {
        self.players = players;
        self
    }

    pub fn add_time_limit(mut self, limit: u16) -> Self {
        self.time_limit = Some(limit);
        self
    }

    pub fn add_actions_count(mut self, actions: usize) -> Self {
        self.actions_count = Some(actions);
        self
    }

    pub fn add_map_size(mut self, map_size: u8) -> Self {
        self.map_size = Some(map_size);
        self
    }

    pub fn build(self) -> Box<dyn LikeGame> {
        Box::new(Game::new(
            Map::new_empty(self.map_size.unwrap_or(5)),
            self.players,
            self.time_limit.unwrap_or(1000),
            self.actions_count.unwrap_or(2),
        ))
    }
}

impl GameInfo {
    fn new(game: &Game) -> Self {
        let mut players = Vec::new();
        for pl in game.players.iter() {
            let mut info = PlayerInfo::new(pl.character());
            if let Cell::Bushes = game.map.get_cell(pl.get_position()) {
                info.without_pos();
            }
            players.push(info);
        }
        Self {
            map: game.map.clone(),
            players,
        }
    }
}

pub enum GameState {
    End { winners: Vec<u8> },
    NotStarted { info: GameInfo },
    TimeIsOver { winners: Vec<u8> },
    Continue { info: GameInfo },
    Empty,
}
