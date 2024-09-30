#[derive(Debug, Clone)]
pub struct Expense {
    pub category: String,
    pub amount: f64,
}

impl Expense {
    pub fn new(category: String, amount: f64) -> Self {
        Self { category, amount }
    }
}