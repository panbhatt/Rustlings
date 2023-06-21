// Implementation of FROM Trait .

// we are going to create an organization from a vec of Employees

#[derive(Debug)]
struct Employee {
    name: String,
    age: u8,
}

impl Employee {
    fn new(name: &str, age: u8) -> Self {
        Employee {
            name: name.to_string(),
            age,
        }
    }
}

#[derive(Debug)]
struct Organization {
    employees: Vec<Employee>,
}

impl From<Vec<Employee>> for Organization {
    // This is the implementation Trait FROM for ORgranziation
    fn from(emps: Vec<Employee>) -> Self {
        Self { employees: emps }
    }
}

impl Organization {
    fn print_employees(&self) {
        println!("===================PRINTING ALL EMPLOYEES==================");
        for emp in &self.employees {
            println!("{:?}", emp);
        }
    }
}

fn main() {
    let emp1 = Employee::new("Pankaj", 34);
    let emp2 = Employee::new("Joh7", 34);
    let emp3 = Employee::new("Jane", 28);

    let emp_vec = vec![emp1, emp2, emp3];

    let org = Organization::from(emp_vec);

    println!("Length of ORg EMployees -> {}", org.employees.len());

    org.print_employees();
}
