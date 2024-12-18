use chrono::NaiveDate;
use rust_decimal::Decimal;

use crate::input::LoanInput;

struct LoanOutput {
    daily_interest_with_margin: Decimal,
    daily_interest_amount_accrued: Decimal,
    accrual_date: NaiveDate,
    elapsed_days: u64,
    total_interest: Decimal,
}

pub fn get_interest_at_day(loan_input: &LoanInput, accrual_date: NaiveDate) -> Decimal {
    let days_since_start = Decimal::from(
        accrual_date
            .signed_duration_since(loan_input.start_date)
            .num_days(),
    );
    loan_input.amount
        * (loan_input.interest_rate + loan_input.margin)
        * (days_since_start / Decimal::from(365))
}

#[cfg(test)]
mod tests {
    use super::*;
    use iso_currency::Currency;
    use std::str::FromStr;

    #[test]
    fn test_get_interest_at_day() {
        let loan_input = LoanInput {
            amount: Decimal::from(1000),
            interest_rate: Decimal::from_str("0.05").unwrap(),
            margin: Decimal::from_str("0.01").unwrap(),
            start_date: NaiveDate::from_str("2025-01-01").expect("Failed to parse date"),
            end_date: NaiveDate::from_str("2026-01-01").expect("Failed to parse start date"),
            currency: Currency::USD,
        };

        let accrual_date = NaiveDate::from_str("2026-01-01").expect("Failed to parse date");
        let interest = get_interest_at_day(&loan_input, accrual_date);
        assert_eq!(interest, Decimal::from(60));

        let accrual_date = NaiveDate::from_str("2025-07-01").expect("Failed to parse date");
        let interest = get_interest_at_day(&loan_input, accrual_date);
        assert_eq!(interest.round(), Decimal::from(30));
    }
}
