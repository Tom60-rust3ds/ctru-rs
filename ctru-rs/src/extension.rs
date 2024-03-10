impl Swap for TopScreenLeft {
    fn swap_buffers(&mut self) {
        return;
        unsafe {
            ctru_sys::gfxScreenSwapBuffers(ctru_sys::GFX_TOP, false);
        }
    }

    fn set_double_buffering(&mut self, enabled: bool) {
        return;
        unsafe { ctru_sys::gfxSetDoubleBuffering(ctru_sys::GFX_TOP, enabled) }
    }    
}

impl Swap for TopScreenRight {
    fn swap_buffers(&mut self) {
        unsafe {
            ctru_sys::gfxScreenSwapBuffers(ctru_sys::GFX_TOP, false);
        }
    }

    fn set_double_buffering(&mut self, enabled: bool) {
        unsafe { ctru_sys::gfxSetDoubleBuffering(ctru_sys::GFX_TOP, enabled) }
    }    
}