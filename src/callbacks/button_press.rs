use crate::windowmanager::WindowManager;
use crate::xlibwrapper::{
    core::*,
    event::*,
    masks::*,
};
use std::rc::Rc;


pub fn button_press(xlib: Rc<XlibWrapper>, wm: &mut WindowManager, event: Event) {

    let (window, x_root, y_root, state, button) =
        match event {
            Event {
                event_type: EventType::ButtonPress,
                payload: Some(EventPayload::ButtonPress(window, _sub_window, button, x_root, y_root, state))
            } => (window, x_root, y_root, state, button),
            _ => { return; }
        };

    if !wm.current_monitor().expect("button_press: current_monitor 1").contains_window(window) || window == xlib.get_root() {
        return
    }

    println!("Button pressed from: {}", window);

    let geometry = xlib.get_geometry(window);

    wm.drag_start_pos = (x_root as i32 , y_root as i32);
    wm.drag_start_frame_pos = (geometry.x,geometry.y);
    wm.drag_start_frame_size = (geometry.width, geometry.height);

    if button == Button1 {
        println!("Button1 pressed");
        /*{
            use std::fs::{File, OpenOptions};
            use std::io::{Write};
            let path = "/home/pierre/hadlock.log";
            match OpenOptions::new()
                .write(true)
                .create(true)
                .append(true)
                .open(path)
                {
                    Ok(mut x) => {
                        let pointer_pos = xlib.pointer_pos();
                        let output_text = &format!("Pointer location: {:?}, Screen from \"pointer_is_inside\":{:?}" , pointer_pos, wm.get_focused_screen());
                        write!(x, "{}\n", output_text);
                    },
                    Err(e) => println!("{}", e)
                };
        }*/
        let ww = wm.current_monitor().expect("button_press: current_monitor 2").get_client(window).expect(&format!("Button press no client: {}", window)).clone();
        println!("Pointer location: {:?}", xlib.pointer_pos());
        match xlib.get_upmost_window() {
            Some(x) if x != window => wm.raise_window(&ww),
            _ => { return }
        }
    }
}
