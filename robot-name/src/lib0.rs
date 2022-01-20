use rand::Rng;
use std::cell::RefCell;
use std::collections::HashSet;

thread_local! {static NAMES: RefCell<HashSet<String>> = RefCell::new(HashSet::new())}

pub struct Robot {
    name: String,
}

impl Robot {
    pub fn new() -> Self {
        Self {
            name: Robot::generate_unique_name(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn reset_name(&mut self) {
        self.name = Robot::generate_unique_name();
    }

    fn generate_unique_name() -> String {
        NAMES.with(|c| {
            let mut names = c.borrow_mut();
            loop {
                let name = Robot::gen_rand_name();
                if !names.contains(&name) {
                    names.insert(name.clone());
                    return name;
                }
            }
        })
    }

    fn gen_rand_name() -> String {
        let mut rng = rand::thread_rng();
        format!(
            "{}{}{:03}",
            rng.gen_range('A'..='Z'),
            rng.gen_range('A'..='Z'),
            rng.gen_range(0..1000),
        )
    }
}

impl Drop for Robot {
    fn drop(&mut self) {
        NAMES.with(|names| names.borrow_mut().remove(self.name()));
    }
}
