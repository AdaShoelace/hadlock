use crate::xlibwrapper::xlibmodels::Window;
pub enum InternalAction {
    Focus,
    FocusSpecific(Window),
    Destroy(Window),
    UpdateLayout
}
