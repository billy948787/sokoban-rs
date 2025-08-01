#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputEvent {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    Undo,
    Redo,
    Restart,
    Quit,
}
