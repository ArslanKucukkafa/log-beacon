use comfy_table::Table;
use comfy_table::presets::UTF8_FULL;
use comfy_table::TableComponent::*;
pub fn show_help() {
    let mut table = Table::new();
    table
        .set_header(vec!["Commands", "Description", "Usage"])
        .add_row(vec![
            "server",
            "This command provide to manage WebSocket server. You can start, stop and check server status.\nAlso you can set port number.",
            "log-beacon server start --port 8080",
        ])
        .add_row(vec![
            "level",
            "This command provide to modify the log level. You can add or remove log levels.\n\
            log levels are not keep like as hierarchy. You can add or remove any log level.",
            "log-beacon level add INFO or log-beacon level remove DEBUG",
        ])
        .add_row(vec![
            "suspend",
            "This command provide to suspend logging for specific classes or services.",
            "log-beacon suspend --service SERVICE_NAME",
        ]);
    table.load_preset(UTF8_FULL);
    table.set_style(BottomLeftCorner, '╰');
    println!("{table}");
}