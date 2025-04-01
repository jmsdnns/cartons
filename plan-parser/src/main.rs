use std::fs;
use winnow::Result;
use winnow::prelude::*;
use winnow::{
    ascii::{line_ending, space0, space1},
    combinator::{opt, repeat, terminated},
    token::take_while,
};

#[derive(Debug, PartialEq)]
enum Command {
    Execute {
        command: String,
    },
    Upload {
        local_path: String,
        remote_path: String,
    },
    Download {
        remote_path: String,
        local_path: String,
    },
}

fn parse_indented_field(input: &mut &str, field_name: &str) -> Result<String> {
    space1.parse_next(input)?;
    (field_name, ":").parse_next(input)?;
    space0.parse_next(input)?;
    let value = take_while(1.., |c| c != '\n').parse_next(input)?;

    Ok(value.to_string())
}

fn parse_execute(input: &mut &str) -> Result<Command> {
    "execute:".parse_next(input)?;
    space0.parse_next(input)?;
    let command = take_while(1.., |c| c != '\n').parse_next(input)?;

    Ok(Command::Execute {
        command: command.to_string(),
    })
}

fn parse_upload(input: &mut &str) -> Result<Command> {
    "upload:".parse_next(input)?;

    line_ending.parse_next(input)?;
    let local_path = parse_indented_field(input, "local_path")?;

    line_ending.parse_next(input)?;
    let remote_path = parse_indented_field(input, "remote_path")?;

    Ok(Command::Upload {
        local_path,
        remote_path,
    })
}

fn parse_download(input: &mut &str) -> Result<Command> {
    "download:".parse_next(input)?;

    line_ending.parse_next(input)?;
    let remote_path = parse_indented_field(input, "remote_path")?;

    line_ending.parse_next(input)?;
    let local_path = parse_indented_field(input, "local_path")?;

    Ok(Command::Download {
        remote_path,
        local_path,
    })
}

fn parse_command(input: &mut &str) -> Result<Command> {
    if input.len() >= 8 && &input.as_bytes()[0..8] == b"execute:" {
        parse_execute(input)
    } else if input.len() >= 7 && &input.as_bytes()[0..7] == b"upload:" {
        parse_upload(input)
    } else if input.len() >= 9 && &input.as_bytes()[0..9] == b"download:" {
        parse_download(input)
    } else {
        winnow::combinator::fail.parse_next(input)
    }
}

fn parse_commands(input: &mut &str) -> Result<Vec<Command>> {
    repeat(1.., terminated(parse_command, (space0, opt(line_ending)))).parse_next(input)
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string("commands.txt")?;

    let mut input = content.as_str();
    match parse_commands(&mut input) {
        Ok(commands) => {
            println!("Successfully parsed {} commands:", commands.len());

            commands.iter().enumerate().for_each(|(i, cmd)| match cmd {
                Command::Execute { command } => {
                    println!("  {}: Execute: {}", i + 1, command);
                }
                Command::Upload {
                    local_path: l,
                    remote_path: r,
                } => {
                    println!("  {}: Upload: local={}, remote={}", i + 1, l, r);
                }
                Command::Download {
                    remote_path: r,
                    local_path: l,
                } => {
                    println!("  {}: Download: remote={}, local={}", i + 1, r, l);
                }
            });
        }
        Err(e) => {
            println!("Error parsing commands: {}", e);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_single_execute_command() {
        let mut input = "execute: echo \"take five\"\n";
        let result = parse_execute(&mut input).unwrap();
        assert_eq!(
            result,
            Command::Execute {
                command: "echo \"take five\"".to_string()
            }
        );
    }

    #[test]
    fn test_parse_single_upload_command() {
        let mut input = "upload:\n  local_path: myscript.sh\n  remote_path: /tmp/myscript.sh\n";
        let result = parse_upload(&mut input).unwrap();
        assert_eq!(
            result,
            Command::Upload {
                local_path: "myscript.sh".to_string(),
                remote_path: "/tmp/myscript.sh".to_string()
            }
        );
    }

    #[test]
    fn test_parse_single_download_command() {
        let mut input =
            "download:\n  remote_path: /tmp/all-the-output.tar.gz\n  local_path: takefive.tar.gz\n";
        let result = parse_download(&mut input).unwrap();
        assert_eq!(
            result,
            Command::Download {
                remote_path: "/tmp/all-the-output.tar.gz".to_string(),
                local_path: "takefive.tar.gz".to_string()
            }
        );
    }

    #[test]
    fn test_parse_multiple_commands() {
        let mut input = r#"execute: some cmd bla bla
upload:
  local_path: file.txt
  remote_path: /tmp/file.txt
download:
  remote_path: /tmp/output.txt
  local_path: result.txt
"#;
        let result = parse_commands(&mut input).unwrap();
        assert_eq!(
            result,
            vec![
                Command::Execute {
                    command: "some cmd bla bla".to_string()
                },
                Command::Upload {
                    local_path: "file.txt".to_string(),
                    remote_path: "/tmp/file.txt".to_string()
                },
                Command::Download {
                    remote_path: "/tmp/output.txt".to_string(),
                    local_path: "result.txt".to_string()
                },
            ]
        );
    }

    #[test]
    fn test_parse_example_plan() {
        let mut input = r#"execute: echo "Take Five!"
upload:
  local_path: myscript.sh
  remote_path: /tmp/myscript.sh
download:
  remote_path: /tmp/all-the-output.tar.gz
  local_path: takefive.tar.gz
"#;
        let result = parse_commands(&mut input).unwrap();
        assert_eq!(
            result,
            vec![
                Command::Execute {
                    command: "echo \"Take Five!\"".to_string()
                },
                Command::Upload {
                    local_path: "myscript.sh".to_string(),
                    remote_path: "/tmp/myscript.sh".to_string()
                },
                Command::Download {
                    remote_path: "/tmp/all-the-output.tar.gz".to_string(),
                    local_path: "takefive.tar.gz".to_string()
                },
            ]
        );
    }
}
