use chrono::{Datelike, NaiveDate};
use rust_decimal::Decimal;

use crate::input::LoanInput;

struct LoanOutput {
    interest_with_margin: Decimal,
    interest_amount_accrued: Decimal,
    accrual_date: NaiveDate,
    elapsed_days: u64,
    total_interest: Decimal,
}

pub fn get_total_interest(loan_input: &LoanInput) -> Decimal {
    // principal * interest rate * number of years
    let loan_term_years = Decimal::from(loan_input.end_date.year() - loan_input.start_date.year());
    loan_input.amount * (loan_input.interest_rate + loan_input.margin) * loan_term_years
}
