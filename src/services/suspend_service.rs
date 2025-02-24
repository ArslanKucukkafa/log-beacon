use std::fs;
use crate::models::config_model::Config;
use crate::models::shell_model::{ObjectType, SuspendSubcommand};
use crate::services::configuration_service::{save_config, load_config};

pub fn add_suspension(
    object_type: ObjectType,
    object_name: &str
) -> Result<(), String> {
    let mut config = load_config()?;

    match object_type {
        ObjectType::CLASS => {
            config.suspend.classes.push(object_name.to_string());
        }
        ObjectType::SERVICE => {
            config.suspend.services.push(object_name.to_string());
        }
        _ => {}
    }

    save_config(config)
}

pub fn remove_suspension(
    object_type: ObjectType,
    object_name: &str
) -> Result<(), String> {
    let mut config = load_config()?;

    match object_type {
        ObjectType::CLASS => {
            config.suspend.classes.retain(|x| x != object_name);
        }
        ObjectType::SERVICE => {
            config.suspend.services.retain(|x| x != object_name);
        }
        _ => {}
    }

    save_config(config)
}

pub fn list_suspends() -> Result<(), String> {
    let config = load_config()?;

    if !config.suspend.classes.is_empty() {
        let classes = config.suspend.classes.iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        println!("Suspended Classes: {}", classes);
    }

    if !config.suspend.services.is_empty() {
        let services = config.suspend.services.iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        println!("Suspended Services: {}", services);
    }

    Ok(())
}

pub fn clear_suspends() -> Result<(), String> {
    let mut config = load_config()?;
    config.suspend.classes.clear();
    config.suspend.services.clear();
    save_config(config)
}

pub fn disable_suspend_filter() -> Result<(), String> {
    let mut config = load_config()?;
    config.suspend.enabled = false;
    save_config(config)
}