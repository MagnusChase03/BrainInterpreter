mod interp;

use interp::interp::Interp;

fn main() {

    let code = "+++++++++++++++++++++++++++++++++++++++++++++++++++++.++.>.++-.<.";
    let mut program = Interp::new(2048, code);
    match program.run() {

        Ok(()) => println!("Program exited successfully."),
        Err(str) => println!("{:?}", str),

    }

}
