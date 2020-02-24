use crate::xlibwrapper::xlibmodels::Window;
pub enum InternalAction {
    Focus,
    _FocusSpecific(Window),
    Destroy(Window),
    UpdateLayout
}
