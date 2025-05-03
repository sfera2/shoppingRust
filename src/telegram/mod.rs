use std::error::Error;
use telegram_bot::*;


fn initialize() -> (Api, ChatId) {
    let token = "XXX";
    let api = Api::new(token);
    let chat = ChatId::new(11);
    (api, chat)
}

#[tokio::main]
async fn send_message(api: &Api, chat: &ChatId, message: String) -> Result<(), Box<dyn Error>> {
    let mut stream = api.stream();
    api.send(chat.text(message)).await?;
    Ok(())
}

pub fn send_single_message(message: String) -> Result<(), Box<dyn Error>> {
    let (api, chat) = initialize();
    send_message(&api, &chat, message)?;
    Ok(())
}

pub fn send_multiple_messages(messages: Vec<String>) -> Result<(), Box<dyn Error>> {
    let (api, chat) = initialize();
    for message in messages {
        send_message(&api, &chat, message)?;
    }
    Ok(())
}
