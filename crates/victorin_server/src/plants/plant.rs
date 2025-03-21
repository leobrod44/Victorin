use crate::config::config::PlantConfig;

#[derive(Debug, Clone)]
pub struct Plant {
    pub(crate) valve_id: u32,
    pub(crate) name: String,
    pub(crate) humidity: f32,
    pub(crate) temperature: f32,
}

impl From<&PlantConfig> for Plant {
    fn from(config: &PlantConfig) -> Self {
        Plant {
            valve_id: config.id,
            name: config.name.clone(),
            humidity: 0.0,
            temperature: 0.0,
        }
    }
}

impl Plant {
    pub fn new(id: u32, name: String) -> Plant {
        Plant {
            valve_id: id,
            name,
            humidity: 0.0,
            temperature: 0.0,
        }
    }
}
