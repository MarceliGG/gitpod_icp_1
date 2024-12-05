use std::cell::RefCell;
mod message;
use message::Message;

thread_local! {
    static CHAT: RefCell<Vec<Message>> = RefCell::new(Vec::new());
}

#[ic_cdk::update]
fn save_msg(msg: String, username: String) -> Result<String, String> {
    if msg.len() > 2000 {
        return Err("message too long".to_string());
    }
    if username.len() > 100 {
        return Err("username too long".to_string());
    }
    CHAT.with(|static_msg| static_msg.borrow_mut().push(Message::new(username, msg)));
    Ok("success".to_string())
}

#[ic_cdk::query]
fn get_chat() -> Vec<Message> {
    CHAT.with(|static_msg| static_msg.borrow().clone())
}

#[ic_cdk::update]
fn clear_chat() {
    CHAT.with(|static_msg| *static_msg.borrow_mut() = Vec::new());
}
