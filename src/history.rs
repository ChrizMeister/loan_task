use chrono::NaiveDate;
use rust_decimal::Decimal;

use crate::input::LoanInput;

pub struct LoanHistory {
    pub loan_input: LoanInput,
    pub total_interest: Decimal,
    pub accrual_date: NaiveDate,
    pub elapsed_days: u64,
}
