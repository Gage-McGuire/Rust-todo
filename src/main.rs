use std::io::stdin;

fn main() {
    println!("---------ToDo List---------");
    println!("***enter help for commands***");
    loop{
        let mut command = String::new();
        stdin().read_line(&mut command).expect("cannot read line");

        print!("command entered: {command}");
    }
}
