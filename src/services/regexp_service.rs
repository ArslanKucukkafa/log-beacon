use crate::services::log_parser::parse_log;
use crate::models::shell_model::ObjectType;
use crate::services::configuration_service::{load_config, save_config};
use comfy_table::{Table, Row, Cell, ContentArrangement};
use regex::Regex;
use comfy_table::Color;
use chrono::format;

pub fn add_regexp(
    pattern: &str
) {
    let mut config = load_config().map_err(|e| format!("Config yükleme hatası: {}", e)).unwrap();

    let compiled_re = Regex::new(pattern)
        .map_err(|e| format!("Geçersiz regex patterni: {}", e));

    config.regexp.pattern = pattern.to_string();
    save_config(config).map_err(|e| format!("Config kaydetme hatası: {}", e));
}

pub fn test_regexp(
    pattern: &str,
    sample_log: &str
) -> Result<Table, String> {
    // 1. Regex validation
    let re = Regex::new(pattern)
        .map_err(|e| format!("❌ Geçersiz regex:\n{}", e))?;

    // 2. Parsing attempt
    let parsed_log = parse_log(sample_log, pattern)
        .map_err(|e| format!("❌ Parse hatası:\n{}", e))?;

    // 3. Tablo oluşturma
    let mut table = Table::new();
    table
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec!["ALAN", "DEĞER"]);
    table
        .add_row(Row::from(vec![
            Cell::new("Pattern").fg(Color::Green),
            Cell::new(pattern)
        ]))
        .add_row(Row::from(vec![
            Cell::new("Örnek Log").fg(Color::Cyan),
            Cell::new(sample_log)
        ]))
        .add_row(Row::from(vec![
            Cell::new("Parse Sonucu").fg(Color::Blue),
            Cell::new(&format!("{:#?}", parsed_log))
        ]));

    Ok(table)
}