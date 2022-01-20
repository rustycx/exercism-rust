pub struct Robot {
    name: String,
}

static mut X: u8 = b'A';
static mut Y: u8 = b'A';
static mut N: u32 = 0;

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
        unsafe {
            N += 1;
            if N == 1000 {
                N = 0;
                Y += 1;
                if Y == b'Z' + 1 {
                    Y = b'A';
                    X += 1;
                }
            }
            format!("{}{}{:03}", X as char, Y as char, N)
        }
    }
}
