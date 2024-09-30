mod income;
mod expense;
mod totals;

// add uses
use income::Income;
use expense::Expense;
use totals::Totals;
use tauri::{command, Builder};

// store the income and expenses
static mut INCOMES: Vec

// command to send data to frontend
// income
#[command]
// expense
#[command]
// total
#[command]

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            add_income, add_expense, get_totals])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
