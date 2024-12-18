use crate::input::LoanInput;

pub struct LoanHistory {
    pub loan_input: LoanInput,
}

pub fn select_loan_to_edit(history: &Vec<LoanHistory>) -> Result<usize, String> {
    let options = history.iter().map(|h| h.loan_input.id).collect::<Vec<_>>();
    if options.is_empty() {
        return Err("No loans found".to_string());
    }
    let option = dialoguer::Select::new()
        .with_prompt("Which loan to edit?")
        .items(&options)
        .interact()
        .map_err(|e| format!("The application encountered an error reading the input: {e}"))?;
    Ok(option)
}
