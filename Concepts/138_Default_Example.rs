// Default pattern
#[derive(Debug)]
enum WeekDay {
    SUNDAY,
    MONDAY,
    TUESDAY,
    WEDNESDAY,
    THURSDAY,
    FRIDAY,
    SATURDAY,
}

#[derive(Debug)]
struct Employee {
    name: String,
    birth_day: WeekDay,
}

impl Employee {
    fn new(name: String, isWeekend: bool) -> Employee {
        // New is the normal way to use ::new() to declare a new employee.
        Self {
            name,
            birth_day: if isWeekend {
                WeekDay::SUNDAY
            } else {
                WeekDay::MONDAY
            },
        }
    }
}

// IMPL DEFAULT for Employee
impl Default for Employee {
    fn default() -> Self {
        Self {
            name: "KUPUTRA GUPTA".into(),
            birth_day: WeekDay::SUNDAY,
        }
    }
}

fn main() {
    let my_i8: i8 = Default::default();
    let my_u32: u32 = Default::default();
    let my_str: &str = Default::default();

    println!("I8 Default = {my_i8} , U32 Default = {my_u32} &str Default = {my_str}");

    let employee = Employee::new("Pankaj Bhatt".into(), true);
    println!("Employee {:?}", employee);

    println!("=============== USING DEFAULT =========================");
    let kup_gupta = Employee::default();
    println!("DEFAULT KUPUTRA -> {:?}", kup_gupta);
}
