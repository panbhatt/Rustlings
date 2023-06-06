// Thi sis the 5th Video in the Let's get Rusty series that talks about STRUCTS.

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

impl User {
    // This will house the functions and methods associated with the Struct user.
    fn is_active(&self) -> bool {
        return self.active;
    }

    // Associated functions can be defined here or in another IMPL block
}

impl User {
    fn is_frequent_user(u: &User) -> bool {
        // Associated functions.
        if u.sign_in_count < 5 {
            return false;
        } else {
            return true;
        }
    }
}

fn build_user(username: String, email: String) -> User {
    return User {
        username,
        email,
        sign_in_count: 0,
        active: false,
    };
}

fn main() {
    let user1 = User {
        username: "panbhatt".to_string(),
        email: "panbhatt1@gmail.com".to_string(),
        sign_in_count: 1,
        active: false,
    };

    println!("User Name : -> {}", user1.username);

    let user2 = build_user("Rahul".to_string(), "RahulKutta@gmail.com".to_string());
    println!("User Name : -> {}", user2.username);

    let user3 = User {
        username: String::from("Namit"),
        ..user2 // i.e. use all the field of STRUCT User2
    };

    println!("user3 email is : {0} ", user3.email);
    println!("User3 is Active {} ", user3.is_active());
    println!(
        "USER3 is frequence user = {}",
        User::is_frequent_user(&user3)
    );

    println!("================================TUPLE STRUCT =================================");

    struct Color(u8, u8, u8); // TUPLE STRUCT.
    let col_1 = Color(255, 23, 23);
    println!("RED -> {}", col_1.0);
    println!("GREEN -> {}", col_1.1);
    println!("BLUE -> {}", col_1.2);
}
