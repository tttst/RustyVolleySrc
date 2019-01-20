extern crate quicksilver;

mod local_game_state;
mod duel_match;
mod game_logic;
mod physic_world;
mod game_constants;
mod vector;
mod global;
mod player_input;

use local_game_state::LocalGameState;

use quicksilver::{
    Result,
    geom::{Vector},
    lifecycle::{Settings, State, Window, Event, run},
};

impl State for LocalGameState {
    
    fn new() -> Result<LocalGameState> {
        Ok(LocalGameState::new())
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        self.step();
        // self.elapsed += window.update_rate()
        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        self.draw_window_content(window)
    }

    fn event(&mut self, event: &Event, window: &mut Window) -> Result<()> {
        self.handle_event(event, window)
    }
}

fn main() {
    run::<LocalGameState>("RustyVolley", Vector::new(800, 600), Settings {
        draw_rate: 4.0, 
        update_rate: 16.0,
        vsync: true,
        //fullscreen: true,
        ..Settings::default()
    });
}