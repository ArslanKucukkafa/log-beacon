use crate::models::shell_model::ObjectType;
use crate::services::configuration_service::{load_config, save_config};

pub fn add_condition(
    object_type: ObjectType,
    object_name: &str
) -> Result<(), String> {
    let mut config = load_config()?;

    match object_type {
        ObjectType::CLASS => {
            if !config.condition.classes.contains(&object_name.to_string()) {
                config.condition.classes.push(object_name.to_string());
            }
        }
        ObjectType::SERVICE => {
            if !config.condition.services.contains(&object_name.to_string()) {
                config.condition.services.push(object_name.to_string());
            }
        }
        _ => {}
    }

    save_config(config)
}

pub fn remove_condition(
    object_type: ObjectType,
    object_name: &str
) -> Result<(), String> {
    let mut config = load_config()?;

    match object_type {
        ObjectType::CLASS => {
            config.condition.classes.retain(|x| x != object_name);
        }
        ObjectType::SERVICE => {
            config.condition.services.retain(|x| x != object_name);
        }
        _ => {}
    }

    save_config(config)
}

pub fn list_conditions() -> Result<(), String> {
    let config = load_config()?;

    if !config.condition.classes.is_empty() {
        println!("Condition Classes:");
        for class in &config.condition.classes {
            println!("- {}", class);
        }
    }

    if !config.condition.services.is_empty() {
        println!("\nCondition Services:");
        for service in &config.condition.services {
            println!("- {}", service);
        }
    }

    Ok(())
}

pub fn disable_condition_filter() -> Result<(), String> {
    let mut config = load_config()?;
    config.condition.enabled = false;
    save_config(config)
}

pub fn clear_conditions() -> Result<(), String> {
    let mut config = load_config()?;
    config.condition.classes.clear();
    config.condition.services.clear();
    save_config(config)
}

pub fn check_condition(object_type: ObjectType, object_name: &str) -> bool {
    match object_type {
        ObjectType::CLASS => {true},
        ObjectType::SERVICE => {false},
        ObjectType::MESSAGE => false, // Veya uygun implementasyon
    }
}
