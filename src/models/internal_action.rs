use crate::xlibwrapper::xlibmodels::Window;
pub enum InternalAction {
    Focus,
    FocusSpecific(Window),
    _Destroy(Window),
    UpdateLayout,
}
