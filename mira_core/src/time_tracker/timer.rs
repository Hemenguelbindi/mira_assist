use std::time::{Instant, Duration};


/// This is struct timer
pub struct Timer{
    start_time: Option<Instant>,
    paused_duration: Duration,
    is_paused: bool
}

impl Timer{

    /// Creates  a new timer instance.
    pub fn new()->Self{
        Timer{
            start_time: None,
            paused_duration: Duration::new(0, 0),
            is_paused: false,
        }
    }
    
    /// Starts the Timer.
    pub fn start(&mut self){
        if self.is_paused{
            println!("Таймер возобновлен!");
            self.start_time = Some(Instant::now() - self.paused_duration);
        } else if self.start_time.is_none(){
            println!("Запускаю таймер!");
            self.start_time = Some(Instant::now());
        } else { 
            println!("Таймер уже запущен")
        }

    }

    /// Paused the Timer.
    pub fn paused(&mut self){
        if let Some(start) = self.start_time{
            self.paused_duration = Instant::now().duration_since(start);
            self.is_paused = true;
        } else {
            println!("Таймер не был запущен.")
        }

    }


    /// Stops the Timer.
    pub fn stop(&mut self){
        if let Some(start) = self.start_time{
            let total_duration = Instant::now().duration_since(start) + self.paused_duration;
            println!("Таймер остановлен! Время прошло {:?}", total_duration);
            self.start_time = None;
            self.paused_duration = Duration::new(0, 0);
            self.is_paused = false;
        } else {
            println!("Таймер не запущен!");
        }
    }
    
    

    ///Retursn the elapsed time between start and stop
    pub fn elapsed(&self) -> Option<Duration> {
        if let Some(start) = self.start_time{
            Some(Instant::now().duration_since(start) + self.paused_duration)
        } else {
            None
        }
    }

}



#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn it_works() {
        let timer = Timer::new();

        assert!(timer.start_time.is_none(), "Ожидаем None в start_time");
        assert!(timer.paused_duration.is_zero(), "Ожидаем нулевые значение в paused_duratuion");
        assert!(!timer.is_paused, "Таймер не должен быть на паузе после создания");
    }

    #[test]
    fn test_timer_start(){
        let mut timer = Timer::new();

        timer.start();

        assert!(timer.start_time.is_some(), "start_time должен быть установлен");
        assert!(!timer.is_paused, "Таймер не долже быть на паузе.");
    }


    #[test]
    fn test_pause_timer(){
        let mut timer = Timer::new();
        
        timer.start();
        std::thread::sleep(std::time::Duration::from_millis(100));
        timer.paused();
        

        assert!(timer.paused_duration >= std::time::Duration::from_millis(100),"paused_duration должен быть не меньше 100ms");
    }

}