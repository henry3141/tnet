use std::{sync::{Arc, Mutex}, thread, time::Duration};

#[derive(Debug, Clone,)]
pub struct Promise<T>
where T:Clone
{
    internal:Arc<Mutex<Option<T>>>,
}

impl<T> Promise<T> 
where T:Clone
{
    pub fn new() -> (Self,Self) {
        let p = Promise {
            internal:Arc::new(Mutex::new(None)),
        };
        (p.clone(),p)
    }

    pub fn set(&self, value:T) {
        *self.internal.lock().unwrap() = Some(value);
    }

    pub fn get(&self) -> Option<T> {
        self.internal.lock().unwrap().clone()
    }

    pub fn wait(&self) -> T {
        loop {
            if let Some(value) = self.get() {
                return value;
            }
            thread::sleep(Duration::from_millis(1));
        }
    }
}