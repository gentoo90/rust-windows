#[feature(globs)];

extern mod win32;

use std::local_data;

use win32::window::*;
use win32::ll::*;

struct MainFrame {
    win: Window,
    title: ~str,
}

impl OnCreate for MainFrame {}
impl OnDestroy for MainFrame {}

impl WndProc for MainFrame {
    fn wnd<'a>(&'a self) -> &'a Window {
        &self.win
    }

    fn wnd_mut<'a>(&'a mut self) -> &'a mut Window {
        &mut self.win
    }

    fn wnd_proc(&self, msg: UINT, w: WPARAM, l: LPARAM) -> LRESULT {
        if msg == 0x0001 { // WM_CREATE
            let cs = unsafe {
                let pcs = std::cast::transmute::<LPARAM, *CREATESTRUCT>(l);
                &(*pcs)
            };
            let ret = self.on_create(cs);
            return if ret { 0 as LRESULT } else { -1 as LRESULT };
        }
        if msg == 0x0002 { // WM_DESTROY
            self.on_destroy();
            return 0 as LRESULT;
        }
        if msg == 0x000F { // WM_PAINT
            let (dc, ps) = (*self).begin_paint();
            self.on_paint(dc);
            (*self).end_paint(&ps);
            return 0 as LRESULT;
        }
        win32::def_window_proc(self.wnd().wnd, msg, w, l)
    }
}

impl OnPaint for MainFrame {
    fn on_paint(&self, dc: HDC) {
        let hello = "hello world";
        (*self).text_out(dc, 0, 0, hello);
    }
}

impl MainFrame {
    fn new(instance: Instance, title: ~str) -> Option<Window> {
        let classname = "MainFrame";
        instance.register(classname);

        let proc = ~MainFrame {
            win: Window::null(),
            title: title.clone(),
        };
        local_data::set(key_init_wnd, proc as ~WndProc);

        Window::create(instance, classname, title)
    }
}

fn main() {
    init_window_map();

    let instance = Instance::main_instance();
    let main = MainFrame::new(instance, ~"Hello");
    let main = main.unwrap();

    main.show(1);
    main.update();

    let exit_code = win32::main_window_loop();
    std::os::set_exit_status(exit_code as int);
}
