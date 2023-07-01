// See this for Module explanation https://www.sheshbabu.com/posts/rust-module-system/

pub mod config; // this means we are using the file config.rs as a module; PUb mean swe are exporting it too.
pub mod configuration; // using a module via a folder. and mod.rs file
mod routes;
mod models; 

fn main() {
    println!("================= SINGLE FILE BASED MODULE ================\n");
    crate::config::parse_config(); // Using the top function at the root level . // self can be used to refer to the same create.
    self::config::parse_config();

    crate::config::yaml::parseYaml(); // Using submodule.

    use self::config::yaml; // when we refer it via use to bring it in scope.
    yaml::parseYaml();
    println!("================= FOLDER MODULE ================\n");
    configuration::parse_config();
    configuration::yaml::parseYaml();

    println!("==================== ROUTE Folder usage ================\n");

    self::routes::health_routes::handle_health();
    
    self::routes::transit_routes::handle_transit();

    self::routes::health_routes::handle_model_function();

    println!("============================ USING EXTERNAL RAND PACKAGE ====================="); 

    println!("Random No -> {}", rand::random::<i32>());
    use rand::random; 
    println!("Random No DIrect usage -> {}", random::<u8>());



}
