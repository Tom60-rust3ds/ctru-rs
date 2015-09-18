use ::raw::services::gsp;

use core::convert::From;

pub enum Event {
    Psc0,
    Psc1,
    VBlank0,
    VBlank1,
    PPF,
    P3D,
    DMA
}

#[derive(Copy, Clone)]
pub enum FramebufferFormat {
    Rgba8,
    Bgr8,
    Rgb565,
    Rgb5A1,
    Rgba4
}

impl FramebufferFormat {
    pub fn pixel_depth_bytes(&self) -> usize {
        use self::FramebufferFormat::*;
        match *self {
            Rgba8 => 4usize,
            Bgr8 => 3usize,
            Rgb565 => 2usize,
            Rgb5A1 => 2usize,
            Rgba4 => 2usize
        }
    }
}

impl From<gsp::GSP_FramebufferFormats> for FramebufferFormat {
    #[inline] fn from(g: gsp::GSP_FramebufferFormats) -> FramebufferFormat {
        use ::raw::services::gsp::GSP_FramebufferFormats::*;
        use self::FramebufferFormat::*;
        match g {
            GSP_RGBA8_OES => Rgba8,
            GSP_BGR8_OES => Bgr8,
            GSP_RGB565_OES => Rgb565,
            GSP_RGB5_A1_OES => Rgb5A1,
            GSP_RGBA4_OES => Rgba4
        }
    }
}

impl From<FramebufferFormat> for gsp::GSP_FramebufferFormats {
    #[inline] fn from(g: FramebufferFormat) -> gsp::GSP_FramebufferFormats {
        use ::raw::services::gsp::GSP_FramebufferFormats::*;
        use self::FramebufferFormat::*;
        match g {
            Rgba8 => GSP_RGBA8_OES,
            Bgr8 => GSP_BGR8_OES,
            Rgb565 => GSP_RGB565_OES,
            Rgb5A1 => GSP_RGB5_A1_OES,
            Rgba4 => GSP_RGBA4_OES
        }
    }
}

fn to_raw_event(ev: Event) -> gsp::GSP_Event {
    use ::raw::services::gsp::GSP_Event::*;
    use self::Event::*;

    match ev {
        Psc0 => GSPEVENT_PSC0,
        Psc1 => GSPEVENT_PSC1,
        VBlank0 => GSPEVENT_VBlank0,
        VBlank1 => GSPEVENT_VBlank1,
        PPF => GSPEVENT_PPF,
        P3D => GSPEVENT_P3D,
        DMA => GSPEVENT_DMA
    }
}

/// Sleep until GSP event fires.
///
/// # Examples
///
/// Wait for VBlank.
///
/// ```
/// use ctru::services::apt;
/// apt::main_loop(|| {
///     wait_for_event(Event::VBlank0);
/// });
pub fn wait_for_event(ev: Event) -> () {
    unsafe {
        // TODO second argument?
        gsp::gspWaitForEvent(to_raw_event(ev), 0);
    }
}