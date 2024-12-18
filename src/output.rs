use chrono::NaiveDate;
use rust_decimal::Decimal;

use crate::input::LoanInput;

pub struct LoanOutput {
    pub daily_interest_with_margin: Decimal,
    pub daily_interest_amount_accrued: Decimal,
    pub accrual_date: NaiveDate,
    pub elapsed_days: Decimal,
    pub total_interest: Decimal,
}

pub fn get_daily_interest_with_margin(loan_input: &LoanInput) -> Decimal {
    loan_input.amount * (loan_input.interest_rate + loan_input.margin) / Decimal::from(365)
}

pub fn get_daily_interest_amount_accrued(loan_input: &LoanInput) -> Decimal {
    loan_input.amount * loan_input.interest_rate / Decimal::from(365)
}

pub fn get_interest_at_day_with_margin(loan_input: &LoanInput, elapsed_days: Decimal) -> Decimal {
    loan_input.amount
        * (loan_input.interest_rate + loan_input.margin)
        * (elapsed_days / Decimal::from(365))
}

pub fn get_loan_output(loan_input: &LoanInput, accrual_date: NaiveDate) -> LoanOutput {
    let elapsed_days = Decimal::from(
        accrual_date
            .signed_duration_since(loan_input.start_date)
            .num_days(),
    );
    let interest = get_interest_at_day_with_margin(loan_input, elapsed_days);
    LoanOutput {
        daily_interest_with_margin: get_daily_interest_with_margin(loan_input),
        daily_interest_amount_accrued: get_daily_interest_amount_accrued(loan_input),
        accrual_date,
        elapsed_days,
        total_interest: interest,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use iso_currency::Currency;
    use std::str::FromStr;

    #[test]
    fn test_get_interest_at_day() {
        let loan_input = LoanInput {
            id: 0,
            amount: Decimal::from(1000),
            interest_rate: Decimal::from_str("0.05").unwrap(),
            margin: Decimal::from_str("0.01").unwrap(),
            start_date: NaiveDate::from_str("2025-01-01").expect("Failed to parse date"),
            end_date: NaiveDate::from_str("2026-01-01").expect("Failed to parse start date"),
            currency: Currency::USD,
        };

        let accrual_date = NaiveDate::from_str("2026-01-01").expect("Failed to parse date");
        let elapsed_days = accrual_date
            .signed_duration_since(loan_input.start_date)
            .num_days();
        let interest = get_interest_at_day_with_margin(&loan_input, elapsed_days.into());
        assert_eq!(interest, Decimal::from(60));

        let accrual_date = NaiveDate::from_str("2025-07-01").expect("Failed to parse date");
        let elapsed_days = accrual_date
            .signed_duration_since(loan_input.start_date)
            .num_days();
        let interest = get_interest_at_day_with_margin(&loan_input, elapsed_days.into());
        assert_eq!(interest.round(), Decimal::from(30));
    }
}
