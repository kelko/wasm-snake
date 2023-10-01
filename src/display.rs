use std::ops::Index;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::CanvasRenderingContext2d;
use crate::model::{GameState, RenderableAsset, WorldDimension, WorldStatus};

#[wasm_bindgen]
pub struct Display {
    canvas: CanvasRenderingContext2d,
    cell_size: usize
}

#[wasm_bindgen]
impl Display {

    pub fn new(canvas: CanvasRenderingContext2d, cell_size: usize) -> Self {
        Self {
            canvas,
            cell_size,
        }
    }

    pub fn render(&self, dimension: &WorldDimension, world: &WorldStatus) {
        let canvas_width = (dimension.width * self.cell_size) as f64;
        let canvas_height = (dimension.width * self.cell_size) as f64;

        self.clear(canvas_width, canvas_height);
        self.render_background(world, canvas_width, canvas_height);

        let state = world.state;

        match state {
            GameState::Play => {
                self.render_snake(&world.player);
                self.render_reward(&world.reward)
            }
            GameState::Pause => {
                self.render_snake(&world.player);
                self.render_message(state, canvas_width, canvas_height);
            }
            GameState::Won | GameState::Lost => {
                self.render_message(state, canvas_width, canvas_height);

            }
        }
    }

    fn clear(&self, canvas_width: f64, canvas_height: f64) {
        self.canvas.clear_rect(0.0,0.0, canvas_width, canvas_height)
    }
    fn render_background(&self, world: &WorldStatus, canvas_width: f64, canvas_height: f64) {
        let background = "#BDCA00".into();
        self.canvas.set_fill_style(&background);
        self.canvas.fill_rect(0.0, 0.0, canvas_width, canvas_height);

        self.canvas.begin_path();

        let color = match world.state {
            GameState::Won =>"green".into(),
            GameState::Pause => "yellow".into(),
            GameState::Play => "#706200".into(),
            GameState::Lost => "red".into(),
        };

        self.canvas.set_stroke_style(&color);

        self.canvas.move_to(0.0, 0.0);
        self.canvas.line_to(0.0, canvas_height);
        self.canvas.line_to(canvas_width, canvas_height);
        self.canvas.line_to(canvas_width, 0.0);
        self.canvas.line_to(0.0, 0.0);

        self.canvas.stroke();
    }

    fn render_snake(&self, snake: &RenderableAsset) {
        let foreground = "#706200".into();
        self.canvas.set_fill_style(&foreground);
        let cell_as_f64 = self.cell_size as f64;

        for position in &snake.positions {

            self.canvas.begin_path();
            self.canvas.fill_rect(
                position.col as f64 * cell_as_f64,
                position.row as f64 * cell_as_f64,
                cell_as_f64,
                cell_as_f64
            );
            self.canvas.stroke()
        }
    }

    fn render_reward(&self, reward: &RenderableAsset) {
        let position = reward.positions.index(0);

        let font = format!("{}px serif", self.cell_size - 1);
        self.canvas.set_font(&font);
        _ = self.canvas.fill_text("🌸", (position.col * self.cell_size as i16 - 2) as f64, ((position.row + 1) * self.cell_size as i16 - 2) as f64);
    }

    fn render_message(&self, state: GameState, canvas_width: f64, canvas_height: f64) {
        let message = match state {
            GameState::Play => panic!(),
            GameState::Pause => "Pause",
            GameState::Won => "Congratulations",
            GameState::Lost => "You Lost!",
        };

        let color = match state {
            GameState::Play => panic!(),
            GameState::Pause => "yellow".into(),
            GameState::Won => "green".into(),
            GameState::Lost => "red".into()
        };

        self.canvas.set_font("32px sans-serif");
        self.canvas.set_fill_style(&color);
        self.canvas.set_text_align("center");
        _ = self.canvas.fill_text(message, canvas_width * 0.5,  canvas_height * 0.33);
    }
}
