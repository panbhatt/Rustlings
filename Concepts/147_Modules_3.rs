

pub mod country {

        fn print_country(country_name :&str)  {
                println!(" We are living in Country -> {}", country_name);
        }

        pub mod province {
                fn print_province(province_name :&str) {
                        println!(" in the PROVINCE of -> {} " , province_name);
                }

                pub mod city {
                        pub fn print_city(city_name :&str, province_name : &str, country_name : &str) {
                                crate::country::print_country(country_name); 
                                crate::country::province::print_province(province_name);
                                println!(" in the City OF -> {} " , city_name);
                        }

                        pub fn print_city_super(city_name :&str, province_name : &str, country_name : &str) {

                                use super::super::*; 
                                use super::*; 
                                print_country(country_name); 
                                print_province(province_name);
                                println!(" in the SUPER City OF -> {} " , city_name);
                        }
                }
        }

}

fn main() {
        crate::country::province::city::print_city("MIAMI", "FLORIDA", "USA");  // normal usage
        println!("============== SUPER ==========="); 
        use crate::country::province::city::*;   // shorten the usage 
        print_city_super("BOCA RATION", "FLORIDA", "AMERICA"); 
}