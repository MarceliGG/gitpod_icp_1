use candid::CandidType;

#[derive(Clone, CandidType)]
pub struct Message {
    sender: String,
    content: String,
}

impl Message {
    pub fn new(sender: String, content: String) -> Self {
        Self { sender, content }
    }
}
