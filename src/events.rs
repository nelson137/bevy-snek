pub(crate) struct GameOverEvent(pub(crate) GameOverReason);

pub(crate) enum GameOverReason {
    SnakeHitWall,
    SnakeHitSelf,
}

#[derive(Default)]
pub(crate) struct GrowthEvent;
