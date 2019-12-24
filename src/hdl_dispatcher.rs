use {
    crate::xlibwrapper::{
        xlibmodels::*,
        action,
    },
    x11_dl::xlib::{
        self,
        XEvent
    },
    std::rc::Rc,
    crate::xlibwrapper::action::*,
    crate::xlibwrapper::core::XlibWrapper,
    crate::state::State,
    reducer::*,
    crate::hdl_reactor::HdlReactor,
};


pub fn run(xlib: Rc<XlibWrapper>) {
    let state = State::new(xlib.clone());
    let mut store = Store::new(state.clone(), HdlReactor::new(xlib.clone(), state));
    loop {
        let xevent = xlib.next_event();
        match xevent.get_type() {
            /*xlib::ConfigureRequest => {
              let event = xlib::XConfigureRequestEvent::from(xevent);
              let window_changes = WindowChanges {
              x: event.x,
              y: event.y,
              width: event.width,
              height: event.height,
              border_width: event.border_width,
              sibling: event.above,
              stack_mode: event.detail
              };
              store.dispatch(action::ConfigurationRequest{win: event.window, win_changes: window_changes, value_mask: event.value_mask})
              },*/
            xlib::MapRequest => {
                let event = xlib::XMapRequestEvent::from(xevent);
                store.dispatch(action::MapRequest{win: event.window})
            },
            /*xlib::UnmapNotify => {
              let event = xlib::XUnmapEvent::from(xevent);
              store.dispatch(action::UnmapNotify{win: event.window})
              },*/
            xlib::ButtonPress => {
                let event = xlib::XButtonEvent::from(xevent);
                store.dispatch(action::ButtonPress{
                    win: event.window,
                    sub_win: event.subwindow,
                    button: event.button,
                    x_root: event.x_root as u32,
                    y_root: event.y_root as u32,
                    state: event.state as u32
                });
            },
            /*xlib::ButtonRelease => {
              let event = xlib::XButtonEvent::from(xevent);
              action::ButtonRelease{
              win: event.window,
              sub_win: event.subwindow,
              button: event.button,
              x_root: event.x_root as u32,
              y_root: event.y_root as u32,
              state: event.state as u32
              };
              },
              xlib::KeyPress => {
              let event = xlib::XKeyEvent::from(xevent);
              action::KeyPress{win: event.window, state: event.state, keycode: event.keycode};
              },
              xlib::KeyRelease => {
              let event = xlib::XKeyEvent::from(xevent);
              action::KeyRelease{win: event.window, state: event.state, keycode: event.keycode};
              },*/
            xlib::MotionNotify => {
                let event = xlib::XMotionEvent::from(xevent);
                store.dispatch(action::MotionNotify{
                    win: event.window,
                    x_root: event.x_root,
                    y_root: event.y_root,
                    state: event.state
                })
            },
            xlib::EnterNotify => {
                let event = xlib::XCrossingEvent::from(xevent);
                store.dispatch(action::EnterNotify{win: event.window, sub_win: event.subwindow})
            },
            xlib::LeaveNotify => {
                let event = xlib::XCrossingEvent::from(xevent);
                store.dispatch(action::LeaveNotify{win: event.window})
            },
            /*xlib::Expose => {
              let event = xlib::XExposeEvent::from(xevent);
              action::Expose{win: event.window};
              },
              xlib::DestroyNotify => {
              let event = xlib::XDestroyWindowEvent::from(xevent);
              action::DestroyNotify{win: event.window};
              },
              xlib::PropertyNotify => {
              let event = xlib::XPropertyEvent::from(xevent);
              action::PropertyNotify{win: event.window, atom: event.atom};
              },
              xlib::ClientMessage => {
              let event = xlib::XClientMessageEvent::from(xevent);
              action::ClientMessageRequest{
              win: event.window,
              message_type: event.message_type,
              data: vec![event.data.get_long(0), event.data.get_long(1), event.data.get_long(2)]
              };
              },*/
            _ => store.dispatch(action::UnknownEvent)
        }
    }
}
