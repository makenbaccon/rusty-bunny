
#[derive(Debug, PartialEq)]
pub struct Command <'original_string> {
    cmd: &'original_string str,
    rest: Option<Vec<&'original_string str>>,
}

impl Command <'_>{
    pub fn new<'original_string>(command_string: &'original_string str) ->  Option<Command> {
        let command_string = command_string.trim();
        if command_string.is_empty() { return None };
        let mut cmd_iter = command_string.split(' ').peekable();
        let cmd = cmd_iter.next();
        let rest = match cmd_iter.peek() {
            Some(_) => Some(cmd_iter.collect::<Vec<&str>>()),
            None => None,
        };

        let ret_val = match cmd {
            Some(cmd) => Some(Command{cmd: cmd, rest: rest}),
            None => None,
        };
        ret_val
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_empty() {
        let fake_command = "";
        assert_eq!(
            Command::new(fake_command),
            None
        );
    }

    #[test]
    fn test_new_command_only() {
        let fake_command = "tw";
        assert_eq!(
            Command::new(fake_command),
            Some(Command{cmd: "tw", rest: None})
        );
    }

    #[test]
    fn test_new_command_plus_one() {
        let fake_command = "tw @home";
        assert_eq!(
            Command::new(fake_command),
            Some(Command{cmd: "tw", rest: Some(vec!["@home"])})
        );
    }

    fn test_new_command_plus_two() {
        let fake_command = "tw @home now";
        assert_eq!(
            Command::new(fake_command),
            Some(Command{cmd: "tw", rest: Some(vec!["@home", "now"])})
        );
    }
}