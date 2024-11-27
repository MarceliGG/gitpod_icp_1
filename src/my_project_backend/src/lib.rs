use std::cell::RefCell;

#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

thread_local! {
    static CHAT: RefCell<Vec<String>> = RefCell::new(Vec::new());
}

#[ic_cdk::update]
fn save_msg(msg: String) {
    CHAT.with(|static_msg| static_msg.borrow_mut().push(msg));
}

#[ic_cdk::query]
fn get_chat() -> Vec<String> {
    CHAT.with(|static_msg| static_msg.borrow().clone())
}

#[ic_cdk::update]
fn clear_chat() {
    CHAT.with(|static_msg| *static_msg.borrow_mut() = Vec::new());
}
