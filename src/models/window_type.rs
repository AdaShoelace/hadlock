#[derive(Debug, Clone, PartialEq)]
pub enum WindowType {
    Desktop,
    Dock,
    Toolbar,
    Menu,
    Utility,
    Splash,
    Dialog,
    Normal,
}

impl WindowType {
    pub fn get_name(&self) -> String {
        match self {
            WindowType::Desktop => "Desktop".into(),
            WindowType::Dock => "Dock".into(),
            WindowType::Toolbar => "Toolbar".into(),
            WindowType::Dialog => "Dialog".into(),
            WindowType::Menu => "Menu".into(),
            WindowType::Splash => "Splash".into(),
            WindowType::Utility => "Utility".into(),
            WindowType::Normal => "Normal".into(),
            _ => "Unknown".into(),
        }
    }
}
