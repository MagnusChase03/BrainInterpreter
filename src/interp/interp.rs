pub struct Interp<'a> {

    memory: Vec<u8>,
    pc: usize,
    program: &'a str

}

impl<'a> Interp<'a> {

    pub fn new(memory_size: usize, program: &'a str) -> Self {

        Interp{memory: vec![0; memory_size], pc: 0, program: program}

    }

    pub fn run(&mut self) -> Result<(), &'static str> {

        for character in self.program.chars() {

            match character {

                '<' => self.pc -= 1,
                '>' => self.pc += 1,
                '+' => self.memory[self.pc] += 1,
                '-' => self.memory[self.pc] -= 1,
                '.' => println!("{:?}", self.memory[self.pc] as char),
                _ => return Err("Invalid chacter"),

            }

        }

        Ok(())

    }

}