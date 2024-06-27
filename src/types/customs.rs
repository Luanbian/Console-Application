use core::fmt;

pub fn structs() {
    struct User {
        name: String,
        age: u8,
        active: bool,
    }
    impl User {
        fn to_string(&self) -> String {
            format!("Name: {}, Age: {}, Active: {}", self.name, self.age, self.active)
        }
    }

    let employer = User {
        name: String::from("John Doe"),
        age: 30,
        active: true,
    };

    println!("Employer: {:?}", employer.to_string());
}

pub fn enums() {
    enum DaysOfWeek {
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday,
    }

    impl fmt::Display for DaysOfWeek {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                DaysOfWeek::Monday => write!(f, "Monday"),
                DaysOfWeek::Tuesday => write!(f, "Tuesday"),
                DaysOfWeek::Wednesday => write!(f, "Wednesday"),
                DaysOfWeek::Thursday => write!(f, "Thursday"),
                DaysOfWeek::Friday => write!(f, "Friday"),
                DaysOfWeek::Saturday => write!(f, "Saturday"),
                DaysOfWeek::Sunday => write!(f, "Sunday"),
            }
        }
    }

    println!("Day: {}", DaysOfWeek::Monday);
    println!("Day: {}", DaysOfWeek::Tuesday);
    println!("Day: {}", DaysOfWeek::Wednesday);
    println!("Day: {}", DaysOfWeek::Thursday);
    println!("Day: {}", DaysOfWeek::Friday);
    println!("Day: {}", DaysOfWeek::Saturday);
    println!("Day: {}", DaysOfWeek::Sunday);
}