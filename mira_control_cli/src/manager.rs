use crate::command::Command;

use mira_core::time_tracker::timer::Timer;

pub struct MiraMenagerCLI{
    timer: Timer,
}



impl MiraMenagerCLI{
    pub fn new() -> Self {
        MiraMenagerCLI{
            timer: Timer::new(),
        }
    }


    pub fn execute(&mut self, command: Command){
        match command{
            Command::Start => {
                self.timer.start();
            },
            Command::Pause => {
                self.timer.paused();
            }
            Command::Stop => {
                self.timer.stop();
            }
        }
    }
}