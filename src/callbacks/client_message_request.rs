use crate::windowmanager::*;
use crate::xlibwrapper::event::*;
use crate::xlibwrapper::core::*;
use std::rc::Rc;

/*
 *  All the credit for this solution goes to lex148 @ github.com
 *  I made some minor tweaks to make it fit my project but all the logic is lex148's
 */


pub fn client_message_request(xlib: Rc<XlibWrapper>, wm: &mut WindowManager, event: Event) {

    let (window, message_type, data) = match event {
        Event::ClientMessageRequest{win, message_type, data} => (win, message_type, data),
        _ => { return; }
    };

    debug!("From client_message_request-handler | message_type: {}, type_raw: {}", xlib.xatom.get_name(message_type), message_type);

    let data_zero = *data.get(0).expect("client_message_request: cleanupt");
    let data_one = *data.get(1).expect("client_message_request: cleanupt");
    let data_two = *data.get(2).expect("client_message_request: cleanupt");

    if message_type == xlib.xatom.NetWMState
        && (data_one == xlib.xatom.NetWMStateFullscreen as i64
            || data_two == xlib.xatom.NetWMStateFullscreen as i64) {

            let set_fullscreen = data_zero == 1;
            let toggle_fullscreen = data_zero == 2;


            let mut states = xlib.get_window_states_atoms(window);

            //determine what to change the state to
            let fullscreen = if toggle_fullscreen {
                !states.contains(&xlib.xatom.NetWMStateFullscreen)
            } else {
                set_fullscreen
            };

            //update the list of states
            if fullscreen {
                states.push(xlib.xatom.NetWMStateFullscreen);
            } else {
                states.retain(|x| x != &xlib.xatom.NetWMStateFullscreen);
            }
            states.sort();
            states.dedup();

            //set the windows state
            xlib.set_window_states_atoms(window, states);
            wm.toggle_maximize(window);
        }
}

