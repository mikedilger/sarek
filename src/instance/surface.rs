
use std::mem;
use std::ptr;
use std::sync::Arc;
use vks::*;
use winit;
use super::{Instance, InstanceLoader};
use Error;

#[cfg(windows)]
use winapi;
#[cfg(windows)]
use user32;

pub struct Surface {
    surface: VkSurfaceKHR,
    loader: InstanceLoader
}

impl Instance {
    #[cfg(feature = "khr_xlib_surface")]
    pub fn create_surface(&self, loader: InstanceLoader, window: &winit::Window)
                          -> Result<Surface, Error>
    {
        use winit::os::unix::WindowExt;

        let x11_display = window.get_xlib_display().unwrap();
        let x11_window = window.get_xlib_window().unwrap();
        let create_info = VkXlibSurfaceCreateInfoKHR {
            sType: VK_STRUCTURE_TYPE_XLIB_SURFACE_CREATE_INFO_KHR,
            pNext: ptr::null(),
            flags: Default::default(),
            dpy: x11_display as *mut xlib_wrapper::Display,
            window: xlib_wrapper::Window(x11_window as u32)
        };
        let surface = unsafe {
            let mut surface: VkSurfaceKHR = mem::uninitialized();
            vk_try!((loader.0.khr_xlib_surface.vkCreateXlibSurfaceKHR)(
                self.0,
                &create_info,
                ptr::null(), // allocator
                &mut surface));
            surface
        };
        Ok(Surface {
            surface: surface,
            loader: loader
        })
    }

    #[cfg(feature = "khr_win32_surface")]
    pub fn create_surface(&self, loader: InstanceLoader, window: &winit::Window)
                          -> Result<Surface, Error>
    {
        use winit::os::windows::WindowExt;

        let hwnd = window.get_hwnd() as *mut winapi::windef::HWND__;
        let hinstance = user32::GetWindow(hwnd, 0) as *const ();
        let create_info = VkWin32SurfaceCreateInfoKHR {
            sType: VK_STRUCTURE_TYPE_WIN32_SURFACE_CREATE_INFO_KHR,
            pNext: ptr::null(),
            flags: Default::default(),
            hinstance: hinstance,
            hwnd: hwnd as *const (),
        };
        let surface = unsafe {
            let mut surface: VkSurfaceKHR = mem::uninitialized();
            vk_try!((loader.0.khr_win32_surface.vkCreateWin32SurfaceKHR)(
                self.0,
                &create_info,
                ptr::null(), // allocator
                &mut surface));
            surface
        };
        Ok(Surface {
            surface: surface,
            loader: loader
        })
    }

    //#[cfg(feature = "khr_xcb_surface")]

    //#[cfg(feature = "khr_wayland_surface")]

    //#[cfg(feature = "khr_mir_surface")]

    //#[cfg(feature = "khr_android_surface")]
}