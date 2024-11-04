//
// Traits is like a physical trait, but implements sth like interface of it
//

trait Work {                                                            // more like an interface for trait
    fn work(&self);

    fn die(&self) { println!("The end is the same for everyone"); }     // default implementation

    fn spawn_cheep(accuracy: f32) -> Self;                              // associative / static function

    fn wokr_n_report(&self) {
        self.work();                                                    // use Trait's method
        println!("Work is finished");
    }
}


struct Robot {                                                          // target
    accuracy    : f32,
    cpu_pow     : i32
}


impl Work for Robot {                                                   // implementation of trait for target
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


fn as_input_param(workable: &impl Work) {                               // input as any object, that has such trait
    workable.die();                                                     // call trait's stuff
}


fn working_robot(accuracy: f32, cpu_pow: i32) -> impl Work {            // returns an object, that must have `Work` trait impl
    Robot {accuracy, cpu_pow}
}


fn usage() {
    let boby = Robot { 
        accuracy    : 99.9,
        cpu_pow     : 24
    };

    boby.work();
    boby.die();

    as_input_param(boby);
    let _better_boby = working_robot(99.9, 50);                         // remember for rust it looks like a function

    let dummy = Robot::spawn_cheep(50.5);
    dummy.wokr_n_report();
}

