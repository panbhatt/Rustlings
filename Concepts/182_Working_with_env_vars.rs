// WOrking with arguments passed by user.
use std::env::{current_dir, current_exe, home_dir, vars};
use std::io;

// RUn this program from below command
// rustc 182_Working_with_args.rs  && ./182_Working_with_args capital hello there  -> Turn in uppercase.

fn main() {
    let env_vars = vars();
    for env_var in env_vars {
        println!("{:?}", env_var);
    }

    // This will print a list of all the env variables.

    // In order to get a specific one.
    println!("GOPATH -> {:?}", env!("GOPATH"));
    println!("USER -> {:?}", env!("USER")); // This one going to panic if it doesn't find.
                                            //println!("USR -> {:?}", env!("USR"));  // This one is not going to compile.

    println!("USER -> {:?}", option_env!("USERBLAHBLAH")); // Don't panic, this will show NONE.

    println!("{:?}", current_exe());
    println!("{:?}", current_dir());
    println!("{:?}", home_dir().unwrap()); // Will return an option.
}
