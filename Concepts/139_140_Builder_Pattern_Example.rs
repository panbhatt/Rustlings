// Builder Pattern.

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
    age: u8,
    birth_day: WeekDay,
    can_use: bool, // will turn true, only if you call build function. 
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
            age: 0,
            can_use: true,
        }
    }

    // BUILDER PATTERN.
    fn name(mut self, name: String) -> Self {
        self.name = name;
        self
    }

    fn age(mut self, age: u8) -> Self {
        self.age = age;
        self
    }

    // This function is end of the builder pattern that verifies all the input parameters before creating the object.
    fn build(mut self) -> Result<Self, String> {
        // This will chk the can_use variable to define
        if self.age > 40 {
            self.can_use = false;
            Err("Your Age can't be less then 40".into())
        } else {
            self.can_use = true;
            Ok(self)
        }
    }
}

// IMPL DEFAULT for Employee
impl Default for Employee {
    fn default() -> Self {
        Self {
            name: "KUPUTRA GUPTA".into(),
            birth_day: WeekDay::SUNDAY,
            age: 0,
            can_use: true,
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

    println!("=============== USING BUILDER PATTERn =========================");
    let john_abhram = Employee::default().name("JOHN ABHRAHAM".into()).age(30);
    println!("JOHN -> {:?}", john_abhram);

    // The is with builder method that would give us the error when we do make build ()
    let hrithink_roshan_result = Employee::default()
        .name("JHRITHINK ROSHAN".into())
        .age(45)
        .build();
    match hrithink_roshan_result {
        Ok(hr) => println!("HR -> {:?}", hr),
        Err(err) => println!("ERROR ->> {}", err),
    };
    // println!("H ROSHAN -> {:?}", hrithink_roshan);
}
