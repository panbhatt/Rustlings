use std::panic;

fn main() {
    panic::set_hook(Box::new(|msg| {
        println!("PANIC -> MSG {msg:#?}"); // This will print location file/line/col where it occurred, and also u can't panic in an panic hook
        //panic!("ERROR");   // This will abort the process. 
        println!{"{:?}", msg.payload()}
    }));

    panic!("This msg is not going to be visible on to the screen");
}


// PANIC Hooks used to safely shutdown the database process or other things in the application. 
