use game_logic::GameLogic;
use physic_world::PhysicWorld;
use global::PlayerSide::*;
use global::PlayerSide;

use vector::Vector2f;

pub struct DuelMatch {
    game_logic : GameLogic,
    physic_world : PhysicWorld,
}

#[derive(PartialEq, Eq)]
pub enum FrameEvent {
    EventBlobbyHit(PlayerSide),
    EventBallHitGround(PlayerSide),
    EventError(PlayerSide),
    EventWin(PlayerSide),
    EventReset,
}

impl DuelMatch {
    pub fn step(&mut self, events : &mut Vec<FrameEvent>) {
        self.physic_world.step();
        self.game_logic.step();

        let mut has_ball_hit_ground = false;

        if self.physic_world.ball_hit_left_player() {
            let valid_hit = self.game_logic.on_ball_hits_player(LeftPlayer);
            if valid_hit {
                events.push(FrameEvent::EventBlobbyHit(LeftPlayer));
            }
        }

        if self.physic_world.ball_hit_right_player() {
            let valid_hit = self.game_logic.on_ball_hits_player(RightPlayer);
            if valid_hit {
                events.push(FrameEvent::EventBlobbyHit(RightPlayer));   
            }
        }

        if self.physic_world.ball_hit_left_ground() {
            has_ball_hit_ground = true;
            self.game_logic.on_ball_hits_ground(LeftPlayer);
            events.push(FrameEvent::EventBallHitGround(LeftPlayer));    
        }

        if self.physic_world.ball_hit_right_ground() {
            has_ball_hit_ground = true;
            events.push(FrameEvent::EventBallHitGround(RightPlayer));
            self.game_logic.on_ball_hits_ground(RightPlayer);
        }

        let last_error = self.game_logic.get_last_error_side();

        match last_error {
            NoPlayer => (),
            _ => {
                if !has_ball_hit_ground {
                    self.physic_world.damp_ball();
                }

                events.push(FrameEvent::EventError(last_error));
                self.physic_world.set_ball_validity(false);
            },
        }

        if self.physic_world.is_round_finished() {
            events.push(FrameEvent::EventReset); 
            self.physic_world.reset(self.game_logic.get_serving_player());
        }

        let winning_player = self.game_logic.get_winning_player();

        match winning_player {
            NoPlayer => (),
            _ => {
                events.push(FrameEvent::EventWin(winning_player));
            },
        }

    }

    pub fn new() -> DuelMatch {
        let mut physic_world = PhysicWorld::new();

        physic_world.reset_player();
        physic_world.step();

        DuelMatch {
            physic_world : physic_world,
            game_logic: GameLogic::new(),
        }
    }

    pub fn get_world(&mut self) -> &mut PhysicWorld {
        &mut self.physic_world
    }

    pub fn get_serving_player(&self) -> PlayerSide {
        self.game_logic.get_serving_player()
    }

    pub fn get_ball_position(&self) -> Vector2f {
        self.physic_world.get_ball_position()
    }

    pub fn get_blob_position(&self, player: PlayerSide) -> Vector2f {
        if player == LeftPlayer
        {
		    return self.physic_world.get_blob(LeftPlayer);
        }
        else if player == RightPlayer
        {
            return self.physic_world.get_blob(RightPlayer);
        }
        else
        {
            return Vector2f::new(0.0f32, 0.0f32);
        }
    }

    pub fn get_scores(&self) -> (i32, i32) {
        self.game_logic.get_scores()
    }
}