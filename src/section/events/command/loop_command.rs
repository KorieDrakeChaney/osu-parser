#[derive(Debug)]
pub struct LoopCommand {
    start_time: i32,
    loop_count: i32,
}

impl std::fmt::Display for LoopCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "L,{},{}\n", self.start_time, self.loop_count)
    }
}

impl LoopCommand {
    pub fn parse(s: &[&str]) -> std::io::Result<Self> {
        let start_time = s[0].parse().unwrap();
        let loop_count = s[1].parse().unwrap();
        Ok(LoopCommand {
            start_time,
            loop_count,
        })
    }
}
