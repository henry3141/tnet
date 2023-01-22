use std::{collections::HashMap, sync::{Mutex, Arc}};
use crate::any::Any; 
use crossbeam_channel::{Receiver, Sender, unbounded};
use std::fmt::Debug;

pub trait HandlerTrait:Send + Sync + Debug {
    fn handle(&mut self, event_data:Event, sender:Sender<Event>);
}

#[derive(Debug, Clone)]
pub struct Event {
    pub event_type: String,
    pub event_data: Any,
}

#[derive(Debug, Clone)]
pub struct InternalEventHandler {
    pub event_handler:Arc<Mutex<dyn HandlerTrait>>,
    pub events:Arc<Mutex<Vec<Event>>>,
}

impl InternalEventHandler {
    pub fn new(event_handler:Arc<Mutex<dyn HandlerTrait>>) -> Self {
        InternalEventHandler {
            event_handler,
            events:Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub async fn push(&self,event:Event) {
        self.events.lock().unwrap().push(event);
    }

    pub fn handle(&self, sender:Sender<Event>) {
        loop {
            let mut events = self.events.lock().unwrap();
            if events.len() > 0 {
                let event = events.remove(0);
                drop(events);
                self.event_handler.lock().unwrap().handle(event, sender.clone());
            } else {
                break;
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct EventManager {
    pub event_handlers:Arc<Mutex<HashMap<String, HashMap<i32,Arc<Mutex<InternalEventHandler>>>>>>,
    pub receiver:Receiver<Event>,
    pub sender:Sender<Event>,
}

impl EventManager{
    pub fn new() -> Self {
        let (sender, receiver) = unbounded();
        EventManager {
            event_handlers:Arc::new(Mutex::new(HashMap::new())),
            receiver,
            sender,
        }
    }  

    pub fn subscribe(&mut self, event_type:String, event_handler:Arc<Mutex<dyn HandlerTrait>>) -> i32 {
        let mut event_handlers = self.event_handlers.lock().unwrap();
        if event_handlers.contains_key(&event_type) {
            let event_handlers2 = event_handlers.get_mut(&event_type).unwrap();
            let mut id = 0;
            for (id2, _) in event_handlers2.iter() {
                if *id2 > id {
                    id = *id2;
                }
            }
            id += 1;
            event_handlers2.insert(id, Arc::new(Mutex::new(InternalEventHandler::new(event_handler))));
            id
        } else {
            let id = 0;
            let mut event_handlers2 = HashMap::new();
            event_handlers2.insert(id, Arc::new(Mutex::new(InternalEventHandler::new(event_handler))));
            event_handlers.insert(event_type, event_handlers2);
            0
        }
    }

    pub fn unsubscribe(&mut self, event_type:String, id:i32) {
        let mut event_handlers = self.event_handlers.lock().unwrap();
        if event_handlers.contains_key(&event_type) {
            let event_handlers2 = event_handlers.get_mut(&event_type).unwrap();
            event_handlers2.remove(&id);
        }
    }

    pub fn send(&self, event:Event) {
        self.sender.send(event).unwrap();
    }

    pub async fn handle(&mut self) {
        loop {
            let event = self.receiver.recv().unwrap();
            match (event.event_type == "manager",event.event_data.deref::<EventManagerControll>().is_some()) {
                (true,false) => {panic!("manager event must be EventManagerControll")},
                (true,true) => {
                    let event_manager_controll = event.event_data.deref::<EventManagerControll>().unwrap();
                    match event_manager_controll {
                        EventManagerControll::Subscribe(event_type, event_handler) => {
                            self.subscribe(event_type.clone(), event_handler.clone());
                        },
                        EventManagerControll::Unsubscribe(event_type, id) => {
                            self.unsubscribe(event_type.clone(), *id);
                        },
                        EventManagerControll::Send(event) => {
                            self.send(event.clone());
                        },
                    }
                },
                (false,false) => {},
                (false,true) => {panic!("event must be Event")},
            }
            let event_handlers = self.event_handlers.lock().unwrap();
            if event_handlers.contains_key(&event.event_type) {
                let event_handlers2 = event_handlers.get(&event.event_type).unwrap();
                for (_, event_handler) in event_handlers2.iter() {
                    event_handler.lock().unwrap().push(event.clone()).await;
                    let c = event_handler.clone();
                    let s = self.sender.clone();
                    tokio::spawn(async move {
                        c.lock().unwrap().handle(s);
                    });
                }
            }
        }
    }

}


pub enum EventManagerControll {
    Subscribe(String, Arc<Mutex<dyn HandlerTrait>>),
    Unsubscribe(String, i32),
    Send(Event),
}
