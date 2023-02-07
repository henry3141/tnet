use routing::*;
use std::time::{Duration, SystemTime};

fn main() {
    let mut routes = Routes::connect();
    let mut route1 = RouteBuilder::new(&routes,1)
        .start((0, 0, 0))
        .end((10, 10, 10))
        .reserve(3000)
        .build();
    routes.add(&route1);
    for i in &route1.points {
        println!("route1:{:?}", i);
    }
    let mut route2 = RouteBuilder::new(&routes,1)
        .start((0, -1, 0))
        .end((10, 10, 10))
        .build();
    routes.add(&route2);
    for i in &route2.points {
        println!("route2:{:?}", i);
    }
    //check for collisions
    let mut route1 = route1.into_iter();
    let mut route2 = route2.into_iter();
    let mut time = 0;
    let mut collisions = 0;
    while !route1.is_end() && !route2.is_end() {
        let pos1 = route1.next().unwrap();
        let pos2 = route2.next().unwrap();
        if pos1.pos == pos2.pos {
            collisions += 1;
            println!("Collision at time: {} pos: {:?}=={:?}", time, pos1.pos,pos2.pos);
        }
        time += 1;
    }
    println!("Collisions: {}", collisions);
}
