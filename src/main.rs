use rust_game::*;
use std::*;

mod screen;
mod tup_math;

fn main() {
    let mut win = Screen::new(None);
    let terrain = Layer::random('~');

    let mut player_pos: (i32, i32) = (0, 0);

    loop {
        win.clear();
        terrain.add_to_surface(&mut win);
        win.set(tup_i32_to_usize(player_pos), '@');

        win.render();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input == "q" || input == "Q" || input == "quit" {
            break;
        }

        for i in input.chars() {
            match i {
                'w' => player_pos.1 -= 1,
                'a' => player_pos.0 -= 1,
                's' => player_pos.1 += 1,
                'd' => player_pos.0 += 1,
                _ => {}
            }
        }

        player_pos = tup_i32_clamp(player_pos, (0,0), tup_usize_to_i32((WIDTH,HEIGHT)));
    }
}
