use duel_match::DuelMatch;
use duel_match::FrameEvent;
use global::PlayerSide::*;
use game_constants::*;
use simple_bot::*;
use new_game_menu_state::GameConfiguration;
use new_game_menu_state::PlayerKind::Computer;

use quicksilver::{
    Result,
    geom::{Shape, Transform, Vector},
    graphics::{Background::Img, Color, Image},
    input::{*},
    lifecycle::{Window, Event},
};

use state_manager::*;
use state_manager::StateTransition::*;

pub struct Scoring {
    score1: i32,
    score2: i32,
    score1_texture : Option<Image>,
    score2_texture : Option<Image>,
}

impl Scoring {
    pub fn new() -> Scoring {
        Scoring {
            score1: -1,
            score2: -1,
            score1_texture: None,
            score2_texture: None,
        }
    }
}

pub struct LocalGameState {
    duel_match : DuelMatch,
    frame_events: Vec<FrameEvent>,
    frame_number : usize,
    scoring : Scoring,
    bot_right : SimpleBot,
    bot_left : SimpleBot,
    use_bot_right : bool,
    use_bot_left : bool,
}

impl LocalGameState {

    pub fn new() -> LocalGameState {
        LocalGameState {
            duel_match: DuelMatch::new(),
            frame_events: vec!(),
            frame_number: 0,
            scoring: Scoring::new(),
            bot_left: SimpleBot::new(LeftPlayer, 0),
            bot_right: SimpleBot::new(RightPlayer, 0),
            use_bot_right : false,
            use_bot_left : false,
        }
    }

    pub fn reset(&mut self) {
        self.duel_match = DuelMatch::new();
        self.frame_events = vec!();
        self.frame_number = 0;
        self.scoring = Scoring::new();
    }

    pub fn set_config(&mut self, config: GameConfiguration) {
        self.use_bot_left = config.player1_configuration == Computer;
        self.use_bot_right = config.player2_configuration == Computer;
    }

    pub fn step(&mut self, game_assets: &mut GamesAssets) -> StateTransition {
        self.frame_events.clear();

        if self.use_bot_right {

            let bot_data = CurrentGameState {
                blob_positions : self.duel_match.get_world().get_blob_positions(),
                blob_velocities : self.duel_match.get_world().get_blob_velocities(),
                is_game_running : self.duel_match.get_world().is_game_running(),
                is_ball_valid : self.duel_match.get_world().is_ball_valid(),
                serving_player : self.duel_match.get_serving_player()
            };

            self.bot_right.step
            (
                bot_data,
                self.duel_match.get_world().get_ball_position(),
                self.duel_match.get_world().get_ball_velocity()
            );

            self.duel_match.get_world().set_player_input(
                RightPlayer, 
                self.bot_right.compute_input()
            );

            self.bot_right.reset_input();
        }

        if self.use_bot_left {

            let bot_data = CurrentGameState {
                blob_positions : self.duel_match.get_world().get_blob_positions(),
                blob_velocities : self.duel_match.get_world().get_blob_velocities(),
                is_game_running : self.duel_match.get_world().is_game_running(),
                is_ball_valid : self.duel_match.get_world().is_ball_valid(),
                serving_player : self.duel_match.get_serving_player()
            };

            self.bot_left.step
            (
                bot_data,
                self.duel_match.get_world().get_ball_position(),
                self.duel_match.get_world().get_ball_velocity()
            );

            self.duel_match.get_world().set_player_input(
                LeftPlayer, 
                self.bot_left.compute_input()
            );

            self.bot_left.reset_input();
        }

        self.duel_match.step(&mut self.frame_events);

        if self.frame_events.iter().any( |x|
            *x == FrameEvent::EventBlobbyHit(LeftPlayer) ||
            *x == FrameEvent::EventBlobbyHit(RightPlayer)
        ) {
            let _ = game_assets.sounds[0].execute(|sound| {
                sound.set_volume(10.0f32);
                let _ = sound.play()?;
                Ok(())
            });
        }

        if self.frame_events.iter().any( |x|
            *x == FrameEvent::EventError(LeftPlayer) ||
            *x == FrameEvent::EventError(RightPlayer)
        ) {
            let _ = game_assets.sounds[1].execute(|sound| {
                sound.set_volume(1.0f32);
                let _ = sound.play()?;
                Ok(())
            });
        }

        if self.frame_number == 0 {
            let _ = game_assets.sounds[1].execute(|sound| {
                sound.set_volume(1.0f32);
                let _ = sound.play()?;
                Ok(())
            });
        }

        self.frame_number += 1;

        if self.frame_events.iter().any( |x|
            *x == FrameEvent::EventWin(LeftPlayer)
        ) {
            StateTransition::WinStateTransition(LeftPlayer)
        }
        else if self.frame_events.iter().any( |x|
            *x == FrameEvent::EventWin(RightPlayer)
        ) {
            StateTransition::WinStateTransition(RightPlayer)
        } else {
            NoTransition
        }
    }

