use std::path::PathBuf;

use rusty_engine::prelude::*;

#[derive(Resource)]
struct GameState {
    high_score: u32,
    current_score: u32,
    enemy_label: Vec<String>,
    spawn_timer: Timer,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            high_score: 0,
            current_score: 0,
            enemy_label: Vec::new(),
            spawn_timer: Timer::from_seconds(10.0, TimerMode::Once),
        }
    }
}

fn main() {
    // Initialize the engine
    let mut game = Game::new();

    // Setup game
    let assets = PathBuf::from("./assets");
    let sprites_folder= assets.join("sprite");
    let audio_folder = assets.join("audio");
    let fonts_folder = assets.join("fonts");
    let racing_assets = sprites_folder.join("racing");

    // Player
    let player_car = racing_assets.join("car_red.png").canonicalize().unwrap();

    let player = game.add_sprite("player", player_car);
    player.translation = Vec2::new(0.0, 0.0);
    player.rotation = SOUTH_WEST;
    player.scale = 1.0;
    player.collision = true;

    // Car
    let car1 = racing_assets.join("car_yellow.png").canonicalize().unwrap();
    let car1 = game.add_sprite("car1", car1);
    car1.translation = Vec2::new(300.0, 0.0);
    car1.collision = true;

    // Game Logic
    game.add_logic(game_logic);

    // Start the game loop
    game.run(GameState::default());
}

fn game_logic(engine: &mut Engine, state: &mut GameState) {
    engine.show_colliders = true;
    for event in engine.collision_events.drain(..) {
        if event.state == CollisionState::Begin && event.pair.one_starts_with("player") {
            // Remove the sprite that player collided with
            for label in [event.pair.0, event.pair.1] {
                if label != "player" {
                    engine.sprites.remove(&label);
                }
            }
        }

        // println!("Collision detected: {:#?}", event);
        state.current_score += 1;
        println!("Current Score: {}", state.current_score);
    }


    let player = engine.sprites.get_mut("player").unwrap();
    player.translation.x += 100.0 * engine.delta_f32;
}
