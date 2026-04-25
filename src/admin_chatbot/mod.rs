pub mod core_logic;

#[derive(Default)]
pub struct AdminChatbot {
    pub core_logic: core_logic::CoreLogic,
}

impl AdminChatbot {
    pub fn new() -> Self {
        AdminChatbot {
            core_logic: core_logic::CoreLogic::new(),
        }
    }
}
