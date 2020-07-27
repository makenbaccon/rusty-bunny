pub mod generic;
pub mod google;
pub mod twitter;
pub mod command;

use command::Command;

pub fn get_command_from_query_string(query_string: &str) -> Option<Command> {
    Command::new(query_string)
}

pub fn 

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_command_from_query_string_no_whitespace() {
        let actual = get_command_from_query_string("tw");
        let expected = "tw";
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_command_from_query_with_whitespace() {
        let actual = get_command_from_query_string("tw @fbOpenSource");
        let expected = "tw";
        assert_eq!(actual, expected);
    }
}