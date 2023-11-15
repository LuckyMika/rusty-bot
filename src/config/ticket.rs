use std::{path::Path, fs::{OpenOptions, File}, io::Read, sync::Arc};
use serenity::{model::{id::ChannelId, prelude::RoleId}, prelude::TypeMapKey};
use serde::{Deserialize, Serialize};

static FILE_NAME: &str = "ticket_config.json";

#[derive(Serialize, Deserialize)]
pub struct TicketConfig {
    pub category: ChannelId,
    pub admin_roles: Box<[RoleId]>,
    pub support_roles: Box<[RoleId]>,
}

impl TicketConfig {
    pub fn load() -> Self {
        if !Path::new(FILE_NAME).exists() {
            let default_config = Self { 
                category: ChannelId(0), admin_roles: Box::new([RoleId(0)]),
                support_roles: Box::new([RoleId(0)]) 
            };

            Self::create_config(&default_config).expect("Failed to create config file");

            return default_config;
        } else {
            let mut file = File::open(FILE_NAME).expect("Failed to open config file"); 
            let mut content = String::new();
            file.read_to_string(&mut content).expect("Failed to get config file contents");
            
            let config: Self = serde_json::from_str(&content).expect("Failed to parse config file");

            return config;
        }

    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        if !Path::new(FILE_NAME).exists() {
            Self::create_config(&self)?;

            return Ok(());
        }
        let file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(FILE_NAME)?;

        serde_json::to_writer_pretty(file, self)?;

        Ok(())
    }

    fn create_config(config: &Self) -> Result<(), Box<dyn std::error::Error>> {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(FILE_NAME)?;

        serde_json::to_writer_pretty(file, config)?;

        Ok(())
    }
}
