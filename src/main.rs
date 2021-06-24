use std::io::stdin;

#[derive(Debug)]
enum VisitorAction {
    Accept,
    AcceptWithNote { note: String }, // enum contains data
    Refuse,
    Probation,
}

#[derive(Debug)] // enable debug placeholder `{:?}`
struct Visitor {
    name: String,
    action: VisitorAction,
    age: i8,
}

// define associate functions
impl Visitor {
    // constructor
    fn new(name: &str, action: VisitorAction, age: i8) -> Self {
        Self {
            name: name.to_lowercase(),
            action,
            age,
        }
    }

    // member function or method
    fn greet_visitor(&self) {
        match &self.action {
            VisitorAction::Accept => println!("Welcome to the treehouse, {}", &self.name),
            VisitorAction::AcceptWithNote { note } => {
                println!("Welcome to the treehouse, {}", self.name);
                println!("{}", note);
                if self.age < 21 {
                    println!("Do not serve alcohol to {}", self.name);
                }
            }
            VisitorAction::Probation => println!("{} is now a probationary member", self.name),
            VisitorAction::Refuse => println!("Do not allow {} in!", self.name),
        }
    }
}

fn what_is_your_name() -> String {
    let mut name = String::new();

    stdin().read_line(&mut name).expect("Failed to read line");

    name.trim().to_lowercase()
}

fn main() {
    let mut visitor_list: Vec<Visitor> = vec![
        Visitor::new(
            "aoi",
            VisitorAction::AcceptWithNote {
                note: String::from("Lactose-free milk is in the fridge"),
            },
            7,
        ),
        Visitor::new("kanade", VisitorAction::Accept, 4),
        Visitor::new("saki", VisitorAction::Accept, 0),
        Visitor::new("daigo", VisitorAction::Accept, 38),
        Visitor::new("fumie", VisitorAction::Accept, 37),
        Visitor::new("unknown", VisitorAction::Refuse, 100),
    ];

    loop {
        println!("Hello, what's your name?");

        let name = what_is_your_name();

        let known_visitor = visitor_list.iter().find(|visitor| visitor.name == name);

        match known_visitor {
            Some(visitor) => visitor.greet_visitor(),
            None => {
                if name.is_empty() {
                    break;
                } else {
                    println!("You are not on the visitor list.");
                    visitor_list.push(Visitor::new(&name, VisitorAction::Probation, 20))
                }
            }
        }
    }

    println!("The final list of visitors:");
    println!("{:#?}", visitor_list)
}
