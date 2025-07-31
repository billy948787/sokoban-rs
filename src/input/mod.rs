#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputEvent {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    Quit,
}
