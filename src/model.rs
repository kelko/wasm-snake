use rand::Rng;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(PartialEq, Clone, Debug)]
pub struct Position {
    pub row: i16,
    pub col: i16
}

#[derive(Clone,Debug)]
pub(crate) enum MoveDirection {
    Up,
    Down,
    Left,
    Right
}

#[wasm_bindgen]
#[derive(Clone,Copy, PartialEq, Debug)]
pub enum GameState {
    Play,
    Pause,
    Won,
    Lost
}

#[wasm_bindgen]
#[derive(Clone, PartialEq, Debug)]
pub struct WorldDimension {
    pub width: usize,
    pub height: usize,
}

impl WorldDimension {
    pub(crate) fn contains(&self, position: &Position) -> bool {
        position.col < self.width as i16 && position.row < self.height as i16
            && position.col >= 0 && position.row >= 0
    }

    pub(crate) fn random_place(&self) -> Position {
        let mut random = rand::thread_rng();
        let col = random.gen_range(0..self.width);
        let row = random.gen_range(0..self.height);

        Position { col: col as i16, row: row as i16 }
    }
}

#[wasm_bindgen]
#[derive(Clone,Copy, PartialEq, Debug)]
pub enum AssetType {
    Player,
    Reward
}

#[derive(Clone, PartialEq, Debug)]
pub struct RenderableAsset {
    pub(crate) asset_type: AssetType,
    pub(crate) positions: Vec<Position>
}

/**
 Readonly memento of current world status
*/
#[wasm_bindgen]
#[derive(Clone, PartialEq, Debug)]
pub struct WorldStatus {
    pub state: GameState,
    pub(crate) player: RenderableAsset,
    pub(crate) reward: RenderableAsset,
    pub level: usize,
}
