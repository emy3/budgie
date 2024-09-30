use crate::income::Income;
use crate::expense::Expense;

#[derive(Debug, Clone)]
pub struct Totals {
    pub total_income: f64,
    pub total_expense: f64,
}

impl Totals {
    pub fn calculate(incomes: &[Income], expenses: &[Expense]) -> Self {

        let total_income = incomes.iter().map(|income| income.amount).sum();
        let total_expense = expenses.iter().map(|expense: &Expense| expense.amount).sum();
        Self {
            total_income,
            total_expense,
        }
    }
}