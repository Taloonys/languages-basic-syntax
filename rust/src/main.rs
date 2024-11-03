// Short comment 

/*

 */


fn main() {
    println!("Entry point");

    test();
}


trait Work {
    fn work(&self);

    fn die(&self) { println!("The end is the same for everyone"); }     // default implementation

    fn spawn_cheep(accuracy: f32) -> Self;                              // associative / static function

    fn wokr_n_report(&self) {
        self.work();                                                    // use Trait's method
        println!("Work is finished");
    }
}

struct Robot {
    accuracy    : f32,
    cpu_pow     : i32
}


impl Work for Robot {
    fn spawn_cheep(accuracy: f32) -> Self {
        Robot {
            accuracy, 
            cpu_pow: 5
        }
    }

    fn work(&self) { 
        println!("Work in process, chance to do it {acc}%", 
                    acc = &self.accuracy);
        println!(".. time left: {t} s", 
                    t   = 3600 / &self.cpu_pow);
    }
    
    // `die()` here is skipped
}


fn test() {
    let boby = Robot { 
        accuracy    : 99.9,
        cpu_pow     : 24
    };

    boby.work();
    boby.die();

    let dummy = Robot::spawn_cheep(50.5);
    dummy.wokr_n_report();
}