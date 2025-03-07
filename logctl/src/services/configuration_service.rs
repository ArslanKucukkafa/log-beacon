use comfy_table::presets::UTF8_FULL;
use comfy_table::{Cell, ContentArrangement, Row, Table};
use log_common::models::config_model::Config;
use log_common::models::log_model::LogLevel;

pub fn display_config(config: &Config) {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec![
            Cell::new("Config Key").fg(comfy_table::Color::Cyan),
            Cell::new("Config Value").fg(comfy_table::Color::Green),
            Cell::new("Status").fg(comfy_table::Color::Yellow)
        ]);

    // Socket Port Satırı
    table.add_row(Row::from(vec![
        "socket.port",
        &config.socket.port.to_string(),
        ""
    ]));

    // Filtre Durumları için Ortak Fonksiyon
    fn add_filter_row(
        table: &mut Table,
        key: &str,
        values: &Vec<String>,
        enabled: bool
    ) {
        let status = if enabled { "✅ Enabled" } else { "❌ Disabled" };
        let value = if values.is_empty() {
            "-".to_string()
        } else {
            values.join(" | ")
        };

        table.add_row(Row::from(vec![
            key,
            &value,
            status
        ]));
    }

    // Log Levels
    add_filter_row(
        &mut table,
        "Log Levels",
        &config.log.levels
            .iter()
            .map(LogLevel::to_string)
            .collect::<Vec<String>>(),
        config.log.enabled
    );

    // Suspended Classes/Services
    add_filter_row(
        &mut table,
        "Suspended Classes",
        &config.suspend.classes,
        config.suspend.enabled
    );

    add_filter_row(
        &mut table,
        "Suspended Services",
        &config.suspend.services,
        config.suspend.enabled
    );

    add_filter_row(
        &mut table,
        "Condition Classes",
        &config.condition.classes,
        config.condition.enabled
    );

    add_filter_row(
        &mut table,
        "Condition Services",
        &config.condition.services,
        config.condition.enabled
    );

    add_filter_row(
        &mut table,
        "Tagged Services",
        &config.tag.services
            .iter()
            .map(|(key, values)| format!("{}: {}", key, values.join(", ")))
            .collect::<Vec<String>>(),
        config.tag.enabled
    );

    add_filter_row(
        &mut table,
        "Tagged Classes",
        &config.tag.classes
            .iter()
            .map(|(key, values)| format!("{}: {}", key, values.join(", ")))
            .collect::<Vec<String>>(),
        config.tag.enabled
    );

    table.add_row(Row::from(vec![
        "Regex Filter",
        &config.regexp.pattern,
        ""
    ]));

    println!("{}", table);
}