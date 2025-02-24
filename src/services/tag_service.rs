use crate::models::config_model::Config;
use std::collections::HashMap;
use crate::models::shell_model::ObjectType;
use crate::services::configuration_service;
use std::fs;
use crate::services::configuration_service::save_config;

/// Burda tag işlemleri yapılacak
/// - Tag ekleme `add_tag()`
/// - Tag silme `remove_tag()`
/// - Mevcut tag configlerini listeleme `list_tags()`


pub fn add_tag(
    object_type: ObjectType,
    object_name: &str,
    tag: &str,
) -> Result<(), String> {
    let mut config = configuration_service::load_config()?;
    
    match object_type {
        ObjectType::CLASS => {
            config.tag.classes
                .entry(object_name.to_string())
                .or_insert_with(Vec::new)
                .push(tag.to_string());
        }
        ObjectType::SERVICE => {
            config.tag.services
                .entry(object_name.to_string())
                .or_insert_with(Vec::new)
                .push(tag.to_string());
        }
        _ => {}
    }
    
    save_config(config)
}

pub fn remove_tag(
    object_type: ObjectType,
    object_name: &str,
    tag: &str,
) -> Result<(), String> {
    let mut config = configuration_service::load_config()?;
    
    match object_type {
        ObjectType::CLASS => {
            if let Some(tags) = config.tag.classes.get_mut(object_name) {
                tags.retain(|t| t != tag);
            }
        }
        ObjectType::SERVICE => {
            if let Some(tags) = config.tag.services.get_mut(object_name) {
                tags.retain(|t| t != tag);
            }
        }
        _ => {}
    }
    
    save_config(config)
}


// TODO: list_tags funcitonu table olarak gösterilecek

pub fn list_tags() {
    let config = configuration_service::load_config().unwrap();

    if !config.tag.classes.is_empty() {
        println!("Tagged Classes:");
        for (class, tags) in &config.tag.classes {
            println!("- {}: {}", class, tags.join(", "));
        }
    }

    if !config.tag.services.is_empty() {
        println!("\nTagged Services:");
        for (service, tags) in &config.tag.services {
            println!("- {}: {}", service, tags.join(", "));
        }
    }
}

pub fn clear_tags() -> Result<(), String> {
    let mut config = configuration_service::load_config()?;

    config.tag.classes.clear();
    config.tag.services.clear();

    save_config(config)
}

pub fn disable_tag_filter() -> Result<(), String> {
    let mut config = configuration_service::load_config()?;
    
    config.tag.enabled = false;
    
    save_config(config)
}