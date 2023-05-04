use std::io;

pub struct Interp<'a> {

    // Memory, memory pointer, and the acutal program to run
    memory: Vec<u8>,
    ptr: usize, // Change to a Box for infinite memory sizes
    program: &'a str

}

impl<'a> Interp<'a> {

    // Create a new program interupreter
    // Takes in a memory size and the program code
    pub fn new(memory_size: usize, program: &'a str) -> Self {

        Interp{memory: vec![0; memory_size], ptr: 0, program: program}

    }

    fn move_memory_left(&mut self) {

        // Wrap to end
        if self.ptr == 0 {

            self.ptr = self.memory.len() - 1;

        } else {

            self.ptr -= 1;

        }

    }

    fn move_memory_right(&mut self) {

        // Wrap to begining
        if self.ptr == self.memory.len() - 1{

            self.ptr = 0;

        } else {

            self.ptr += 1;

        }

    }

    fn increment_memory(&mut self) {

        self.memory[self.ptr] += 1

    }

    fn decrement_memory(&mut self) -> Result<(), &'static str> {

        // Make sure memory does not go negative
        if self.memory[self.ptr] != 0 {

            self.memory[self.ptr] -= 1

        } else {

            return Err("Tried to decrement memory to a negative value");

        }

        Ok(())

    }

    fn get_input(&mut self) -> Result<(), &'static str> {

        let mut buffer = String::new();
        let stdin = io::stdin();

        // Read from stdin and store the character in the memory location
        match stdin.read_line(&mut buffer) {

            Ok(_) => {

                // Get the first character in the buffer and write into memory
                match buffer.chars().nth(0) {

                    Some(c) => self.memory[self.ptr] = c as u8,
                    None => return Err("No character read into the buffer from stdin"),

                }

            },
            Err(_) => return Err("Error reading from stdin"),


        };

        Ok(())

    }

    fn parse_command(&mut self, character: char) -> Result<(), &'static str> {

        match character {

            '<' => self.move_memory_left(), // Go left in memory
            '>' => self.move_memory_right(), // Go right in memory
            '+' => self.increment_memory(), // Increment memory

            // Decrement memory
            '-' => {

                // Handle if there was an error with decrementing memory
                let command_result = self.decrement_memory();
                if command_result.is_err() {

                    return command_result;

                }

            },

            '.' => print!("{:?}", self.memory[self.ptr] as char), // Print memory
            ',' => {

                let command_result = self.get_input();
                if command_result.is_err() {

                    return command_result;

                }
            
            }, // Read user input
             _ => return Err("Invalid chacter"), // Not a known character return an error

        }

        Ok(()) // Command ran successfully

    }

    // Runs the program through the interupreter
    pub fn run(&mut self) -> Result<(), &'static str> {

        // Go through every character in the code and execute its meaning
        for character in self.program.chars() {

             // Execute the current command and return an error if any
            let command_result = self.parse_command(character);
            if command_result.is_err() {

                return command_result;

            }

        }

        Ok(()) // Done running the code

    }

}