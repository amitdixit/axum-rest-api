#[derive(Clone, Debug)]
pub struct Ctx {
    user_id: u32,
}

impl Ctx {
    pub fn new(user_id: u32) -> Self {
        Self { user_id }
    }
}

impl Ctx {
    pub fn user_id(&self) -> u32 {
        self.user_id
    }
}
