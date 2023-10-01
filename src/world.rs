use wasm_bindgen::prelude::wasm_bindgen;
use crate::model::{MoveDirection, Position, GameState, WorldStatus, WorldDimension};
use crate::reward::Reward;
use crate::snake::Snake;

#[wasm_bindgen]
pub struct World {
    dimension: WorldDimension,
    snake: Snake,
    reward: Reward,
    level: usize,
    state: GameState,
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

#[wasm_bindgen]
impl World {
    pub fn new() -> Self {
        World {
            dimension: WorldDimension {
                width: 21,
                height: 12,
            },
            snake: Snake::new(Position { row: 1, col: 1 }),
            reward: Reward::new(Position { row: 3, col: 3 }),
            level: 1,
            state: GameState::Play,
        }
    }

    pub fn get_dimension(&self) -> WorldDimension {
        self.dimension.clone()
    }

    pub fn get_status(&self) -> WorldStatus {
        WorldStatus {
            state: self.state,
            level: self.level,
            player: (&self.snake).into(),
            reward: (&self.reward).into(),
        }
    }

    pub fn next_frame(&mut self) -> WorldStatus {
        if self.state != GameState::Play {
            return self.get_status();
        }

        self.snake.next_frame();
        let snake_head = self.snake.cell_at_index(0);
        let snake_hit_reward = snake_head == self.reward.position();
        let snake_hit_edge = !self.dimension.contains(snake_head);

        if snake_hit_edge || self.snake.hit_itself() {
            self.state = GameState::Lost;

        } else if snake_hit_reward {
            self.snake.grow(self.level);
            let next_reward_place = self.find_place_for_reward();
            self.reward = Reward::new(next_reward_place);

            if self.level == 10 {
                self.state = GameState::Won;
            } else {
                self.level += 1;
            }
        }

        self.get_status()
    }

    pub fn face_right(&mut self) {
        if let MoveDirection::Left = self.snake.direction {
            return;
        }
        self.snake.direction = MoveDirection::Right;
    }

    pub fn face_left(&mut self) {
        if let MoveDirection::Right = self.snake.direction {
            return;
        }
        self.snake.direction = MoveDirection::Left;
    }

    pub fn face_up(&mut self) {
        if let MoveDirection::Down = self.snake.direction {
            return;
        }
        self.snake.direction = MoveDirection::Up;
    }

    pub fn face_down(&mut self) {
        if let MoveDirection::Up = self.snake.direction {
            return;
        }
        self.snake.direction = MoveDirection::Down;
    }

    fn find_place_for_reward(&self) -> Position {
        let mut place = self.dimension.random_place();
        while self.snake.is_at_position(&place) {
            place = self.dimension.random_place();
        }

        place
    }


    pub fn toggle_pause(&mut self) {
        match self.state {
            GameState::Play => self.state = GameState::Pause,
            GameState::Pause => self.state = GameState::Play,
            _ => ()
        }
    }
}
