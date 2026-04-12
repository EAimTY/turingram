use serde::Serialize;

/// Dice animation type for [`crate::methods::SendDice`].
#[derive(Debug, Serialize)]
pub enum DiceType {
    /// Standard dice animation.
    #[serde(rename = "🎲")]
    Dice,
    /// Dartboard animation.
    #[serde(rename = "🎯")]
    Dart,
    /// Slot machine animation.
    #[serde(rename = "🎰")]
    Slot,
}
