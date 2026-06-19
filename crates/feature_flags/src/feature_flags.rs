use std::sync::LazyLock;
use gpui::{App, Context, Global, Subscription, Window};

pub use feature_flags_macros::EnumFeatureFlag;

pub static ZED_DISABLE_STAFF: LazyLock<bool> = LazyLock::new(|| false);

pub trait FeatureFlagValue: Sized + Clone + Eq + Default + std::fmt::Debug + Send + Sync + 'static {
    fn all_variants() -> &'static [Self];
    fn override_key(&self) -> &'static str;
    fn from_wire(wire: &str) -> Option<Self>;
    fn label(&self) -> &'static str { self.override_key() }
    fn on_variant() -> Self { Self::default() }
}

#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub enum PresenceFlag { On, #[default] Off }

impl std::ops::Deref for PresenceFlag {
    type Target = bool;
    fn deref(&self) -> &bool { match self { PresenceFlag::On => &true, PresenceFlag::Off => &false } }
}

impl FeatureFlagValue for PresenceFlag {
    fn all_variants() -> &'static [Self] { &[PresenceFlag::On, PresenceFlag::Off] }
    fn override_key(&self) -> &'static str { match self { PresenceFlag::On => "on", PresenceFlag::Off => "off" } }
    fn label(&self) -> &'static str { match self { PresenceFlag::On => "On", PresenceFlag::Off => "Off" } }
    fn from_wire(_: &str) -> Option<Self> { Some(PresenceFlag::On) }
    fn on_variant() -> Self { PresenceFlag::On }
}

pub trait FeatureFlag {
    const NAME: &'static str;
    type Value: FeatureFlagValue;
    fn enabled_for_staff() -> bool { true }
    fn enabled_for_all() -> bool { false }
    fn watch<V: 'static>(cx: &mut Context<V>) {
        cx.observe_global::<FeatureFlagStore>(|_, cx| cx.notify()).detach();
    }
}

pub trait FeatureFlagViewExt<V: 'static> {
    fn observe_flag<T: FeatureFlag, F>(&mut self, window: &Window, callback: F) -> Subscription
    where F: Fn(T::Value, &mut V, &mut Window, &mut Context<V>) + Send + Sync + 'static;
}

impl<V> FeatureFlagViewExt<V> for Context<'_, V> where V: 'static {
    fn observe_flag<T: FeatureFlag, F>(&mut self, window: &Window, callback: F) -> Subscription
    where F: Fn(T::Value, &mut V, &mut Window, &mut Context<V>) + 'static {
        let value = T::Value::on_variant();
        self.defer_in(window, move |view, window, cx| { callback(value.clone(), view, window, cx); });
        self.observe_global_in::<FeatureFlagStore>(window, |_, _, _| {})
    }
}

pub struct OnFlagsReady { pub is_staff: bool }

pub trait FeatureFlagAppExt {
    fn update_flags(&mut self, staff: bool, flags: Vec<String>);
    fn set_staff(&mut self, staff: bool);
    fn has_flag<T: FeatureFlag>(&self) -> bool;
    fn flag_value<T: FeatureFlag>(&self) -> T::Value;
    fn is_staff(&self) -> bool;
    fn feature_flag_overrides_enabled(&self) -> bool;
    fn on_flags_ready<F>(&mut self, callback: F) -> Subscription where F: FnMut(OnFlagsReady, &mut App) + 'static;
    fn observe_flag<T: FeatureFlag, F>(&mut self, callback: F) -> Subscription where F: FnMut(T::Value, &mut App) + 'static;
}

impl FeatureFlagAppExt for App {
    fn update_flags(&mut self, _staff: bool, _flags: Vec<String>) {}
    fn set_staff(&mut self, _staff: bool) {}
    fn has_flag<T: FeatureFlag>(&self) -> bool { T::enabled_for_all() || (T::enabled_for_staff() && !*ZED_DISABLE_STAFF) }
    fn flag_value<T: FeatureFlag>(&self) -> T::Value { T::Value::on_variant() }
    fn is_staff(&self) -> bool { !*ZED_DISABLE_STAFF }
    fn feature_flag_overrides_enabled(&self) -> bool { !*ZED_DISABLE_STAFF }
    fn on_flags_ready<F>(&mut self, mut callback: F) -> Subscription where F: FnMut(OnFlagsReady, &mut App) + 'static {
        let is_staff = !*ZED_DISABLE_STAFF;
        self.defer(move |cx| { callback(OnFlagsReady { is_staff }, cx); });
        self.observe_global::<FeatureFlagStore>(move |_| {})
    }
    fn observe_flag<T: FeatureFlag, F>(&mut self, mut callback: F) -> Subscription where F: FnMut(T::Value, &mut App) + 'static {
        let value = T::Value::on_variant();
        self.defer(move |cx| { callback(value, cx); });
        self.observe_global::<FeatureFlagStore>(move |_| {})
    }
}

pub struct FeatureFlagDescriptor {
    pub name: &'static str,
    pub variants: fn() -> Vec<FeatureFlagVariant>,
    pub on_variant_key: fn() -> &'static str,
    pub default_variant_key: fn() -> &'static str,
    pub enabled_for_all: fn() -> bool,
    pub enabled_for_staff: fn() -> bool,
    pub type_id: fn() -> std::any::TypeId,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FeatureFlagVariant {
    pub override_key: &'static str,
    pub label: &'static str,
}

#[macro_export]
macro_rules! register_feature_flag { ($flag:ty) => {}; }

#[derive(Default)]
pub struct FeatureFlagStore {
    staff: bool,
    server_flags: collections::HashMap<String, String>,
    server_flags_received: bool,
    _settings_subscription: Option<Subscription>,
}

impl FeatureFlagStore {
    pub fn init(cx: &mut App) { cx.set_global(FeatureFlagStore::default()); }
    pub fn known_flags() -> impl Iterator<Item = &'static FeatureFlagDescriptor> { [].iter() }
    pub fn is_staff(&self) -> bool { self.staff }
    pub fn set_staff(&mut self, staff: bool, _cx: &mut App) { self.staff = staff; }
    pub fn set_flag_override(&mut self, _flag: String, _variant: Option<String>) {}
    pub fn server_flags_received(&self) -> bool { true }
    pub fn all_overrides(&self, _flag: &str, _app: &App) -> Vec<(String, String)> { vec![] }
    pub fn is_forced_on(_descriptor: &FeatureFlagDescriptor) -> bool { false }
    pub fn resolved_key(_descriptor: &FeatureFlagDescriptor, _cx: &App) -> &'static str { "" }
    pub fn override_for(_name: &str, _cx: &App) -> Option<String> { None }
    pub fn clear_override(_name: &str, _fs: &dyn fs::Fs, _cx: &mut App) {}
    pub fn set_override(_name: &str, _variant_key: String, _fs: &dyn fs::Fs, _cx: &mut App) {}
}

impl Global for FeatureFlagStore {}
