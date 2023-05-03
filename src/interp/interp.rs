pub struct Interp<'a> {

    // Memory, memory pointer, and the acutal program to run
    memory: Vec<u8>,
    ptr: usize,
    program: &'a str

}

impl<'a> Interp<'a> {

    // Create a new program interupreter
    // Takes in a memory size and the program code
    pub fn new(memory_size: usize, program: &'a str) -> Self {

        Interp{memory: vec![0; memory_size], ptr: 0, program: program}

    }

    // Runs the program through the interupreter
    pub fn run(&mut self) -> Result<(), &'static str> {

        // Go through every character in the code and execute its meaning
        for character in self.program.chars() {

            match character {

                '<' => {

                    // Wrap to end
                    if self.ptr == 0 {

                        self.ptr = self.memory.len() - 1;

                    } else {

                        self.ptr -= 1;

                    }

                }, // Go left in memory
                '>' => {

                    // Wrap to begining
                    if self.ptr == self.memory.len() - 1{

                        self.ptr = 0;

                    } else {

                        self.ptr += 1;

                    }

                }, // Go right in memory
                '+' => self.memory[self.ptr] += 1, // Increment memory
                '-' => {

                    // Make sure memory does not go negative
                    if self.memory[self.ptr] != 0 {

                        self.memory[self.ptr] -= 1

                    } else {

                        return Err("Tried to decrement memory to a negative value");

                    }

                }, // Decrement memory
                '.' => print!("{:?}", self.memory[self.ptr] as char), // Print memory
                _ => return Err("Invalid chacter"), // Not a known character return an error

            }

        }

        Ok(()) // Done running the code

    }

}