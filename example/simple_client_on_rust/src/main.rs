use game_client_api::*;
use tungstenite::{connect, Message};
use url::Url;

fn main() {
    loop {
        // Connect to server
        let (mut socket, _) =
            connect(Url::parse("ws://localhost:8080").unwrap()).expect("Can't connect");
        if let Ok(Message::Text(text)) = socket.read_message() {
            // Server accept our connection, can start game
            if text.as_str() == "Accept connection" {
                // This block of code already contains your character control logic.
                // Here I am creating a state that will help me keep track of which
                // direction I moved in the last turn. You can store anything, even
                // the entire chronology of the game. Most likely you will need to
                // remember where you were before, when you need to reload the weapon,
                // where the opponent was before, etc.
                let mut my_state = MyState {
                    direction_now: Direction::None,
                };
                'game: loop {
                    // Get game state, it's all information about your hero, enemy and map
                    if let Ok(Message::Text(text)) = socket.read_message() {
                        // Deserealize game state
                        let game_state: GameInfo = serde_json::from_str(text.as_str()).unwrap();
                        println!("{:?}", game_state);
                        // Generate our actions from game state
                        let actions = player_program(&mut my_state, game_state);
                        // Send actions to server
                        socket
                            .write_message(Message::Text(
                                serde_json::to_string(&Response::new(actions.as_slice())).unwrap(),
                            ))
                            .unwrap();
                    } else {
                        // When the game is over, you can no longer read the messages
                        // because the connection will be broken
                        break 'game;
                    }
                }
            }
        }
    }
}

struct MyState {
    direction_now: Direction,
}

fn player_program(my_state: &mut MyState, game_state: GameInfo) -> [Action; 2] {
    let direction_now = my_state.direction_now.clone();
    match direction_now {
        Direction::Bottom => my_state.direction_now = Direction::Left,
        Direction::Top => my_state.direction_now = Direction::Right,
        Direction::Left => my_state.direction_now = Direction::Top,
        Direction::Right => my_state.direction_now = Direction::Bottom,
        Direction::None => my_state.direction_now = Direction::Top,
    };
    // Check if we can shoot and do it or reload gun
    if game_state.players[0].character.gun_reloading_await == 0 {
        return [
            Action::Move {
                direction: my_state.direction_now,
                range: 2,
            },
            Action::Attack {
                direction: my_state.direction_now,
            },
        ];
    } else {
        return [
            Action::Move {
                direction: my_state.direction_now,
                range: 2,
            },
            Action::Reload,
        ];
    }
}
