// quiz2.rs
// This is a quiz for the following sections:
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums

// Let's build a little machine in the form of a function.
// As input, we're going to give a list of strings and commands. These commands
// determine what action is going to be applied to the string. It can either be:
// - Uppercase the string
// - Trim the string
// - Append "bar" to the string a specified amount of times
// The exact form of this will be:
// - The input is going to be a Vector of a 2-length tuple,
//   the first element is the string, the second one is the command.
// - The output element is going to be a Vector of strings.
// No hints this time!

// I AM DONE

pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    // TODO: Complete the function signature!
    pub fn transformer(input: Vec< (String, Command)>) -> Vec<String> {
        // TODO: Complete the output declaration!
        let mut output: Vec<String> = vec![];
        for (index, (string, command)) in input.iter().enumerate() {
            // TODO: Complete the function body. You can do it!
            // vec.push
            if let Command::Uppercase = command {
                output.push(string.to_string().to_ascii_uppercase());
            } else if let Command::Trim = command {
                output.push(string.trim().to_string());
            } else if let Command::Append(usize) = command {
                let mut bar_part = String::from("");
                let mut complete = String::from(string);
                
                for n in 0..*usize {
                    bar_part.push_str(&String::from("bar"));
                }
                
                complete.push_str(&bar_part);
                println!("complete part is : {}", &complete);

                output.push(complete.to_string());
            } else {
                output.push(string.to_string());
            }
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use crate::my_module::transformer;
    use super::Command;

    #[test]
    fn it_works() {
        let output = transformer(vec![
            ("hello".into(), Command::Uppercase),
            (" all roads lead to rome! ".into(), Command::Trim),
            ("foo".into(), Command::Append(1)),
            ("bar".into(), Command::Append(5)),
        ]);
        assert_eq!(output[0], "HELLO");
        assert_eq!(output[1], "all roads lead to rome!");
        assert_eq!(output[2], "foobar");
        assert_eq!(output[3], "barbarbarbarbarbar");
    }
}
