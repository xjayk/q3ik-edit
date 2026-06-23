use gpui::{App, Context, Task};

pub static ZED_DISABLE_STAFF: bool = false;

pub trait FeatureFlagAppExt {
    fn on_flags_ready(
        &mut self,
        callback: impl 'static + FnMut(FlagState, &mut App),
    ) -> Task<()>;
    fn update_flags(&mut self, enabled: bool, flags: Vec<String>);
}

impl FeatureFlagAppExt for App {
    fn on_flags_ready(
        &mut self,
        _: impl 'static + FnMut(FlagState, &mut App),
    ) -> Task<()> {
        Task::ready(())
    }
    fn update_flags(&mut self, _: bool, _: Vec<String>) {}
}

impl<T> FeatureFlagAppExt for Context<'_, T> {
    fn on_flags_ready(
        &mut self,
        _: impl 'static + FnMut(FlagState, &mut App),
    ) -> Task<()> {
        Task::ready(())
    }
    fn update_flags(&mut self, _: bool, _: Vec<String>) {}
}

pub struct FlagState {
    pub is_staff: bool,
}
