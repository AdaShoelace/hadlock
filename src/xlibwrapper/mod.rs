pub mod action;
pub mod core;
pub mod cursor;
pub mod masks;
pub mod mock_core;
pub mod util;
pub mod xatom;
pub mod xlibmodels;

use {
    super::models::{dockarea::DockArea, screen::Screen, window_type::WindowType},
    std::os::raw::*,
    util::Position,
    util::*,
    x11_dl::xlib,
    xatom::XAtom,
    xlibmodels::*,
};

pub trait DisplayServer {
    fn get_screens(&self) -> Vec<Screen> {
        unimplemented!()
    }

    fn update_desktops(&self, _current_ws: u32, _num_of_ws: Option<u32>) {
        unimplemented!()
    }

    fn init_desktops_hints(&self) {
        unimplemented!()
    }

    fn get_window_states_atoms(&self, _window: xlib::Window) -> Vec<xlib::Atom> {
        unimplemented!()
    }

    fn set_window_states_atoms(&self, _window: xlib::Window, _states: Vec<xlib::Atom>) {
        unimplemented!()
    }

    fn set_desktop_prop_u64(&self, _value: u64, _atom: c_ulong, _type_: c_ulong) {
        unimplemented!()
    }

    fn atom_name(&self, _atom: xlib::Atom) -> Result<String, Box<dyn std::error::Error>> {
        unimplemented!()
    }

    fn add_to_save_set(&self, _w: Window) {
        unimplemented!()
    }

    fn remove_focus(&self, _w: Window) {
        unimplemented!()
    }

    fn set_input_focus(&self, _w: Window) {
        unimplemented!()
    }

    fn take_focus(&self, _w: Window) {
        unimplemented!()
    }

    fn set_window_background_color(&self, _w: Window, _color: Color) {
        unimplemented!()
    }

    fn set_border_color(&self, _w: Window, _color: Color) {
        unimplemented!()
    }

    fn window_under_pointer(&self) -> Option<Window> {
        unimplemented!()
    }

    fn pointer_pos(&self, _w: Window) -> Position {
        unimplemented!()
    }

    fn move_cursor(&self, _pos: Position) {
        unimplemented!()
    }

    fn center_cursor(&self, _w: Window) {
        unimplemented!()
    }

    fn configure_window(&self, _window: Window, _value_mask: Mask, _changes: WindowChanges) {
        unimplemented!()
    }

    fn add_to_root_net_client_list(&self, _w: Window) {
        unimplemented!()
    }

    fn update_net_client_list(&self, _clients: Vec<Window>) {
        unimplemented!()
    }

    fn create_simple_window(
        &self,
        _w: Window,
        _pos: Position,
        _size: Size,
        _border_width: u32,
        _border_color: Color,
        _bg_color: Color,
    ) -> Window {
        unimplemented!()
    }

    fn destroy_window(&self, _w: Window) {
        unimplemented!()
    }

    fn get_geometry(&self, _w: Window) -> Geometry {
        unimplemented!()
    }

    fn get_root(&self) -> Window {
        unimplemented!()
    }

    fn get_window_attributes(&self, _w: Window) -> WindowAttributes {
        unimplemented!()
    }

    fn grab_server(&self) {
        unimplemented!()
    }

    fn ungrab_server(&self) {
        unimplemented!()
    }

    fn ungrab_keys(&self, _w: Window) {
        unimplemented!()
    }

    fn ungrab_all_buttons(&self, _w: Window) {
        unimplemented!()
    }

    fn grab_button(
        &self,
        _button: u32,
        _modifiers: u32,
        _grab_window: Window,
        _owner_events: bool,
        _event_mask: u32,
        _pointer_mode: i32,
        _keyboard_mode: i32,
        _confine_to: Window,
        _cursor: u64,
    ) {
        unimplemented!()
    }

    fn str_to_keycode(&self, _key: &str) -> Option<KeyCode> {
        unimplemented!()
    }

    fn keycode_to_key_sym(&self, _keycode: KeyCode) -> String {
        unimplemented!()
    }

    fn key_sym_to_keycode(&self, _keysym: u64) -> KeyCode {
        unimplemented!()
    }

    fn get_window_type_atom(&self, _w: Window) -> Option<xlib::Atom> {
        unimplemented!()
    }

    fn get_class_hint(
        &self,
        _w: Window,
    ) -> Result<(String, String), Box<dyn std::error::Error>> {
        unimplemented!()
    }

    fn get_atom_prop_value(
        &self,
        _window: xlib::Window,
        _prop: xlib::Atom,
    ) -> Option<xlib::Atom> {
        unimplemented!()
    }

    fn grab_keys(&self, _w: Window, _keysym: u32, _modifiers: u32) {
        unimplemented!()
    }

    fn grab_key(
        &self,
        _key_code: u32,
        _modifiers: u32,
        _grab_window: Window,
        _owner_event: bool,
        _pointer_mode: i32,
        _keyboard_mode: i32,
    ) {
        unimplemented!()
    }

    fn kill_client(&self, _w: Window) -> bool {
        unimplemented!()
    }

    fn map_window(&self, _window: Window) {
        unimplemented!()
    }

    fn move_window(&self, _w: Window, _position: Position) {
        unimplemented!()
    }

    fn next_event(&self) -> xlib::XEvent {
        unimplemented!()
    }

    fn raise_window(&self, _w: Window) {
        unimplemented!()
    }

    fn resize_window(&self, _w: Window, _size: Size) {
        unimplemented!()
    }

    fn select_input(&self, _window: xlib::Window, _masks: Mask) {
        unimplemented!()
    }

    fn set_border_width(&self, _w: Window, _border_width: u32) {
        unimplemented!()
    }

    fn sync(&self, _discard: bool) {
        unimplemented!()
    }

    fn flush(&self) {
        unimplemented!()
    }

    fn get_window_strut_array(&self, _window: Window) -> Option<DockArea> {
        unimplemented!()
    }

    //new way to get strut

    fn get_window_type(&self, _window: xlib::Window) -> WindowType {
        unimplemented!()
    }

    fn get_upmost_window(&self) -> Option<Window> {
        unimplemented!()
    }

    fn reparent_client(&self, _w: Window, _size: Size, _pos: Position) -> Window {
        unimplemented!()
    }

    fn get_top_level_windows(&self) -> Vec<Window> {
        unimplemented!()
    }

    fn top_level_window_count(&self) -> u32 {
        unimplemented!()
    }

    fn transient_for_hint(&self, _w: Window) -> Option<Window> {
        unimplemented!()
    }

    fn unmap_window(&self, _w: Window) {
        unimplemented!()
    }

    fn should_be_managed(&self, _w: Window) -> bool {
        unimplemented!()
    }

    fn xatom(&self) -> &XAtom {
        unimplemented!()
    }

    fn exit(&self) {
        unimplemented!()
    }
}