    pub fn draw_window_content(&mut self, window: &mut Window, game_assets: &mut GamesAssets) -> Result<()> {

        window.clear(Color::WHITE)?;

        // draw background
        {
            let transform =
                Transform::IDENTITY *
                Transform::scale(
                    Vector::new(
                        DISPLAY_SCALE_FACTOR,
                        DISPLAY_SCALE_FACTOR
                    )
                );

            game_assets.background_image.execute(|image| {
                window.draw_ex(
                    &image.area().with_center(
                        (
                            WINDOW_WIDTH as f32 / 2.0f32 * DISPLAY_SCALE_FACTOR,
                            WINDOW_HEIGHT as f32 / 2.0f32 * DISPLAY_SCALE_FACTOR
                        )
                    ),
                    Img(&image),
                    transform,
                    0.0f32
                );
                Ok(())
            })?;
        }

        // draw left player
        {
            let blob_pos = self.duel_match.get_blob_position(LeftPlayer);
            let blob_state = (self.duel_match.get_world().get_blob_state(LeftPlayer) as usize) % (BLOBBY_ANIMATION_FRAMES) ;
             let transform =
                Transform::scale(
                    Vector::new(
                        DISPLAY_SCALE_FACTOR * 2.4f32 * 0.5f32,
                        DISPLAY_SCALE_FACTOR * 2.4f32 * 0.5f32
                    )
                );

            game_assets.blobs_images_left[blob_state].execute(|image| {
                window.draw_ex(
                    &image.area().with_center(
                        (
                            blob_pos.x * DISPLAY_SCALE_FACTOR * 2.4f32,
                            blob_pos.y * DISPLAY_SCALE_FACTOR * 2.4f32
                        )
                    ),
                    Img(&image),
                    transform,
                    2.0f32
                );

                Ok(())
            })?;
        }

        // draw right player
        {
            let blob_pos = self.duel_match.get_blob_position(RightPlayer);
            let blob_state = (self.duel_match.get_world().get_blob_state(RightPlayer) as usize) % (BLOBBY_ANIMATION_FRAMES);
            let transform =
                Transform::scale(
                    Vector::new(
                        DISPLAY_SCALE_FACTOR * 2.4f32 * 0.5f32,
                        DISPLAY_SCALE_FACTOR * 2.4f32 * 0.5f32
                    )
                );

            game_assets.blobs_images_right[blob_state].execute(|image| {
                window.draw_ex(
                    &image.area().with_center(
                        (
                            blob_pos.x * DISPLAY_SCALE_FACTOR * 2.4f32,
                            blob_pos.y * DISPLAY_SCALE_FACTOR * 2.4f32
                        )
                    ),
                    Img(&image),
                    transform,
                    3.0f32
                );

                Ok(())
            })?;
        }

        // draw the ball
        {
            let ball_pos = self.duel_match.get_ball_position();
            let ball_rot = self.duel_match.get_world().get_ball_rotation();

            let transform =
                Transform::scale(
                    Vector::new(
                        DISPLAY_SCALE_FACTOR * 2.4f32 * 0.5f32,
                        DISPLAY_SCALE_FACTOR * 2.4f32 * 0.5f32
                    )
                ) *
                Transform::rotate(
                    ball_rot as f32 / std::f32::consts::PI * 180.0f32
                );

            game_assets.ball_image.execute(|image| {
                window.draw_ex(
                    &image.area().with_center(
                        (
                            ball_pos.x * DISPLAY_SCALE_FACTOR * 2.4f32,
                            ball_pos.y * DISPLAY_SCALE_FACTOR * 2.4f32
                        )
                    ),
                    Img(&image),
                    transform,
                    1.0f32
                );

                Ok(())
            })?;
        }

        //draw ball indicator
        {
            let ball_pos = self.duel_match.get_ball_position();

            if ball_pos.y < (0.0f32 - BALL_RADIUS) {

                let transform =
                    Transform::scale(
                        Vector::new(
                            DISPLAY_SCALE_FACTOR * 2.0f32,
                            DISPLAY_SCALE_FACTOR * 2.0f32
                        )
                    );

                game_assets.ball_indicator.execute(|image| {
                    window.draw_ex(
                        &image.area().with_center(
                            (
                                ball_pos.x * DISPLAY_SCALE_FACTOR * 2.4f32,
                                BALL_INDICATOR_HEIGHT as f32 / 2.0f32 * DISPLAY_SCALE_FACTOR * 2.0f32
                            )
                        ),
                        Img(&image),
                        transform,
                        5.0f32
                    );

                    Ok(())
                })?;
            }
        }

        // draw the score
        {
            let transform =
                    Transform::scale(
                        Vector::new(
                            DISPLAY_SCALE_FACTOR * 1.6f32,
                            DISPLAY_SCALE_FACTOR * 1.6f32
                        )
                    );

            let (score1, score2) = self.duel_match.get_scores();

            let should_recreate_texture =
                self.scoring.score1 != score1 ||
                self.scoring.score2 != score2 ||
                self.scoring.score1_texture.is_none() ||
                self.scoring.score2_texture.is_none();

            let cloned_font_ref = game_assets.font.clone();

            cloned_font_ref.borrow_mut().execute(|a_font| {

                if should_recreate_texture {

                    let score1_texture =
                        a_font.render(&format!("{:02}", score1), &game_assets.font_style)
                        .unwrap();

                    self.scoring.score1 = score1;
                    self.scoring.score1_texture = Some(score1_texture);

                   let score2_texture =
                       a_font.render(&format!("{:02}", score2), &game_assets.font_style)
                       .unwrap();

                    self.scoring.score2 = score2;
                    self.scoring.score2_texture = Some(score2_texture);
                }

                match self.scoring.score1_texture {
                    None => (),
                    Some(ref image) => {
                        window.draw_ex(
                            &image.area().with_center(
                                (
                                    SCORE_PADDING_X as f32 * DISPLAY_SCALE_FACTOR,
                                    SCORE_BASELINE_HEIGHT as f32 * DISPLAY_SCALE_FACTOR
                                )
                            ),
                            Img(&image),
                            transform,
                            4.0f32
                        );
                    }
                }

                match self.scoring.score2_texture {
                    None => (),
                    Some(ref image) => {
                        window.draw_ex(
                            &image.area().with_center(
                                (
                                    (WINDOW_WIDTH - SCORE_PADDING_X) as f32 * DISPLAY_SCALE_FACTOR,
                                    SCORE_BASELINE_HEIGHT as f32 * DISPLAY_SCALE_FACTOR
                                )
                            ),
                            Img(&image),
                            transform,
                            4.0f32
                        );
                    }
                }

                Ok(())
            })?;
        }

        Ok(())
    }

