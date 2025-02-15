use serde_json::Value;

#[derive(Debug, Clone)]
pub struct User {
    id: i64,
    is_bot: bool,
    pub first_name: String
}

#[derive(Debug, Clone)]
pub struct Chat {
    id: i64,
    pub chat_type: String
}

#[derive(Debug, Clone)]
pub struct Message<'a> {
    id: i64,
    from: &'a User,
    chat: &'a Chat,
    date: i32,
    pub text: String
}

impl From<Value> for User {
    fn from(user: Value) -> Self {
        User {
            id: user["id"].as_i64().unwrap(),
            is_bot: user["is_bot"].as_bool().unwrap(),
            first_name: user["first_name"].as_str().unwrap().to_string()
        }
    }
}

impl From<Value> for Chat{
    fn from(chat: Value) -> Self {
        Chat {
            id: chat["id"].as_i64().unwrap(),
            chat_type: chat["type"].as_str().unwrap().to_string()
        }
    }
}

impl Message<'_> {
    pub fn new<'a>(message: Value, user: &'a User, chat: &'a Chat) -> Result<Message<'a>, String>{
        match message.get("message_id") {
            Some(_) =>
                Ok( Message {
                id: message["message_id"].as_i64().unwrap(),
                from: user,
                chat: chat,
                date: message["date"].as_i64().unwrap() as i32,
                text: message["text"].as_str().unwrap().to_string()
                }),
            None => Err("Could not find message".to_string())
        }
    }

    pub fn get_user_first_name(&self) -> &str{
        &self.from.first_name.as_str()
    }

    pub fn get_user_id(&self) -> &i64{
        &self.from.id
    }

    pub fn get_chat_id(&self) -> &i64{
        &self.chat.id
    }
}
