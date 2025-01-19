use windows::Win32::Foundation::*;
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::core::PCSTR;

use crate::win32::{self, Error as Win32Error};
use super::tree_ctrl::TreeCtrl;
use super::log_ctrl::LogCtrl;
use super::status_bar::StatusBar;
use super::toolbar::Toolbar;
use super::UIComponent;

pub struct MainFrame {
    hwnd: HWND,
    tree_ctrl: TreeCtrl,
    log_ctrl: LogCtrl,
    status_bar: StatusBar,
    toolbar: Toolbar,
    is_visible: bool,
} 

impl MainFrame {
    pub fn new(instance: HINSTANCE) -> Result<Self, Win32Error> {
        // Register window class
        win32::register_window_class(
            PCSTR::from_raw(MAIN_WINDOW_CLASS.as_ptr()),
            Some(Self::window_proc),
            instance,
        )?;

        // Create main window
        let hwnd = win32::create_window(
            PCSTR::from_raw(MAIN_WINDOW_CLASS.as_ptr()),
            PCSTR::from_raw(b"EventGhost\0".as_ptr()),
            WS_OVERLAPPEDWINDOW | WS_CLIPCHILDREN,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            800,
            600,
            None,
            instance,
        )?;

        let mut frame = Self {
            hwnd,
            tree_ctrl: TreeCtrl::new(hwnd, instance)?,
            log_ctrl: LogCtrl::new(hwnd, instance)?,
            status_bar: StatusBar::new(hwnd, instance)?,
            toolbar: Toolbar::new(hwnd, instance)?,
            is_visible: false,
        };

        // Store the MainFrame instance pointer in the window's user data
        unsafe {
            SetWindowLongPtrA(hwnd, GWLP_USERDATA, &mut frame as *mut _ as isize);
        }

        frame.initialize()?;
        Ok(frame)
    }

    fn initialize(&mut self) -> Result<(), Win32Error> {
        // Initialize child controls
        self.tree_ctrl.initialize()?;
        self.log_ctrl.initialize()?;
        self.status_bar.initialize()?;
        self.toolbar.initialize()?;

        // Layout controls
        self.layout_controls();

        Ok(())
    }

    fn layout_controls(&mut self) {
        // Get the client area dimensions
        let mut client_rect = RECT::default();
        unsafe {
            GetClientRect(self.hwnd, &mut client_rect);
        }

        let width = client_rect.right - client_rect.left;
        let height = client_rect.bottom - client_rect.top;

        // Define heights for toolbar and status bar
        let toolbar_height = 40; // Example height
        let status_bar_height = 20; // Example height

        // Calculate tree and log control widths
        let tree_width = (width as f32 / 3.0) as i32;
        let log_width = width - tree_width;

        // Layout toolbar at the top
        unsafe {
            MoveWindow(
                self.toolbar.get_hwnd(),
                0,
                0,
                width,
                toolbar_height,
                true
            );
        }

        // Layout tree control on the left side
        unsafe {
            MoveWindow(
                self.tree_ctrl.get_hwnd(),
                0,
                toolbar_height,
                tree_width,
                height - toolbar_height - status_bar_height,
                true
            );
        }

        // Layout log control on the right side
        unsafe {
            MoveWindow(
                self.log_ctrl.get_hwnd(),
                tree_width,
                toolbar_height,
                log_width,
                height - toolbar_height - status_bar_height,
                true
            );
        }

        // Layout status bar at the bottom
        unsafe {
            MoveWindow(
                self.status_bar.get_hwnd(),
                0,
                height - status_bar_height,
                width,
                status_bar_height,
                true
            );
        }
    }

    unsafe extern "system" fn window_proc(
        hwnd: HWND,
        msg: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT {
        match msg {
            WM_DESTROY => {
                win32::post_message(hwnd, WM_QUIT, WPARAM(0), LPARAM(0))
                    .expect("Failed to post quit message");
                LRESULT(0)
            }
            WM_SIZE => {
                // Retrieve the MainFrame instance associated with this window
                let frame_ptr = GetWindowLongPtrA(hwnd, GWLP_USERDATA) as *mut MainFrame;

                if !frame_ptr.is_null() {
                    let frame = &mut *frame_ptr;
                    frame.layout_controls();
                }

                LRESULT(0)
            }
            WM_CREATE => {
                // Store the MainFrame instance pointer when the window is created
                let create_struct = lparam.0 as *const CREATESTRUCTA;
                if !create_struct.is_null() {
                    SetWindowLongPtrA(hwnd, GWLP_USERDATA, (*create_struct).lpCreateParams as isize);
                }
                LRESULT(0)
            }
            _ => unsafe { DefWindowProcA(hwnd, msg, wparam, lparam) }
        }
    }
}

impl Drop for MainFrame {
    fn drop(&mut self) {
        // Child controls will be destroyed automatically when parent window is destroyed
        unsafe {
            DestroyWindow(self.hwnd);
        }
    }
}

#[cfg(feature = "mainframe-test")]
mod tests {
    use super::*;
    use std::ptr::null_mut;

    #[test]
    fn test_mainframe_initialization() {
        // Mock HINSTANCE (nullptr for testing purposes)
        let instance = HINSTANCE(0);
        let result = MainFrame::new(instance);
        assert!(result.is_ok(), "MainFrame initialization failed");
        let frame = result.unwrap();
        assert_eq!(frame.is_visible, false, "MainFrame should not be visible initially");
    }

    #[test]
    fn test_layout_controls() {
        // Mock HINSTANCE (nullptr for testing purposes)
        let instance = HINSTANCE(0);
        let mut frame = MainFrame::new(instance).expect("Failed to create MainFrame");

        // Mock HWND and client area
        frame.hwnd = HWND(0); // Assign a mock HWND

        // Call layout_controls and ensure it doesn't panic
        frame.layout_controls();
    }
} 