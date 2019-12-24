
use {
    std::cell::RefCell,
    crate::xlibwrapper::{
        masks::*,
        util::*,
        xlibmodels::*,
        action,
    },
    crate::models::{
        windowwrapper::*,
    },
    x11_dl::xlib::{
        self,
        XEvent
    },
    std::rc::Rc,
    crate::xlibwrapper::action::*,
    crate::xlibwrapper::core::XlibWrapper,
    crate::state::State,
    crate::config::CONFIG,
    reducer::*,
};

pub struct HdlReactor {
    lib: Rc<XlibWrapper>,
    state: RefCell<State>,
}

impl HdlReactor {
    pub fn new(lib: Rc<XlibWrapper>, original_state: State) -> Self {
        Self {
            lib,
            state: RefCell::new(original_state)
        }
    }

    fn subscribe_to_events(&self, w: Window) {
        self.lib.select_input(
            w,
            EnterWindowMask | LeaveWindowMask | FocusChangeMask | PropertyChangeMask
        );
        self.lib.sync(false);
    }

    fn grab_buttons(&self, w: Window) {
        let buttons = vec![Button1, Button3];
        buttons
            .iter()
            .for_each(|button| {
                self.lib.grab_button(
                    *button,
                    Mod4Mask,
                    w,
                    false,
                    (ButtonPressMask | ButtonReleaseMask | ButtonMotionMask) as u32,
                    GrabModeAsync,
                    GrabModeAsync,
                    0,0);
            });
    }

    fn grab_keys(&self, w: Window) {

        let _keys = vec!["q",
        "Left",
        "Up",
        "Right",
        "Down",
        "Return",
        "f",
        "e",
        "c",
        "h", "j", "k", "l",
        "d",
        "1", "2", "3", "4", "5", "6", "7", "8", "9"]
            .iter()
            .map(|key| { keysym_lookup::into_keysym(key).expect("Core: no such key") })
            .for_each(|key_sym| { self.lib.grab_keys(w, key_sym, Mod4Mask | Shift) });
    }

    fn set_focus(&self, focus: Window) {
        if focus == self.lib.get_root() { return }
        self.grab_buttons(focus);
        self.grab_keys(focus);
        self.lib.take_focus(focus);
        self.lib.set_border_width(focus, CONFIG.border_width as u32);
        self.lib.set_border_color(focus, CONFIG.border_color);
        //self.lib.resize_window(focus, size.width - 2* CONFIG.border_width as u32, size.height - 2*CONFIG.border_width as u32);
        self.lib.raise_window(focus);
        self.lib.sync(false);
    }

    pub fn unset_focus(&self, w: Window) {
        self.lib.remove_focus(w);
        self.lib.ungrab_all_buttons(w);
        self.lib.ungrab_keys(w);
        self.lib.set_border_width(w, 0);
        self.lib.set_border_color(w, CONFIG.background_color);
        /*let size = ww.get_size();
          self.lib.resize_window(w, size.width, size.height);*/
        self.lib.sync(false);
    }
}

impl Reactor<State> for HdlReactor {
    type Output = ();

    fn react(&self, state: &State) {
        state.windows
            .iter()
            .for_each(|(key, val)| {
                match val.handle_state {
                    HandleState::New => {
                        self.lib.add_to_save_set(*key);
                        self.lib.add_to_root_net_client_list(*key);
                        self.lib.move_window(*key, val.get_position());
                        self.lib.resize_window(*key, val.get_size());
                        self.subscribe_to_events(*key);
                        self.lib.map_window(*key);
                    },
                    HandleState::Map => {
                        self.lib.map_window(*key);
                    },
                    HandleState::Move(pos) => self.lib.move_window(*key, pos),
                    HandleState::ResizeRelative(size) => {
                        let size = Size{ width: val.get_size().width + (size.width - val.get_size().width),
                                            height: val.get_size().height + (size.height - val.get_size().height)};
                        self.lib.resize_window(*key, size);
                    },
                    HandleState::Focus => self.set_focus(*key),
                    HandleState::Unfocus => self.unset_focus(*key),
                    _ => ()
                }
            });
        self.state.replace(state.clone());
    }
}
