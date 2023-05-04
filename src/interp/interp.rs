use std::io;

pub struct Interp<'a> {

    // Memory, memory pointer, program counter, and the acutal program to run
    memory: Vec<u8>,
    ptr: usize,
    pc: usize,
    program: &'a str

}

impl<'a> Interp<'a> {

    // Create a new program interupreter
    // Takes in a memory size and the program code
    pub fn new(memory_size: usize, program: &'a str) -> Self {

        Interp{memory: vec![0; memory_size], ptr: 0, pc: 0, program: program}

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

    fn find_next(&mut self, character: char) -> Option<usize> {

        for index in self.pc..self.memory.len() {

            // Look at the nth command of the program
            match self.program.chars().nth(index) {

                // If this command is what we are looking fore return the current index
                Some(c) => {

                    if c == character {

                        return Some(index);

                    }

                },
                None => continue,

            }

        }

        return None;

    }

    fn find_before(&mut self, character: char) -> Option<usize> {

        for index in 0..self.pc {

            // Look at the nth command of the program
            match self.program.chars().nth(index) {

                // If this command is what we are looking fore return the current index
                Some(c) => {

                    if c == character {

                        return Some(index);

                    }

                },
                None => continue,

            }

        }

        return None;

    }

    fn start_loop(&mut self) -> Result<(), &'static str> {
        
        // If current memory is zero, go to end of loop else continue
        if self.memory[self.ptr] == 0 {

            match self.find_next(']') {

                Some(index) => self.pc = index,
                None => return Err("Cannot find end of loop"),

            }

        }

        Ok(())

    }

    fn end_loop(&mut self) -> Result<(), &'static str> {

        // If at end of loop go to begining
        match self.find_before('[') {

            Some(index) => self.pc = index,
            None => return Err("Cannot find beginning of loop"),

        }

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

            // Read user input
            ',' => {

                // Handle if there was an error with reading input
                let command_result = self.get_input();
                if command_result.is_err() {

                    return command_result;

                }
            
            },

            // Start of a loop
            '[' => {

                // Handle if there was an error with looping
                let command_result = self.start_loop();
                if command_result.is_err() {

                    return command_result;

                }


            },
            ']' => {

                // Handle if there was an error with looping
                let command_result = self.end_loop();
                if command_result.is_err() {

                    return command_result;

                }


            },
             _ => return Err("Invalid chacter"), // Not a known character return an error

        }

        Ok(()) // Command ran successfully

    }

    // Runs the program through the interupreter
    pub fn run(&mut self) -> Result<(), &'static str> {

        // Loop through the program until we reach the end of the program
        while self.pc < self.program.len() {

            // Execute the current command at program counter and return an error if any
            match self.program.chars().nth(self.pc) {

                Some(c) => {

                    // Run the command at existing program counter and handle error
                    let command_result = self.parse_command(c);
                    if command_result.is_err() {

                        return command_result;

                    }

                    if c != ']' {

                        self.pc += 1;

                    }

                },

                // Failed to read index, out of bounds program counter
                None => return Err("Error trying to read character at program counter")

            }

        }

        Ok(()) // Done running the code

    }

}