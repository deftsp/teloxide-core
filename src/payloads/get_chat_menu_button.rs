// This file is auto generated by [`cg`] from [`schema`].
//
// **DO NOT EDIT THIS FILE**,
//
// Edit `cg` or `schema` instead.
//
// [cg]: https://github.com/teloxide/cg
// [`schema`]: https://github.com/WaffleLapkin/tg-methods-schema
use serde::Serialize;

use crate::types::{ChatId, MenuButton};

impl_payload! {
    /// Use this method to get the current value of the bot's menu button in a private chat, or the default menu button.
    #[derive(Debug, PartialEq, Eq, Hash, Default, Clone, Serialize)]
    pub GetChatMenuButton (GetChatMenuButtonSetters) => MenuButton {

        optional {
            /// Unique identifier for the target private chat. If not specified, default bot's menu button will be returned
            pub chat_id: ChatId [into],
        }
    }
}