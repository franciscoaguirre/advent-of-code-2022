pub enum Command {
    Cd(CdArgument),
    Ls,
}

use Command::*;

type DirectoryName = String;

pub enum CdArgument {
    Root,
    In(DirectoryName),
    Out,
}

use CdArgument::*;

impl Command {
    pub fn from(line: &str) -> Self {
        if !line.starts_with("$") {
            panic!("Tried to parse something that is not a command!");
        }

        let line = &line[2..]; // Removing '$' and space character

        if line.starts_with("cd") {
            let line = &line[3..]; // Removing 'cd' and space character

            match line {
                "/" => Cd(Root),
                ".." => Cd(Out),
                directory => Cd(In(directory.to_string())),
            }
        } else if line.starts_with("ls") {
            Ls
        } else {
            panic!("Not a valid command!");
        }
    }

    pub fn execute(&self, current_directory: &str) -> String {
        match self {
            Cd(argument) => match argument {
                Root => "/".to_string(),
                In(directory_name) => format!("{current_directory}{directory_name}/"),
                Out => {
                    let last_slash_index = current_directory[..current_directory.len() - 1]
                        .rfind('/')
                        .expect("No slashes, can't go back!");
                    current_directory[..last_slash_index + 1].to_string()
                }
            },
            Ls => current_directory.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn commands_work() {
        let commands = [
            Cd(Root),
            Cd(In("a".to_string())),
            Cd(In("b".to_string())),
            Ls,
            Cd(Out),
        ];

        let mut current_directory = String::new();
        for command in commands {
            current_directory = command.execute(&current_directory);
        }

        assert_eq!(current_directory, "/a/".to_string());
    }
}
