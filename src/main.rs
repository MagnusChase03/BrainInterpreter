mod interp;

use interp::interp::Interp;

fn main() {

    let code = "<.>+++++++++++++++++++++++++++++++++++++++++++++++++++++.++.>.++-.<.l";
    let mut program = Interp::new(2048, code);
    match program.run() {

        Ok(()) => println!("\n\nProgram exited successfully."),
        Err(str) => println!("\n\n{:?}", str),

    }

}
