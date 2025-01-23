#[derive(Debug, Clone)]
pub struct Plant {
    pub(crate) id: i32,
    pub(crate) name: String,
    pub(crate) humidity: f32,
    pub(crate) temperature: f32,
}

impl Plant {
    pub fn new(id: i32, name: String) -> Plant {
        Plant {
            id,
            name,
            humidity: 0.0,
            temperature: 0.0,
        }
    }
}
