pub struct Route {
    pub id:i32,
    pub end:(i32,i32,i32),
    pub start:(i32,i32,i32),
    pub path:Vec<(i32,i32,i32)>,
    pub time:i32,
    pub cost:i32,
}

impl Route {
    pub fn build() -> Self {
        Route {
            id: 0,
            end: (0,0,0),
            start: (0,0,0),
            path: Vec::new(),
            time: 0,
            cost: 0,
        }
    }

    pub fn start(&mut self, start:(i32,i32,i32)) -> &mut Self {
        self.start = start;
        self
    }

    pub fn end(&mut self, end:(i32,i32,i32)) -> &mut Self {
        self.end = end;
        self
    }

    pub fn finish(&mut self) -> &mut Self {
        self
    }
}