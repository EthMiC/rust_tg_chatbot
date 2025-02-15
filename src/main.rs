use minreq;
use serde_json::Value;
use tiny_http::{Request, Response};
use telegram::{Chat, User, Message};
use dotenv::dotenv;
use std::{env, sync::Arc};

mod bot;
mod telegram;

const PORT: i16 = 8080;

fn main() {
    //extract api_key
    dotenv().ok();

    //setup server listener
    let server = tiny_http::Server::http(format!("0.0.0.0:{}", PORT)).unwrap();

    //loop indefinately for incoming requests
    loop {
        //listen for requests
        let mut request: Request = match server.recv() {
            Ok(rq) => rq,
            Err(e) => {
                eprintln!("error: {}", e);
                break;
            }
        };

        //spawn thread and handle requests
        std::thread::spawn(move || 'thread: {
            //read request onto String
            let mut message_buffer = String::new();
            request
                .as_reader()
                .read_to_string(&mut message_buffer)
                .unwrap();

            //respond to incoming request
            let response = Response::empty(200);
            if let Err(e) = request.respond(response) {
                eprintln!("{}", e)
            }

            //convert message to Json Value
            let message_json: Value = match serde_json::from_str::<Value>(message_buffer.as_str()) {
                Ok(msg_b) => msg_b["message"].clone(),
                Err(_) => {
                    eprintln!("Message could not be parsed");
                    break 'thread;
                }
            };
            
            drop(message_buffer);

            //extract message, user and chat info
            let user: User = message_json["from"].clone().into();
            let chat: Chat = message_json["chat"].clone().into();
            let message: Message = match Message::new(message_json, &user, &chat) {
                Ok(msg) => msg,
                Err(e) => {
                    eprintln!{"{}", e};
                    break 'thread
                }
            };

            println! {"Bot has recieved a message:\n\"{}\"\n from the user:\n{}",
                &message.text,
                &message.get_user_first_name()
            };

            //Responde to the player
            let bot_message = bot::get_response(message);

            let api_token:Arc<String> = Arc::new(env::var("API_TOKEN").unwrap());

            match minreq::post(format!(
                "http://api.telegram.org/bot{}/sendMessage",
                api_token
            ))
                .with_header("Content-Type", "application/json")
                .with_body(bot_message.to_string())
                .send(){
                    Ok(response) => println!("Replied to {} in a {} chat with: {}",
                        &user.first_name,
                        &chat.chat_type,
                        match serde_json::from_str::<Value>(response.as_str().unwrap())
                            .unwrap()
                            .get("result")
                            .and_then(|r| r.get("text"))
                            .and_then(|text| text.as_str()) {
                                Some(text) => text,
                                None => "failed"
                            }
                    ),
                    Err(e) => eprintln!("Error sending Telegram message: {}", e),
            }
        });
    }
}
