mod income;
mod expense;
mod totals;
mod command;

use tauri::Builder;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            command::add_income, command::add_expense, command::get_totals])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
