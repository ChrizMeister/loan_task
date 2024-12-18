use chrono::Datelike;

mod input;
mod output;

use input::get_loan_input;
use output::get_total_interest;

fn main() -> Result<(), String> {
    let loan_input = get_loan_input()?;

    let total_interest = get_total_interest(&loan_input);
    let loan_duration_years = loan_input.end_date.year() - loan_input.start_date.year();
    println!(
        "After {} years you will pay {:.2} {} in total interest",
        loan_duration_years,
        total_interest,
        loan_input.currency.code()
    );
    // todo: per day breakdown of the accrued interest

    // todo: output results to console and optionally csv
    Ok(())
}
