#[derive(Debug, Clone)]
pub struct Income {
    pub description: String,
    pub amount: f64,
}

impl Income {
    pub fn new(description: String, amount: f64) -> Self {
        Self { description, amount }
    }
}