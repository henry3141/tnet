use std::collections::HashSet;

pub struct DB {
    pub no_go:HashSet<(i32,i32,i32)>,
    pub blocked:HashSet<((i32,i32,i32),i32)>,
    pub tick:i32,
}

impl DB {
    pub fn new() -> DB {
        DB {
            no_go:HashSet::new(),
            blocked:HashSet::new(),
            tick:0,
        }
    }

    pub fn add_no_go(&mut self, x:i32, y:i32, z:i32) {
        self.no_go.insert((x,y,z));
    }

    pub fn add_blocked(&mut self, x:i32, y:i32, z:i32, time:i32) {
        self.blocked.insert(((x,y,z),time));
    }

    pub fn is_ok(&self, x:i32, y:i32, z:i32 , time:i32) -> bool {
        if self.no_go.contains(&(x,y,z)) {
            return false;
        }
        for (pos, t) in &self.blocked {
            if pos.0 == x && pos.1 == y && pos.2 == z && t == &time {
                return false;
            }
        }
        true
    }
}

