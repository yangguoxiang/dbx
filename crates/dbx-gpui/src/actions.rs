use gpui::actions;

actions!(
    dbx,
    [
        NewQueryTab,
        ExecuteQuery,
        CancelQuery,
        FormatSql,
        SaveSql,
        CloseTab,
        ToggleAiPanel,
        ToggleHistoryPanel,
        ToggleSidebar,
        OpenConnectionDialog,
        OpenSettingsDialog,
        ToggleTheme,
    ]
);
