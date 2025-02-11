/// ## Configuration service
/// - Configuration service, uygulamanın çalışma zamanında değiştirilebilen ayarlarını tutar.
/// Tek bir işlevi vardır: `get_config()`


use crate::models::config_model::Config;

pub fn get_config() {
    match Config::from_file("app-config.toml") {
        Ok(cfg) => {
            println!("Config Loaded: {:#?}", cfg);
            
            if let Some(ref cla) = cfg.suspend.classes {
                println!("\nSuspend Classes:");
                for (k, v) in cla {
                    println!("  {} => {}", k, v);
                }
            }

            if let Some(tag_services) = cfg.tag.services {
                println!("Tag Services:");
                for (key, value) in tag_services {
                    println!("  {} = {}", key, value);
                }
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}



