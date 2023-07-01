pub fn handle_health() {
    println!("Handle Health Route ");
}

pub fn handle_model_function() {
    println!("Handle Model function  ");

    crate::models::user_model::print_user_model();
}
