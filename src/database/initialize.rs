// initialize.rs

fn start() {
    println!("no issues creating database... starting database");
}

fn create() {
    println!("create database and tables")
}

pub fn run(name: &str) {
    println!("called database::initialize::run({})", &name);
    println!("This function should check if a database exists first.
             If none exists, create it, otherwise start it.");

}
