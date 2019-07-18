// enum Direction {
//     N,
//     E,
//     S,
//     W
// }

// enum PlayerAction {
//     Move {
//         direction: Direction,
//         speed: u8
//     },
//     Wait,
//     Attack(Direction)
// }

struct Player {
    name: String,
    iq: u8,
    friends: u8
}

impl Player {
    fn with_name(name: &str) -> Player {
        Player {
            name: name.to_string(),
            iq: 100,
            friends: 100
        }
    }

    fn get_friends(&self) -> u8 {
        self.friends
    }

    fn set_friends(&mut self, count: u8) {
        self.friends = count;
    }
}

fn main() {
    // let simulated_player_action = PlayerAction::Move {
    //     direction: Direction::N,
    //     speed: 2,
    // };
    // match simulated_player_action {
    //     PlayerAction::Wait => println!("Player wants to wait"),
    //     PlayerAction::Move { direction, speed } => println!("Player wants to move in direction {:?} with speed {}", direction, speed),
    //     PlayerAction::Attack(direction) => println!("Player wants to attack direction {:?}", direction)
    // };

    let mut player = Player::with_name("Jonathan");
    player.set_friends(23);
    println!("{}'s friends count: {}", player.name, player.get_friends());
    let _ = Player::get_friends(&player);
}
