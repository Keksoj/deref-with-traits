mod structs;
use structs::{Runner, Walker};
use moving::{Running, Walking, lib_module::Contest};
use std::ops::Deref;


fn main() {
    let old_joe = Walker {
        name: "Joe".to_string(),
    };
    old_joe.walk();

    let usain_bolt = Runner {
        walker_with_name: Walker {
            name: "Usain Bolt".to_string(),
        },
        sprint_time: 9.58,
    };

    usain_bolt.run();
    usain_bolt.walk();

    let contest = Contest {
        name : "Olympics".to_string(),
    };

    contest.welcome_a_walker(&old_joe);
    contest.welcome_a_walker(usain_bolt.deref());
    contest.start_a_race(&usain_bolt);
}


