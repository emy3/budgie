// add uses
use crate::income::Income;
use crate::expense::Expense;
use crate::totals::Totals;
use tauri::{command};

// store the income and expenses
static mut INCOMES: Vec<Income> = Vec::new();
static mut EXPENSE: Vec<Expense> = Vec::new();

// command to send data to frontend
// income
#[command]
pub fn add_income(description: String, amount: f64) -> String {
    let new_income = Income::new(description, amount);
    unsafe {
        INCOMES.push(new_income.clone());
    }
    format!("Added income: {:?}", new_income)
}

// expense
#[command]
pub fn add_expense(category: String, amount: f64) -> String {
    let new_expense = Expense::new(category, amount);
    unsafe {
        EXPENSE.push(new_expense.clone());
    }
    format!("Added expense: {:?}", new_expense)
}

// total
#[command]
pub fn get_totals() -> String {
    unsafe { 
        let totals = Totals::calculate(&INCOMES, &EXPENSE);
        format!("Total Income: {}, Total Expense: {}", totals.total_income, totals.total_expense)
    }
}