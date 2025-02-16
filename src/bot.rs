use minreq;
use serde_json::{json, Value};
use std::env;

use crate::telegram::Message;

pub fn get_response(message: &Message) -> Value {
    //get response from AI
    let api_token = env::var("AI_API_TOKEN").unwrap();

    let ai_response: Value = match minreq::post("https://openrouter.ai/api/v1/chat/completions")
        .with_header("Authorization", format!("Bearer {}", api_token))
        .with_body(
            json!({
                "model": "deepseek/deepseek-r1:free",
                "messages": [
                    {
                        "role": "user",
                        "content": "wake up"
                    },
                    {
                        "role": "assistant",
                        "content": "Greetings"
                    },
                    {
                        "role": "user",
                        "content": format!("{}", message.text)
                    },
                ],
                "response_format": {
                    "type": "json_schema",
                    "json_schema": {
                        "name": "Name of model used",
                        "strict": true,
                        "schema": {
                            "messages": {
                                "role": "String",
                                "text": "The response of the assistant without the thought process"
                            }
                        },
                    },
                },
                "temperature": 0.5,
            })
            .to_string(),
        )
        .send()
    {
        Ok(res) => serde_json::from_str(res.as_str().unwrap()).unwrap(),
        Err(_) => json!({}),
    };

    //handle and format response
    let ai_message = ai_response["choices"]
        .as_array()
        .unwrap()[0]
        ["message"]
        .get("content")
        .unwrap();

    let response = json!({
        "chat_id": message.get_chat_id(),
        "text": ai_message,
        "parse_mode": "Markdown"
    });

    response
}
