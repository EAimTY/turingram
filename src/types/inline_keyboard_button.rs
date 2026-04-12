use super::InlineKeyboardButtonKind;
use serde::Serialize;

/// Button displayed in an inline keyboard row.
#[derive(Debug, Serialize)]
pub struct InlineKeyboardButton {
    /// Button label shown to the user.
    pub text: String,

    /// Button behavior.
    #[serde(flatten)]
    pub kind: InlineKeyboardButtonKind,
}
