// How to pass closures in functions.

struct Employee {
    name: String,
    age: u8,
    salary: f32,
}

impl Employee {
    fn print(&self) {
        println!(
            "Name = {}, age = {}, salary= {}",
            self.name, self.age, self.salary
        );
    }

    fn change_salary<F>(&mut self, mut salary_changer: F)
    where
        F: FnMut(&mut Self), // This func can be FnMut or Fn
    {
        
        salary_changer(self)
    }
}

fn main() {
    let mut employee = Employee {
        name: "Pankaj bhatt".to_string(),
        age: 38,
        salary: 2000.0,
    };

    let sal_changer = |emp_to_be_changed: &mut Employee| {
        emp_to_be_changed.salary += 100.0;
    };

    println!("================= BEFORE SALARY CHANGE ===============");
    employee.print();
    employee.change_salary(sal_changer);
    println!("=========== AFTER SALARY CHANGE via CLOSURE =================");
    employee.print();
}
