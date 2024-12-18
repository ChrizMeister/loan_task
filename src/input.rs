use std::str::FromStr;

use chrono::NaiveDate;
use iso_currency::Currency;
use rust_decimal::Decimal;

pub struct LoanInput {
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub amount: Decimal,
    pub currency: Currency,
    pub interest_rate: Decimal,
    pub margin: Decimal,
}

pub fn get_loan_input() -> Result<LoanInput, String> {
    println!("Please enter the following details for the loan:");
    let start_date = get_start_date()?;
    let end_date = get_end_date(start_date)?;
    let amount = get_loan_amount()?;
    let currency = get_currency()?;
    let interest_rate = get_interest_rate()?;
    let margin = get_margin()?;
    Ok(LoanInput {
        start_date,
        end_date,
        amount,
        currency,
        interest_rate,
        margin,
    })
}

fn parse_input_into_type<T: FromStr>(input: &str) -> Result<T, T::Err> {
    input.trim().parse()
}

fn get_start_date() -> Result<NaiveDate, String> {
    loop {
        let mut start_date = String::new();
        println!("Start date (YYYY-MM-DD):");
        std::io::stdin()
            .read_line(&mut start_date)
            .map_err(|e| format!("The application encountered an error reading the input: {e}"))?;

        let start_date: NaiveDate = match parse_input_into_type(&start_date) {
            Ok(parsed_val) => parsed_val,
            Err(_) => {
                println!("* Please enter a valid date in the future");
                continue;
            }
        };

        if start_date < chrono::Utc::now().date_naive() {
            println!("* Start date cannot be in the past");
            continue;
        }

        return Ok(start_date);
    }
}

fn get_end_date(start_date: NaiveDate) -> Result<NaiveDate, String> {
    loop {
        let mut end_date = String::new();
        println!("End date (YYYY-MM-DD):");
        std::io::stdin()
            .read_line(&mut end_date)
            .map_err(|e| format!("The application encountered an error reading the input: {e}"))?;

        let end_date: NaiveDate = match parse_input_into_type(&end_date) {
            Ok(parsed_val) => parsed_val,
            Err(_) => {
                println!("* Please enter a valid date in the future");
                continue;
            }
        };

        if end_date <= start_date {
            println!("* The end date of the loan must be after the start date");
            continue;
        }

        return Ok(end_date);
    }
}

fn get_loan_amount() -> Result<Decimal, String> {
    loop {
        let mut loan_amount = String::new();
        println!("Loan amount:");
        std::io::stdin()
            .read_line(&mut loan_amount)
            .map_err(|e| format!("The application encountered an error reading the input: {e}"))?;

        let loan_amount: Decimal = match parse_input_into_type(&loan_amount) {
            Ok(parsed_val) => parsed_val,
            Err(_) => {
                println!("* Please enter a valid number");
                continue;
            }
        };

        return Ok(loan_amount);
    }
}

fn get_currency() -> Result<Currency, String> {
    loop {
        let mut currency = String::new();
        println!("Currency:");
        std::io::stdin()
            .read_line(&mut currency)
            .map_err(|e| format!("The application encountered an error reading the input: {e}"))?;

        match Currency::from_code(currency.trim()) {
            Some(currency) => return Ok(currency),
            None => {
                println!("* Please enter a valid currency");
                continue;
            }
        }
    }
}

fn get_interest_rate() -> Result<Decimal, String> {
    loop {
        let mut interest_rate = String::new();
        println!("Interest rate:");
        std::io::stdin()
            .read_line(&mut interest_rate)
            .map_err(|e| format!("The application encountered an error reading the input: {e}"))?;

        match parse_input_into_type(&interest_rate) {
            Ok(parsed_val) => return Ok(parsed_val),
            Err(_) => {
                println!("* Please enter a valid decimal number");
                continue;
            }
        };
    }
}

fn get_margin() -> Result<Decimal, String> {
    loop {
        let mut margin = String::new();
        println!("Margin:");
        std::io::stdin()
            .read_line(&mut margin)
            .map_err(|e| format!("The application encountered an error reading the input: {e}"))?;

        match parse_input_into_type(&margin) {
            Ok(parsed_val) => return Ok(parsed_val),
            Err(_) => {
                println!("* Please enter a valid decimal number");
                continue;
            }
        };
    }
}
