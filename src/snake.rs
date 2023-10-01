use crate::model::{AssetType, MoveDirection, Position, RenderableAsset};

#[derive(Clone,Debug)]
pub(crate) struct Snake {
    body: Vec<Position>,
    pub(crate) direction: MoveDirection,
}

impl Snake {
    pub(crate) fn new(initial_cell: Position) -> Self {
        Snake {
            body: vec![initial_cell],
            direction: MoveDirection::Right
        }
    }

    #[inline(always)]
    pub(crate) fn len(&self) -> usize {
        self.body.len()
    }

    #[inline(always)]
    pub(crate) fn cell_at_index(&self, index: usize) -> &Position {
        &self.body[index]
    }

    pub(crate) fn next_frame(&mut self) {
        let new_head = self.calculate_next_head();
        self.body.insert(0, new_head);
        self.body.pop();
    }

    fn calculate_next_head(&mut self) -> Position {
        let position = &self.body[0];
        match self.direction {
            MoveDirection::Right => Position { row: position.row, col: position.col + 1},
            MoveDirection::Left => Position { row: position.row, col: position.col - 1},
            MoveDirection::Up => Position { row: position.row - 1, col: position.col},
            MoveDirection::Down => Position { row: position.row + 1, col: position.col}
        }
    }

    pub(crate) fn grow(&mut self, count: usize) {
        let position = self.body.last().unwrap().clone();
        for _ in 0..count {
            self.body.push(Position { row: position.row, col: position.col});
        }
    }

    pub(crate) fn hit_itself(&self) -> bool {
        let head = &self.body[0];
        for body_part in &self.body[1..self.len()] {
            if head == body_part {
                return true;
            }
        }

        false
    }
    
    pub(crate) fn is_at_position(&self, position: &Position) -> bool {
        for body_part in self.body.iter() {
            if position == body_part {
                return true;
            }
        }

        false
    }
}


impl From<&Snake> for RenderableAsset {
    fn from(value: &Snake) -> Self {
        RenderableAsset {
            asset_type: AssetType::Player,
            positions: value.body.clone()
        }
    }
}
