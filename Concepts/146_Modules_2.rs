pub mod organization {

    #[derive(Debug)]
    pub struct Company {
        name: String,
        employees: Vec<Employee>,
    }

    #[derive(Debug)]
    pub struct Employee {
        pub name: String,
        pub age: u8,
    }

    impl Company {
        pub fn new(name: String, employees: Vec<Employee>) -> Self {
            Self { name, employees }
        }

        pub fn add_employee(&mut self, employee: Employee) {
            self.employees.push(employee);
        }

        pub fn print_players(self) {
            println!("Players :{:?}", self.employees)
        }
    }
}

fn main() {
    use crate::organization::*;

    let james_harden = Employee {
        name: "James Harden".into(),
        age: 32,
    };

    let joel_embid = Employee {
        name: "Joel Embid".into(),
        age: 28,
    };

    let players = vec![james_harden, joel_embid];

    let mut nba = Company::new("NBA".into(), players);
    let pascal_siacam = Employee {
        name: "Pascal Siakam".into(),
        age: 27,
    };

    nba.add_employee(pascal_siacam);

    nba.print_players();
}