    pub fn handle_event(&mut self, event: &Event, _window: &mut Window) -> StateTransition {
        let mut player_right_input = self.duel_match.get_world().get_player_input(RightPlayer);
        let mut player_left_input = self.duel_match.get_world().get_player_input(LeftPlayer);

        if let &Event::Key(key, state) = event {

            if key == Key::W {
                match state {
                    ButtonState::Pressed => player_left_input.up = true,
                    ButtonState::Released => player_left_input.up = false,
                    _ => ()
                }
            }

            if key == Key::A {
                match state {
                    ButtonState::Pressed => player_left_input.left = true,
                    ButtonState::Released => player_left_input.left = false,
                    _ => ()
                }
            }

            if key == Key::D {
                match state {
                    ButtonState::Pressed => player_left_input.right = true,
                    ButtonState::Released => player_left_input.right = false,
                    _ => ()
                }
            }

            if key == Key::Up {
                match state {
                    ButtonState::Pressed => player_right_input.up = true,
                    ButtonState::Released => player_right_input.up = false,
                    _ => ()
                }
            }

            if key == Key::Left {
                match state {
                    ButtonState::Pressed => player_right_input.left = true,
                    ButtonState::Released => player_right_input.left = false,
                    _ => ()
                }
            }

            if key == Key::Right {
                match state {
                    ButtonState::Pressed => player_right_input.right = true,
                    ButtonState::Released => player_right_input.right = false,
                    _ => ()
                }
            }
            self.duel_match.get_world().set_player_input(LeftPlayer, player_left_input);
            self.duel_match.get_world().set_player_input(RightPlayer, player_right_input);
        }
        NoTransition
    }
}

impl RustyVollyState for LocalGameState {
    fn step(&mut self, game_assets: &mut GamesAssets) -> StateTransition {
        self.step(game_assets)
    }

    fn draw_window_content(&mut self, window: &mut Window, game_assets: &mut GamesAssets) -> Result<()> {
        self.draw_window_content(window, game_assets)
    }

    fn handle_event(&mut self, event: &Event, _window: &mut Window) -> StateTransition {
        self.handle_event(event, _window)
    }
}
