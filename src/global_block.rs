#[cfg(target_os = "windows")]
mod windows {
    use std::ptr;
    use std::sync::mpsc;
    use std::thread::{self, JoinHandle};

    use windows_sys::Win32::Foundation::{LPARAM, LRESULT, WPARAM};
    use windows_sys::Win32::UI::Input::KeyboardAndMouse::{
        GetAsyncKeyState, VK_CONTROL, VK_ESCAPE, VK_F12, VK_LWIN, VK_MENU, VK_RWIN, VK_TAB,
    };
    use windows_sys::Win32::UI::WindowsAndMessaging::{
        CallNextHookEx, DispatchMessageW, GetMessageW, HC_ACTION, HHOOK, KBDLLHOOKSTRUCT, MSG,
        PostThreadMessageW, SetWindowsHookExW, TranslateMessage, UnhookWindowsHookEx,
        WH_KEYBOARD_LL, WM_KEYDOWN, WM_KEYUP, WM_QUIT, WM_SYSKEYDOWN, WM_SYSKEYUP,
    };

    const LLKHF_ALTDOWN: u32 = 0x20;

    pub struct GlobalKeyBlocker {
        thread_id: u32,
        hook_thread: Option<JoinHandle<()>>,
    }

    impl GlobalKeyBlocker {
        pub fn start() -> Self {
            let (tx, rx) = mpsc::channel();

            let handle = thread::spawn(move || {
                let thread_id = unsafe { windows_sys::Win32::System::Threading::GetCurrentThreadId() };
                let hook = unsafe { SetWindowsHookExW(WH_KEYBOARD_LL, Some(hook_proc), ptr::null_mut(), 0) };
                let _ = tx.send((thread_id, !hook.is_null()));

                if hook.is_null() {
                    return;
                }

                let mut msg: MSG = unsafe { std::mem::zeroed() };
                loop {
                    let got = unsafe { GetMessageW(&mut msg, ptr::null_mut(), 0, 0) };
                    if got <= 0 {
                        break;
                    }

                    unsafe {
                        TranslateMessage(&msg);
                        DispatchMessageW(&msg);
                    }
                }

                unsafe {
                    UnhookWindowsHookEx(hook);
                }
            });

            let (thread_id, installed) = rx.recv().unwrap_or((0, false));
            if !installed {
                return Self {
                    thread_id: 0,
                    hook_thread: Some(handle),
                };
            }

            Self {
                thread_id,
                hook_thread: Some(handle),
            }
        }
    }

    impl Drop for GlobalKeyBlocker {
        fn drop(&mut self) {
            if self.thread_id != 0 {
                unsafe {
                    PostThreadMessageW(self.thread_id, WM_QUIT, 0, 0);
                }
            }

            if let Some(handle) = self.hook_thread.take() {
                let _ = handle.join();
            }
        }
    }

    unsafe extern "system" fn hook_proc(code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
        if code as u32 == HC_ACTION {
            let msg = w_param as u32;
            if msg == WM_KEYDOWN || msg == WM_SYSKEYDOWN || msg == WM_KEYUP || msg == WM_SYSKEYUP {
                let kb = unsafe { &*(l_param as *const KBDLLHOOKSTRUCT) };

                if should_block(kb) {
                    return 1;
                }
            }
        }

        unsafe { CallNextHookEx(0 as HHOOK, code, w_param, l_param) }
    }

    fn should_block(kb: &KBDLLHOOKSTRUCT) -> bool {
        let vk = kb.vkCode as i32;
        let alt_down = (kb.flags & LLKHF_ALTDOWN) != 0;
        let ctrl_down = unsafe { GetAsyncKeyState(VK_CONTROL as i32) < 0 };

        vk == VK_LWIN as i32
            || vk == VK_RWIN as i32
            || vk == VK_F12 as i32
            || (vk == VK_TAB as i32 && alt_down)
            || (vk == VK_ESCAPE as i32 && alt_down)
            || (vk == VK_ESCAPE as i32 && ctrl_down)
            || (vk == VK_MENU as i32 && alt_down)
    }

    pub fn start() -> GlobalKeyBlocker {
        GlobalKeyBlocker::start()
    }
}

#[cfg(not(target_os = "windows"))]
mod windows {
    pub struct GlobalKeyBlocker;

    pub fn start() -> GlobalKeyBlocker {
        GlobalKeyBlocker
    }
}

pub use windows::start;
