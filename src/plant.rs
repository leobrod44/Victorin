use chrono::Duration;

pub(crate) struct Plant{
    id: i32,
    name: String,
    humidity: f32,
    temperature: f32,
    
}

impl Plant {
    pub(crate) fn new(id: i32, name: String) -> Plant {
        Plant {
            id,
            name,
            humidity: 0.0,
            temperature: 0.0
        }
    }
}