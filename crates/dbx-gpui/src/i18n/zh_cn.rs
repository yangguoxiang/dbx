use std::collections::HashMap;

pub fn messages() -> HashMap<String, String> {
    let mut m = HashMap::new();
    // App
    m.insert("app.name".into(), "DBX".into());
    m.insert("app.tagline".into(), "15 MB, 40+ 数据库, AI 驱动".into());

    // Toolbar
    m.insert("toolbar.new_query".into(), "新建查询".into());
    m.insert("toolbar.new_connection".into(), "新建连接".into());
    m.insert("toolbar.toggle_theme".into(), "切换主题".into());
    m.insert("toolbar.toggle_ai".into(), "AI 助手".into());
    m.insert("toolbar.toggle_history".into(), "查询历史".into());
    m.insert("toolbar.settings".into(), "设置".into());
    m.insert("toolbar.transfer".into(), "数据传输".into());
    m.insert("toolbar.schema_diff".into(), "Schema 对比".into());
    m.insert("toolbar.data_compare".into(), "数据对比".into());

    // Sidebar
    m.insert("sidebar.connections".into(), "连接".into());
    m.insert("sidebar.saved_sql".into(), "保存的 SQL".into());
    m.insert("sidebar.no_connections".into(), "暂无连接".into());

    // Connection
    m.insert("connection.new".into(), "新建连接".into());
    m.insert("connection.edit".into(), "编辑连接".into());
    m.insert("connection.delete".into(), "删除连接".into());
    m.insert("connection.test".into(), "测试连接".into());
    m.insert("connection.connect".into(), "连接".into());
    m.insert("connection.disconnect".into(), "断开".into());
    m.insert("connection.save".into(), "保存".into());
    m.insert("connection.cancel".into(), "取消".into());
    m.insert("connection.name".into(), "名称".into());
    m.insert("connection.type".into(), "数据库类型".into());
    m.insert("connection.host".into(), "主机".into());
    m.insert("connection.port".into(), "端口".into());
    m.insert("connection.username".into(), "用户名".into());
    m.insert("connection.password".into(), "密码".into());
    m.insert("connection.database".into(), "数据库".into());
    m.insert("connection.ssh_enabled".into(), "SSH 隧道".into());
    m.insert("connection.ssl_enabled".into(), "SSL".into());
    m.insert("connection.testing".into(), "正在测试连接...".into());
    m.insert("connection.test_success".into(), "连接成功".into());
    m.insert("connection.test_failed".into(), "连接失败".into());
    m.insert("connection.connecting".into(), "正在连接...".into());
    m.insert("connection.connected".into(), "已连接".into());

    // Query Editor
    m.insert("editor.execute".into(), "执行".into());
    m.insert("editor.cancel".into(), "取消".into());
    m.insert("editor.format".into(), "格式化".into());
    m.insert("editor.save".into(), "保存".into());
    m.insert("editor.explain".into(), "执行计划".into());
    m.insert("editor.placeholder".into(), "在此输入 SQL 查询...".into());
    m.insert("editor.no_results".into(), "无结果显示".into());
    m.insert("editor.execution_time".into(), "执行时间".into());
    m.insert("editor.rows_affected".into(), "影响行数".into());
    m.insert("editor.truncated".into(), "结果已截断".into());

    // Data Grid
    m.insert("grid.loading".into(), "加载中...".into());
    m.insert("grid.no_data".into(), "无数据".into());
    m.insert("grid.row_count".into(), "行".into());
    m.insert("grid.column_count".into(), "列".into());
    m.insert("grid.copy".into(), "复制".into());
    m.insert("grid.copy_with_headers".into(), "复制含表头".into());
    m.insert("grid.export_csv".into(), "导出 CSV".into());
    m.insert("grid.export_excel".into(), "导出 Excel".into());

    // AI
    m.insert("ai.title".into(), "AI 助手".into());
    m.insert("ai.placeholder".into(), "让 AI 帮你写、修、解释 SQL...".into());
    m.insert("ai.generate_sql".into(), "生成 SQL".into());
    m.insert("ai.fix_sql".into(), "修复 SQL".into());
    m.insert("ai.explain_sql".into(), "解释 SQL".into());
    m.insert("ai.optimize_sql".into(), "优化 SQL".into());
    m.insert("ai.copy_sql".into(), "复制 SQL 到编辑器".into());
    m.insert("ai.sending".into(), "思考中...".into());
    m.insert("ai.configure".into(), "请在设置中配置 AI 提供商".into());

    // Schema
    m.insert("schema.databases".into(), "数据库".into());
    m.insert("schema.tables".into(), "表".into());
    m.insert("schema.views".into(), "视图".into());
    m.insert("schema.functions".into(), "函数".into());
    m.insert("schema.procedures".into(), "存储过程".into());
    m.insert("schema.triggers".into(), "触发器".into());
    m.insert("schema.indexes".into(), "索引".into());
    m.insert("schema.columns".into(), "列".into());
    m.insert("schema.foreign_keys".into(), "外键".into());
    m.insert("schema.refresh".into(), "刷新".into());

    // Settings
    m.insert("settings.title".into(), "设置".into());
    m.insert("settings.general".into(), "通用".into());
    m.insert("settings.editor".into(), "编辑器".into());
    m.insert("settings.ai".into(), "AI".into());
    m.insert("settings.shortcuts".into(), "快捷键".into());
    m.insert("settings.cloud_sync".into(), "云同步".into());
    m.insert("settings.about".into(), "关于".into());
    m.insert("settings.theme".into(), "主题".into());
    m.insert("settings.language".into(), "语言".into());
    m.insert("settings.font".into(), "字体".into());
    m.insert("settings.font_size".into(), "字号".into());

    // Dialogs
    m.insert("dialog.confirm".into(), "确认".into());
    m.insert("dialog.cancel".into(), "取消".into());
    m.insert("dialog.close".into(), "关闭".into());
    m.insert("dialog.danger_title".into(), "潜在危险操作".into());

    // History
    m.insert("history.title".into(), "查询历史".into());
    m.insert("history.search".into(), "搜索历史...".into());
    m.insert("history.no_records".into(), "暂无查询历史".into());

    // Status Bar
    m.insert("status.ready".into(), "就绪".into());
    m.insert("status.executing".into(), "执行中...".into());

    m
}
