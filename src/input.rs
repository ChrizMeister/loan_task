use std::str::FromStr;

use chrono::NaiveDate;
use iso_currency::Currency;
use rust_decimal::Decimal;

pub struct LoanInput {
    pub id: usize,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub amount: Decimal,
    pub currency: Currency,
    pub interest_rate: Decimal,
    pub margin: Decimal,
}

pub enum CliChoice {
    AddLoan,
    EditLoan,
    Exit,
}

impl From<usize> for CliChoice {
    fn from(value: usize) -> Self {
        match value {
            0 => CliChoice::AddLoan,
            1 => CliChoice::EditLoan,
            2 => CliChoice::Exit,
            _ => panic!("Invalid CLI choice"),
        }
    }
}

pub fn cli_choices() -> Result<CliChoice, String> {
    let options = ["Add loan", "Edit loan", "Exit"];
    let option = dialoguer::Select::new()
        .with_prompt("Please select an option:")
        .items(&options)
        .interact()
        .map_err(|e| format!("The application encountered an error reading the input: {e}"))?;
    Ok(CliChoice::from(option))
}

pub fn get_loan_input(
    id: usize,
    existing_loan_input: Option<&LoanInput>,
) -> Result<LoanInput, String> {
    if existing_loan_input.is_some() {
        println!(
            "Please enter the following details for the loan (leave blank to use existing values):"
        );
    } else {
        println!("Please enter the following details for the loan:");
    }

    // Start date
    let mut start_date_prompt = "Start date (YYYY-MM-DD)".to_string();
    if let Some(existing_loan_input) = existing_loan_input {
        start_date_prompt = format!(
            "{} (default: {})",
            start_date_prompt, existing_loan_input.start_date
        );
    }
    let mut start_date_str = get_console_input(&start_date_prompt)?;
    let start_date = loop {
        if start_date_str == "\n" {
            if let Some(existing_loan_input) = existing_loan_input {
                break existing_loan_input.start_date;
            }
        }
        match parse_date(&start_date_str) {
            Ok(start_date) => break start_date,
            Err(e) => {
                println!("{}", e);
                start_date_str = get_console_input(&start_date_prompt)?;
            }
        }
    };

    // End date
    let mut end_date_prompt = "End date (YYYY-MM-DD)".to_string();
    if let Some(existing_loan_input) = existing_loan_input {
        end_date_prompt = format!(
            "{} (default: {})",
            end_date_prompt, existing_loan_input.end_date
        );
    }
    let mut end_date_str = get_console_input(&end_date_prompt)?;
    let end_date = loop {
        if end_date_str == "\n" {
            if let Some(existing_loan_input) = existing_loan_input {
                break existing_loan_input.end_date;
            }
        }
        match parse_end_date(&end_date_str, start_date) {
            Ok(end_date) => break end_date,
            Err(e) => {
                println!("{}", e);
                end_date_str = get_console_input(&end_date_prompt)?;
            }
        }
    };

    // Amount
    let mut amount_prompt = "Loan amount (XXXX.XX)".to_string();
    if let Some(existing_loan_input) = existing_loan_input {
        amount_prompt = format!(
            "{} (default: {})",
            amount_prompt, existing_loan_input.amount
        );
    }
    let mut amount_str = get_console_input(&amount_prompt)?;
    let amount = loop {
        if amount_str == "\n" {
            if let Some(existing_loan_input) = existing_loan_input {
                break existing_loan_input.amount;
            }
        }
        match parse_decimal(&amount_str) {
            Ok(amount) => break amount,
            Err(e) => {
                println!("{}", e);
                amount_str = get_console_input(&amount_prompt)?;
            }
        }
    };

    // Currency
    let mut currency_prompt = "Currency (USD, EUR, GBP, etc.)".to_string();
    if let Some(existing_loan_input) = existing_loan_input {
        currency_prompt = format!(
            "{} (default: {})",
            currency_prompt,
            existing_loan_input.currency.code()
        );
    }
    let mut currency_str = get_console_input(&currency_prompt)?;
    let currency = loop {
        if currency_str == "\n" {
            if let Some(existing_loan_input) = existing_loan_input {
                break existing_loan_input.currency;
            }
        }
        match parse_currency(&currency_str) {
            Ok(currency) => break currency,
            Err(e) => {
                println!("{}", e);
                currency_str = get_console_input(&currency_prompt)?;
            }
        }
    };

    // Interest rate
    let mut interest_rate_prompt = "Interest rate (XX.XX)%".to_string();
    if let Some(existing_loan_input) = existing_loan_input {
        interest_rate_prompt = format!(
            "{} (default: {:.2})",
            interest_rate_prompt,
            existing_loan_input.interest_rate * Decimal::from(100)
        );
    }
    let mut interest_rate_str = get_console_input(&interest_rate_prompt)?;
    let interest_rate = loop {
        if interest_rate_str == "\n" {
            if let Some(existing_loan_input) = existing_loan_input {
                break existing_loan_input.interest_rate;
            }
        }
        match parse_decimal(&interest_rate_str) {
            Ok(interest_rate) => break interest_rate / Decimal::from(100),
            Err(e) => {
                println!("{}", e);
                interest_rate_str = get_console_input(&interest_rate_prompt)?;
            }
        }
    };

    // Margin
    let mut margin_prompt = "Margin (XX.XX)%".to_string();
    if let Some(existing_loan_input) = existing_loan_input {
        margin_prompt = format!(
            "{} (default: {:.2})",
            margin_prompt,
            existing_loan_input.margin * Decimal::from(100)
        );
    }
    let mut margin_str = get_console_input(&margin_prompt)?;
    let margin = loop {
        if margin_str == "\n" {
            if let Some(existing_loan_input) = existing_loan_input {
                break existing_loan_input.margin;
            }
        }
        match parse_decimal(&margin_str) {
            Ok(margin) => break margin / Decimal::from(100),
            Err(e) => {
                println!("{}", e);
                margin_str = get_console_input(&margin_prompt)?;
            }
        }
    };
    Ok(LoanInput {
        id,
        start_date,
        end_date,
        amount,
        currency,
        interest_rate,
        margin,
    })
}

