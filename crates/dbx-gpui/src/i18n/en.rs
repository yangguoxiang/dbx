use std::collections::HashMap;

pub fn messages() -> HashMap<String, String> {
    let mut m = HashMap::new();
    // App
    m.insert("app.name".into(), "DBX".into());
    m.insert("app.tagline".into(), "15 MB, 40+ Databases, AI-Powered".into());

    // Toolbar
    m.insert("toolbar.new_query".into(), "New Query".into());
    m.insert("toolbar.new_connection".into(), "New Connection".into());
    m.insert("toolbar.toggle_theme".into(), "Toggle Theme".into());
    m.insert("toolbar.toggle_ai".into(), "AI Assistant".into());
    m.insert("toolbar.toggle_history".into(), "Query History".into());
    m.insert("toolbar.settings".into(), "Settings".into());
    m.insert("toolbar.transfer".into(), "Data Transfer".into());
    m.insert("toolbar.schema_diff".into(), "Schema Diff".into());
    m.insert("toolbar.data_compare".into(), "Data Compare".into());

    // Sidebar
    m.insert("sidebar.connections".into(), "Connections".into());
    m.insert("sidebar.saved_sql".into(), "Saved SQL".into());
    m.insert("sidebar.no_connections".into(), "No connections yet".into());

    // Connection
    m.insert("connection.new".into(), "New Connection".into());
    m.insert("connection.edit".into(), "Edit Connection".into());
    m.insert("connection.delete".into(), "Delete Connection".into());
    m.insert("connection.test".into(), "Test Connection".into());
    m.insert("connection.connect".into(), "Connect".into());
    m.insert("connection.disconnect".into(), "Disconnect".into());
    m.insert("connection.save".into(), "Save".into());
    m.insert("connection.cancel".into(), "Cancel".into());
    m.insert("connection.name".into(), "Name".into());
    m.insert("connection.type".into(), "Database Type".into());
    m.insert("connection.host".into(), "Host".into());
    m.insert("connection.port".into(), "Port".into());
    m.insert("connection.username".into(), "Username".into());
    m.insert("connection.password".into(), "Password".into());
    m.insert("connection.database".into(), "Database".into());
    m.insert("connection.ssh_enabled".into(), "SSH Tunnel".into());
    m.insert("connection.ssl_enabled".into(), "SSL".into());
    m.insert("connection.testing".into(), "Testing connection...".into());
    m.insert("connection.test_success".into(), "Connection successful".into());
    m.insert("connection.test_failed".into(), "Connection failed".into());
    m.insert("connection.connecting".into(), "Connecting...".into());
    m.insert("connection.connected".into(), "Connected".into());

    // Query Editor
    m.insert("editor.execute".into(), "Execute".into());
    m.insert("editor.cancel".into(), "Cancel".into());
    m.insert("editor.format".into(), "Format".into());
    m.insert("editor.save".into(), "Save".into());
    m.insert("editor.explain".into(), "Explain".into());
    m.insert("editor.placeholder".into(), "Write your SQL query here...".into());
    m.insert("editor.no_results".into(), "No results to display".into());
    m.insert("editor.execution_time".into(), "Execution time".into());
    m.insert("editor.rows_affected".into(), "Rows affected".into());
    m.insert("editor.truncated".into(), "Results truncated".into());

    // Data Grid
    m.insert("grid.loading".into(), "Loading...".into());
    m.insert("grid.no_data".into(), "No data".into());
    m.insert("grid.row_count".into(), "rows".into());
    m.insert("grid.column_count".into(), "columns".into());
    m.insert("grid.copy".into(), "Copy".into());
    m.insert("grid.copy_with_headers".into(), "Copy with Headers".into());
    m.insert("grid.export_csv".into(), "Export as CSV".into());
    m.insert("grid.export_excel".into(), "Export as Excel".into());

    // AI
    m.insert("ai.title".into(), "AI Assistant".into());
    m.insert("ai.placeholder".into(), "Ask AI to write, fix, or explain SQL...".into());
    m.insert("ai.generate_sql".into(), "Generate SQL".into());
    m.insert("ai.fix_sql".into(), "Fix SQL".into());
    m.insert("ai.explain_sql".into(), "Explain SQL".into());
    m.insert("ai.optimize_sql".into(), "Optimize SQL".into());
    m.insert("ai.copy_sql".into(), "Copy SQL to Editor".into());
    m.insert("ai.sending".into(), "Thinking...".into());
    m.insert("ai.configure".into(), "Configure AI provider in Settings".into());

    // Schema
    m.insert("schema.databases".into(), "Databases".into());
    m.insert("schema.tables".into(), "Tables".into());
    m.insert("schema.views".into(), "Views".into());
    m.insert("schema.functions".into(), "Functions".into());
    m.insert("schema.procedures".into(), "Procedures".into());
    m.insert("schema.triggers".into(), "Triggers".into());
    m.insert("schema.indexes".into(), "Indexes".into());
    m.insert("schema.columns".into(), "Columns".into());
    m.insert("schema.foreign_keys".into(), "Foreign Keys".into());
    m.insert("schema.refresh".into(), "Refresh".into());

    // Settings
    m.insert("settings.title".into(), "Settings".into());
    m.insert("settings.general".into(), "General".into());
    m.insert("settings.editor".into(), "Editor".into());
    m.insert("settings.ai".into(), "AI".into());
    m.insert("settings.shortcuts".into(), "Shortcuts".into());
    m.insert("settings.cloud_sync".into(), "Cloud Sync".into());
    m.insert("settings.about".into(), "About".into());
    m.insert("settings.theme".into(), "Theme".into());
    m.insert("settings.language".into(), "Language".into());
    m.insert("settings.font".into(), "Font".into());
    m.insert("settings.font_size".into(), "Font Size".into());

    // Dialogs
    m.insert("dialog.confirm".into(), "Confirm".into());
    m.insert("dialog.cancel".into(), "Cancel".into());
    m.insert("dialog.close".into(), "Close".into());
    m.insert("dialog.danger_title".into(), "Potentially Dangerous Operation".into());

    // History
    m.insert("history.title".into(), "Query History".into());
    m.insert("history.search".into(), "Search history...".into());
    m.insert("history.no_records".into(), "No query history".into());

    // Status Bar
    m.insert("status.ready".into(), "Ready".into());
    m.insert("status.executing".into(), "Executing...".into());

    m
}
