use chrono::Datelike;

mod history;
mod input;
mod output;

use history::LoanHistory;
use input::{get_accrual_date, get_loan_input};
use output::get_interest_at_day;

fn main() -> Result<(), String> {
    let mut history: Vec<LoanHistory> = vec![];

    let loan_input = get_loan_input()?;

    let accrual_date = get_accrual_date()?.unwrap_or(loan_input.end_date);

    let interest_at_day = get_interest_at_day(&loan_input, accrual_date);
    let loan_duration_years = loan_input.end_date.year() - loan_input.start_date.year();
    println!(
        "After {} days you will pay {:.2} {} in total interest",
        loan_duration_years,
        interest_at_day,
        loan_input.currency.code()
    );
    // todo: per day breakdown of the accrued interest

    // todo: output results to console and optionally csv
    Ok(())
}
