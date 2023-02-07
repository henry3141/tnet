use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Routes {
    pub routes: HashMap<((i32, i32, i32),i32), RoutePoint>,
}

impl Routes {
    pub fn connect() -> Self {
        Routes {
            routes: HashMap::new(),
        }
    }

    pub fn get(&self, pos: (i32, i32, i32), time: i32) -> Option<RoutePoint> {
        self.routes.get(&(pos,time)).cloned()
    }

    pub fn add(&mut self, route: &Route) {
        for i in &route.points {
            self.routes.insert((i.pos,i.time), i.clone());
        }
    }
}

#[derive(Debug, Clone)]
pub struct RoutePoint {
    pub pos: (i32, i32, i32),
    pub time: i32,
}

#[derive(Debug, Clone)]
pub struct Route {
    pub start: (i32, i32, i32),
    pub end: (i32, i32, i32),
    pub points: Vec<RoutePoint>,
}

#[derive(Debug, Clone)]
pub struct RouteInterface {
    pub route: Route,
    pub index: usize,
}

impl RouteInterface {
    pub fn new(route: Route) -> Self {
        RouteInterface { route, index: 0 }
    }

    pub fn current(&self) -> Option<RoutePoint> {
        if self.index >= self.route.points.len() {
            return None;
        }
        Option::Some(self.route.points[self.index].clone())
    }

    pub fn prev(&mut self) -> Option<RoutePoint> {
        if self.index == 0 {
            return None;
        }
        self.index -= 1;
        Option::Some(self.route.points[self.index].clone())
    }

    pub fn reset(&mut self) {
        self.index = 0;
    }

    pub fn is_end(&self) -> bool {
        self.index >= self.route.points.len()
    }

    pub fn is_start(&self) -> bool {
        self.index == 0
    }

    pub fn get(&self) -> Option<RoutePoint> {
        if self.index >= self.route.points.len() {
            return None;
        }
        Option::Some(self.route.points[self.index].clone())
    }

    pub fn get_index(&self) -> usize {
        self.index
    }

    pub fn set_index(&mut self, index: usize) {
        self.index = index;
    }

    pub fn to_route(self) -> Route {
        self.route
    }
}

impl IntoIterator for Route {
    type Item = RoutePoint;
    type IntoIter = RouteInterface;

    fn into_iter(self) -> Self::IntoIter {
        RouteInterface::new(self)
    }
}

impl Iterator for RouteInterface {
    type Item = RoutePoint;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.route.points.len() {
            return None;
        }
        let point = self.route.points[self.index].clone();
        self.index += 1;
        Option::Some(point)
    }
}

pub struct RouteBuilder {
    pub start: (i32, i32, i32),
    pub end: (i32, i32, i32),
    pub points: Vec<RoutePoint>,
    pub routes: Routes,
    pub time: i32,
    pub reserve: i32,
}

impl RouteBuilder {
    pub fn new(routes: &Routes, c_time: i32) -> RouteBuilder {
        RouteBuilder {
            start: (0, 0, 0),
            end: (0, 0, 0),
            points: Vec::new(),
            routes:routes.clone(),
            time: c_time,
            reserve: 0,
        }
    }

    pub fn start(&mut self, start: (i32, i32, i32)) -> &mut Self {
        self.start = start;
        self
    }

    pub fn end(&mut self, end: (i32, i32, i32)) -> &mut Self {
        self.end = end;
        self
    }

    pub fn reserve(&mut self, reserve: i32) -> &mut Self {
        self.reserve = reserve;
        self
    }

    ///add up/down movement to route
    pub fn build(&mut self) -> Route {
        let mut current = self.start;
        let mut time = self.time;
        while current != self.end {
            let best = self.best(current);
            let point = RoutePoint { pos: best, time };
            self.points.push(point);
            current = best;
            time += 1;
        }
        self.points.push(RoutePoint {
            pos: self.end,
            time,
        });
        //deflect positions to avoid collisions
        let mut index = 0;
        let route = Route {
            start: self.start,
            end: self.end,
            points: self.points.clone(),
        };
        let mut route = route.into_iter();
        while !route.is_end() {
            let point = route.next().unwrap();
            if self.routes.get(point.pos, point.time).is_some() {
                index += 1;
                let route_index = {
                    let x = route.index as i32;
                    if x < 5 {
                        route.index = 0;
                        0
                    } else {
                        route.index = x as usize - 5;
                        x - 5
                    }
                };
                route = Self::delay(route.clone().to_route(), route_index,1).into_iter();
                if index > 2000 {
                    println!("WARN: RouteBuilder::create_route() - Route delayed by > 2000 ticks:breaking");
                    let mut new_route = route.clone().to_route();
                    new_route.points.truncate(route_index as usize);
                    route = new_route.into_iter();
                    break;
                }
            }
        }
        if index > 1000 {
            println!("WARN: RouteBuilder::create_route() - Route delayed by > 1000 ticks ({} ticks)", index);
        }
        if self.reserve > 0 {
            for i in 0..self.reserve {
                let point = RoutePoint {
                    pos: self.end,
                    time: time + i,
                };
                route.route.points.push(point.clone());
            }
        }
        route.to_route()
    }

    fn delay(route: Route, index: i32,delay:i32) -> Route {
        let mut new_route = route.clone();
        for i in index..new_route.points.len() as i32 {
            new_route.points[i as usize].time += delay;
        }
        new_route
    }

    fn dist(&self, s: (i32, i32, i32)) -> f32 {
        let x = self.end.0 - s.0;
        let y = self.end.1 - s.1;
        let z = self.end.2 - s.2;
        ((x * x + y * y + z * z) as f32).sqrt()
    }

    fn best(&self, c_pos: (i32, i32, i32)) -> (i32, i32, i32) {
        let mut best = None;
        let mut best_dist = self.dist(c_pos);
        for x in -1..2 {
            let change = (x, 0, 0);
            let pos = (c_pos.0 + change.0, c_pos.1 + change.1, c_pos.2 + change.2);
            let dist = self.dist(pos);
            if dist < best_dist {
                best = Option::Some(pos);
                best_dist = dist;
            }
        }

        for y in -1..2 {
            let change = (0, y, 0);
            let pos = (c_pos.0 + change.0, c_pos.1 + change.1, c_pos.2 + change.2);
            let dist = self.dist(pos);
            if dist < best_dist {
                best = Option::Some(pos);
                best_dist = dist;
            }
        }

        for z in -1..2 {
            let change = (0, 0, z);
            let pos = (c_pos.0 + change.0, c_pos.1 + change.1, c_pos.2 + change.2);
            let dist = self.dist(pos);
            if dist < best_dist {
                best = Option::Some(pos);
                best_dist = dist;
            }
        }
        if c_pos == self.end {
            return self.end;
        }
        best.unwrap()
    }
}
