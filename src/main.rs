use textadventure::{get_input, World, Game, commands::CommandRepo};
fn main() {
    println!("Welcome to the Rust Text Adventure");
    println!("Have fun!");
    let mut world = World::init_world();
    let mut command_repo = CommandRepo::new();
    let mut game = Game::new(world, command_repo);
    game_loop(game);
}

fn game_loop(game: Game) {
    let mut is_dead = false;
    while !is_dead {
        game.world.player.location.print_location();
        println!("hp: {}>>", game.world.player.hp);
        let player_input = get_input();
        textadventure::commands::parse_command(player_input);
        is_dead = true;
    }
}
