pub mod game;
pub mod raws;

use game::Game;
use rltk::BError;

const WIDTH: u32 = 80;
const HEIGHT: u32 = 50;

fn main() -> BError {
    use rltk::RltkBuilder;
    let try_context = RltkBuilder::simple80x50()
        .with_dimensions(WIDTH * 2, HEIGHT * 2)
        .with_title("Dangerous Deliveries")
        .build();

    match try_context {
        Ok(context) => {
            //context.with_post_scanlines(true);

            rltk::main_loop(context, load_game())
        }
        Err(err) => Err(err),
    }
}

fn load_game() -> Game {
    raws::load_raws();
    Game::new(WIDTH, HEIGHT)
}