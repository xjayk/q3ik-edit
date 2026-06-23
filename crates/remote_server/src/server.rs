pub struct HeadlessProject;
pub struct HeadlessAppState;

impl HeadlessProject {
    pub fn init(_cx: &mut gpui::App) {}
    pub fn new(_state: HeadlessAppState, _cx: &mut gpui::Context<Self>) -> Self {
        Self
    }
}
