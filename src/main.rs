use std::io;

use menu::add_bill;

#[derive(Debug, Clone)]
pub struct Bill {
    name: String,
    amt: f32,
}

pub struct Bills {
    inner: Vec<Bill>,
}

enum Menu {
    AddBill,
    ViewBill,
}

impl Bills {
    fn new() -> Self {
        Self { inner: vec![] }
    }
    fn add(&mut self, bill: Bill) {
        self.inner.push(bill);
    }
    fn get_all(&self) -> Vec<&Bill> {
        self.inner.iter().collect()
    }
}

mod menu {
    use crate::{get_input, get_input_amt, Bill, Bills};

    pub fn add_bill(bills: &mut Bills) {
        println!("Bill Name: ");
        let name = match get_input() {
            Some(name) => name,
            None => return,
        };
        println!("Amount: ");
        let amount = match get_input_amt() {
            Some(amount) => amount,
            None => return,
        };
        let bill = Bill { name, amt: amount };
        bills.add(bill);
    }

    pub fn view_bills(bills: &Bills) {
        for bill in bills.get_all() {
            println!("{:?}", bill);
        }
    }
}

impl Menu {
    fn from_str(input: &str) -> Option<Menu> {
        match input {
            "1" => Some(Self::AddBill),
            "2" => Some(Self::ViewBill),
            _ => None,
        }
    }
    fn show_menu() {
        println!("");
        println!("--- Bill Manager ---");
        println!("1. Add Bill");
        println!("2. View Bill");
        println!("");
    }
}

fn get_input() -> Option<String> {
    let mut buffer = String::new();
    while io::stdin().read_line(&mut buffer).is_err() {
        print!("Please try again!")
    }
    let input = buffer.trim().to_owned();
    if &input == "" {
        None
    } else {
        Some(input)
    }
}
fn get_input_amt() -> Option<f32> {
    let mut buffer = String::new();
    while io::stdin().read_line(&mut buffer).is_err() {
        print!("Please try again!")
    }
    let input = buffer.trim().to_owned();
    if &input == "" {
        return None;
    }
    let pasrsed_input: Result<f32, _> = input.parse();
    match pasrsed_input {
        Ok(amt) => return Some(amt),
        Err(_) => {
            println!("Incorrect amount!");
            return None;
        }
    }
}

fn main() {
    let mut bills = Bills::new();
    loop {
        Menu::show_menu();
        println!("Enter Selection: ");
        let input = get_input().expect("No data entered!");
        match Menu::from_str(input.as_str()) {
            Some(Menu::AddBill) => menu::add_bill(&mut bills),
            Some(Menu::ViewBill) => menu::view_bills(&bills),
            None => return,
        }
    }
}
