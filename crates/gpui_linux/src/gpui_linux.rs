use gpui::Platform;
use std::rc::Rc;

pub fn current_platform(_headless: bool) -> Rc<dyn Platform> {
    unimplemented!("q3ik-edit only supports macOS")
}