pub fn get_accrual_date() -> Result<Option<NaiveDate>, String> {
    let prompt = "Accrual date (YYYY-MM-DD. Leave empty for end of loan):";
    let accrual_date_str = get_console_input(prompt)?;
    if accrual_date_str == "\n" {
        return Ok(None);
    }
    Ok(Some(parse_date(&accrual_date_str)?))
}

fn parse_input_into_type<T: FromStr>(input: &str) -> Result<T, T::Err> {
    input.trim().parse()
}

pub fn get_console_input(prompt: &str) -> Result<String, String> {
    let mut input = String::new();
    println!("{}", prompt);
    std::io::stdin()
        .read_line(&mut input)
        .map_err(|e| format!("The application encountered an error reading the input: {e}"))?;
    Ok(input)
}

pub fn parse_date(date: &str) -> Result<NaiveDate, String> {
    let date: NaiveDate = match parse_input_into_type(&date) {
        Ok(parsed_val) => parsed_val,
        Err(_) => {
            return Err("* Please enter a valid date in the future".to_string());
        }
    };

    if date < chrono::Utc::now().date_naive() {
        return Err("* Start date cannot be in the past".to_string());
    }

    Ok(date)
}

fn parse_end_date(end_date: &str, start_date: NaiveDate) -> Result<NaiveDate, String> {
    let end_date: NaiveDate = match parse_input_into_type(&end_date) {
        Ok(parsed_val) => parsed_val,
        Err(_) => {
            return Err("* Please enter a valid date in the future".to_string());
        }
    };

    if end_date <= start_date {
        return Err("* The end date of the loan must be after the start date".to_string());
    }

    Ok(end_date)
}

fn parse_decimal(decimal: &str) -> Result<Decimal, String> {
    let decimal_value: Decimal = match parse_input_into_type(&decimal) {
        Ok(parsed_val) => parsed_val,
        Err(_) => {
            return Err("* Please enter a valid number".to_string());
        }
    };
    Ok(decimal_value)
}

fn parse_currency(currency: &str) -> Result<Currency, String> {
    match Currency::from_code(currency.trim()) {
        Some(currency) => return Ok(currency),
        None => {
            return Err("* Please enter a valid currency".to_string());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_start_date_valid() {
        let start_date = parse_date("2025-01-01").expect("Failed to parse start date");
        assert!(start_date > chrono::Utc::now().date_naive());
    }

    #[test]
    fn test_parse_start_date_past() {
        let result = parse_date("2024-01-01");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_start_date_invalid_date() {
        let result = parse_date("202-01");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_end_date_valid() {
        let start_date = NaiveDate::from_str("2025-01-01").expect("Failed to parse start date");
        let end_date = parse_end_date("2026-01-01", start_date).expect("Failed to parse end date");
        assert!(end_date > start_date);
    }

    #[test]
    fn test_parse_end_date_invalid() {
        let start_date = NaiveDate::from_str("2025-01-01").expect("Failed to parse start date");
        let result = parse_end_date("2024-01-01", start_date);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_decimal_valid() {
        let decimal = parse_decimal("1000.00").expect("Failed to parse decimal");
        assert_eq!(decimal, Decimal::from(1000));
    }

    #[test]
    fn test_parse_decimal_invalid() {
        let result = parse_decimal("1a00.00b");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_currency_valid() {
        let currency = parse_currency("USD").expect("Failed to parse currency");
        assert_eq!(currency, Currency::USD);
    }

    #[test]
    fn test_parse_currency_invalid() {
        let result = parse_currency("invalid currency");
        assert!(result.is_err());
    }
}
