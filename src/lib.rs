use std::error::Error;
pub trait Day: std::fmt::Debug {
    fn day_id(&self) -> u32;
    fn filename(&self) -> String;
    fn parse_input(&mut self, input: &str) -> Result<(), Box<dyn Error>>;
    fn part_one(&self) -> String;
    fn part_two(&self) -> String;
    fn run(&mut self) -> Result<(), Box<dyn Error>> {
        let input = std::fs::read_to_string(self.filename())?;
        let t1 = std::time::Instant::now();
        self.parse_input(&input)?;
        let t2 = std::time::Instant::now();
        let p1 = self.part_one();
        let t3 = std::time::Instant::now();
        let p2 = self.part_two();
        let t4 = std::time::Instant::now();
        let parse_time = t2 - t1;
        let p1_time = t3 - t2;
        let p2_time = t4 - t3;
        let total_time = t4 - t1;
        println!("Day {}: Took {:?}", self.day_id(), total_time);
        println!("\tParsing took {:?}", parse_time);
        println!("\tPart 1: {}", p1);
        println!("\t\took {:?}", p1_time);
        println!("\tPart 2: {}", p2);
        println!("\t\ttook {:?}", p2_time);
        Ok(())
    }
}

pub struct AdventOfCode {
    days: Vec<Box<dyn Day>>,
}

impl AdventOfCode {
    pub fn new() -> AdventOfCode {
        Self { days: vec![] }
    }
    pub fn add<T>(&mut self, day_fn: impl Fn() -> T)
    where
        T: Day + 'static,
    {
        self.days.push(Box::new(day_fn()));
    }
    pub fn run_all(&mut self) -> Result<(), Box<dyn Error>> {
        for day in self.days.iter_mut() {
            let _ = day.run().map_err(|e| {
                println!("Day {}: [ERROR]", day.day_id());
                println!("\t{}", e);
            });
        }
        Ok(())
    }
}

impl Default for AdventOfCode {
    fn default() -> Self {
        Self::new()
    }
}
