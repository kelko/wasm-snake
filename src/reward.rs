use crate::model::{AssetType, Position, RenderableAsset};

#[derive(Clone,Debug)]
pub(crate) struct Reward(Position);

impl Reward {
    pub(crate) fn new(at: Position) -> Self{
        Self(at)
    }

    pub(crate) fn position(&self) -> &Position {
        &self.0
    }
}

impl From<&Reward> for RenderableAsset {
    fn from(value: &Reward) -> Self {
        RenderableAsset {
            asset_type: AssetType::Reward,
            positions: vec![value.0.clone()]
        }
    }
}
