
pub fn get_validator_first_error_message(errors: &validator::ValidationErrors) -> String {
    errors.field_errors().values().next().unwrap().iter().next().unwrap().message.as_ref().unwrap().to_string()
}