use serde_json::{json, Value};
use crate::telegram::{Message, User, Chat};

pub fn get_response(message: Message) -> Value {
    let response = json!({
        "chat_id": message.get_chat_id(),
        "text": message.text,
        "parse_mode": "Markdown"
    });

    response
}
