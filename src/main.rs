mod history;
mod input;
mod output;

use history::{select_loan_to_edit, LoanHistory};
use input::{
    cli_choices, get_accrual_date, get_console_input, get_loan_input, CliChoice, LoanInput,
};
use output::get_loan_output;

fn main() -> Result<(), String> {
    let mut history: Vec<LoanHistory> = vec![];
    loop {
        let choice = cli_choices()?;
        match choice {
            CliChoice::AddLoan => {
                let new_loan = add_new_loan(history.len())?;
                history.push(LoanHistory {
                    loan_input: new_loan,
                });
            }
            CliChoice::EditLoan => {
                let loan_index = match select_loan_to_edit(&history) {
                    Ok(loan_index) => loan_index,
                    Err(e) => {
                        println!("{}", e);
                        continue;
                    }
                };
                let curr_loan_input = &history[loan_index].loan_input;
                let updated_loan_input = get_loan_input(curr_loan_input.id, Some(curr_loan_input))?;
                history[loan_index].loan_input = updated_loan_input;
            }
            CliChoice::Exit => {
                break;
            }
        }
    }
    Ok(())
}

fn add_new_loan(idx: usize) -> Result<LoanInput, String> {
    let loan_input = get_loan_input(idx, None)?;

    let accrual_date = get_accrual_date()?.unwrap_or(loan_input.end_date);

    let loan_output = get_loan_output(&loan_input, accrual_date);
    println!(
        "After {} days ({}) you will pay {:.2} {} in total interest",
        loan_output.elapsed_days,
        loan_output.accrual_date,
        loan_output.total_interest,
        loan_input.currency.code()
    );
    println!(
        "Daily interest with margin: {:.2} {} ",
        loan_output.daily_interest_with_margin,
        loan_input.currency.code()
    );
    println!(
        "Daily interest without margin: {:.2} {}",
        loan_output.daily_interest_amount_accrued,
        loan_input.currency.code()
    );
    let _ = get_console_input("Enter to continue...")?;
    Ok(loan_input)
}
