#![allow(dead_code)]
use prelude::*;
use std::mem;
use instance::Instance;
use entry::Entry;
use vk;
use std::ffi::CStr;
use ::RawPtr;

#[derive(Clone)]
pub struct XlibSurface {
    handle: vk::Instance,
    xlib_surface_fn: vk::XlibSurfaceFn,
}

impl XlibSurface {
    pub fn new(entry: &Entry, instance: &Instance) -> Result<XlibSurface, Vec<&'static str>> {
        let surface_fn = vk::XlibSurfaceFn::load(|name| {
            unsafe {
                mem::transmute(entry.get_instance_proc_addr(instance.handle(), name.as_ptr()))
            }
        })?;
        Ok(XlibSurface {
            handle: instance.handle(),
            xlib_surface_fn: surface_fn,
        })
    }

    pub fn name() -> &'static CStr {
        CStr::from_bytes_with_nul(b"VK_KHR_xlib_surface\0").expect("Wrong extension string")
    }

    pub unsafe fn create_xlib_surface_khr(&self,
                                          create_info: &vk::XlibSurfaceCreateInfoKHR,
                                          allocation_callbacks: Option<&vk::AllocationCallbacks>)
                                          -> VkResult<vk::SurfaceKHR> {
        let mut surface = mem::uninitialized();
        let err_code = self.xlib_surface_fn
            .create_xlib_surface_khr(self.handle,
                                     create_info,
                                     allocation_callbacks.as_raw_ptr(),
                                     &mut surface);
        match err_code {
            vk::Result::Success => Ok(surface),
            _ => Err(err_code),
        }
    }
}