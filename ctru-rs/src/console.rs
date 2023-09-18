//! Virtual text console.
//!
//! The [`Console`] works as a virtual shell that renders on screen all output of `stdout`. As such, it is useful as a basic interface to show info to the user,
//! such as in simple "Hello World" applications or more complex software that does not need much user interaction.
//!
//! Have a look at [`Soc::redirect_to_3dslink()`](crate::services::soc::Soc::redirect_to_3dslink) for a better alternative when debugging applications.

use std::cell::RefMut;
use std::default::Default;

use ctru_sys::{consoleClear, consoleInit, consoleSelect, consoleSetWindow, PrintConsole};

use crate::services::gfx::Screen;

static mut EMPTY_CONSOLE: PrintConsole = unsafe { const_zero::const_zero!(PrintConsole) };

/// Virtual text console.
///
/// [`Console`] lets the application redirect `stdout` and `stderr` to a simple text displayer on the 3DS screen.
/// This means that any text written to `stdout` and `stderr` (e.g. using `println!`, `eprintln!` or `dbg!`) will become visible in the area taken by the console.
///
/// # Notes
///
/// The [`Console`] will take full possession of the screen handed to it as long as it stays alive. It also supports some ANSI codes, such as text color and cursor positioning.
/// The [`Console`]'s window size will be:
/// - 40x30 on the [`BottomScreen`](crate::services::gfx::BottomScreen).
/// - 50x30 on the normal [`TopScreen`](crate::services::gfx::TopScreen).
/// - 100x30 on the [`TopScreen`](crate::services::gfx::TopScreen) when wide mode is enabled.
///
/// # Alternatives
///
/// If you'd like to see live standard output while running the application but cannot or do not want to show the text on the 3DS itself,
/// you can try using [`Soc::redirect_to_3dslink`](crate::services::soc::Soc::redirect_to_3dslink) while activating the `--server` flag for `3dslink` (also supported by `cargo-3ds`).
/// More info in the [`cargo-3ds` docs](https://github.com/rust3ds/cargo-3ds#running-executables).
#[doc(alias = "PrintConsole")]
pub struct Console<'screen> {
    context: Box<PrintConsole>,
    screen: RefMut<'screen, dyn Screen>,
}

impl<'screen> Console<'screen> {
    /// Initialize a console on the chosen screen.
    ///
    /// # Notes
    ///
    /// This operation overwrites whatever was on the screen before the initialization (including other [`Console`]s)
    /// and changes the [`FramebufferFormat`](crate::services::gspgpu::FramebufferFormat) of the selected screen to better suit the [`Console`].
    ///
    /// The new console is automatically selected for printing.
    ///
    /// [`Console`] automatically takes care of flushing and swapping buffers for its screen when printing.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// #
    /// use ctru::services::gfx::Gfx;
    /// use ctru::console::Console;
    ///
    /// // Initialize graphics.
    /// let gfx = Gfx::new()?;
    ///
    /// // Create a `Console` that takes control of the upper LCD screen.
    /// let top_console = Console::new(gfx.top_screen.borrow_mut());
    ///
    /// println!("I'm on the top screen!");
    /// #
    /// # Ok(())
    /// # }
    /// ```
    #[doc(alias = "consoleInit")]
    pub fn new(screen: RefMut<'screen, dyn Screen>) -> Self {
        let mut context = Box::<PrintConsole>::default();

        unsafe { consoleInit(screen.as_raw(), context.as_mut()) };

        Console { context, screen }
    }

    /// Returns `true` if a valid [`Console`] to print on is currently selected.
    ///
    /// # Notes
    ///
    /// This function is used to check whether one of the two screens has an existing (and selected) [`Console`],
    /// so that the program can be sure its output will be shown *somewhere*.
    ///
    /// The main use of this is within the [`ctru::use_panic_handler()`](crate::use_panic_handler()) hook,
    /// since it will only stop the program's execution if the user is able to see the panic information output on screen.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// #
    /// # use ctru::services::gfx::Gfx;
    /// # // Initialize graphics.
    /// # let gfx = Gfx::new()?;
    /// #
    /// use ctru::console::Console;
    /// let top_console = Console::new(gfx.top_screen.borrow_mut());
    ///
    /// // There is at least one selected `Console`.
    /// assert!(Console::exists());
    /// #
    /// # Ok(())
    /// # }
    /// ```
    pub fn exists() -> bool {
        unsafe {
            let current_console = ctru_sys::consoleSelect(&mut EMPTY_CONSOLE);

            let res = (*current_console).consoleInitialised;

            ctru_sys::consoleSelect(current_console);

            res
        }
    }

    /// Select this console as the current target for standard output.
    ///
    /// # Notes
    ///
    /// Any previously selected console will be unhooked and will not show the `stdout` and `stderr` output.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// #
    /// # use ctru::services::gfx::Gfx;
    /// # let gfx = Gfx::new()?;
    /// #
    /// use ctru::console::Console;
    ///
    /// // Create a `Console` that takes control of the upper LCD screen.
    /// let top_console = Console::new(gfx.top_screen.borrow_mut());
    ///
    /// // Create a `Console` that takes control of the lower LCD screen.
    /// let bottom_console = Console::new(gfx.bottom_screen.borrow_mut());
    ///
    /// // Remember that `Console::new` automatically selects the new `Console` for output.
    /// println!("I'm on the bottom screen!");
    ///
    /// top_console.select();
    ///
    /// println!("Being on the upper screen is much better!");
    /// #
    /// # Ok(())
    /// # }
    /// ```
    #[doc(alias = "consoleSelect")]
    pub fn select(&self) {
        unsafe {
            consoleSelect(self.context.as_ref() as *const _ as *mut _);
        }
    }

    /// Clear all text from the console.
    #[doc(alias = "consoleClear")]
    pub fn clear(&self) {
        unsafe { consoleClear() }
    }

    /// Resize the console to fit in a smaller portion of the screen.
    ///
    /// # Notes
    ///
    /// The first two arguments are the desired coordinates of the top-left corner
    /// of the new window based on the row/column coordinates of a full-screen console.
    /// The second pair is the new width and height.
    ///
    /// # Panics
    ///
    /// This function will panic if the new window's position or size does not fit the screen.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// #
    /// # use ctru::services::gfx::Gfx;
    /// # let gfx = Gfx::new()?;
    /// #
    /// # use ctru::console::Console;
    /// #
    /// let mut top_console = Console::new(gfx.top_screen.borrow_mut());
    /// top_console.set_window(10, 10, 16, 6);
    ///
    /// println!("I'm becoming claustrophobic in here!");
    /// #
    /// # Ok(())
    /// # }
    /// ```
    #[doc(alias = "consoleSetWindow")]
    pub fn set_window(&mut self, x: u8, y: u8, width: u8, height: u8) {
        let height_limit = 30;
        let length_limit = self.max_width();

        if x >= length_limit {
            panic!("x coordinate of new console window out of bounds");
        }
        if y >= height_limit {
            panic!("y coordinate of new console window out of bounds");
        }

        if (x + width) > length_limit {
            panic!("width of new console window out of bounds");
        }
        if (y + height) > height_limit {
            panic!("height of new console window out of bounds");
        }

        unsafe {
            consoleSetWindow(
                self.context.as_mut(),
                x.into(),
                y.into(),
                width.into(),
                height.into(),
            )
        };
    }

    /// Reset the window's size to default parameters.
    ///
    /// This can be used to undo the changes made by [`set_window()`](Console::set_window()).
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// #
    /// # use ctru::services::gfx::Gfx;
    /// # let gfx = Gfx::new()?;
    /// #
    /// # use ctru::console::Console;
    /// #
    /// let mut top_console = Console::new(gfx.top_screen.borrow_mut());
    /// top_console.set_window(15, 15, 8, 10);
    ///
    /// println!("It's really jammed in here!");
    ///
    /// top_console.reset_window();
    ///
    /// println!("Phew, finally a breath of fresh air.");
    /// #
    /// # Ok(())
    /// # }
    /// ```
    pub fn reset_window(&mut self) {
        let width = self.max_width();

        unsafe { consoleSetWindow(self.context.as_mut(), 0, 0, width.into(), 30) };
    }

    /// Returns this [`Console`]'s maximum character width depending on the screen used.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// #
    /// # use ctru::services::gfx::Gfx;
    /// # use ctru::console::Console;
    /// #
    /// let gfx = Gfx::new()?;
    ///
    /// let top_console = Console::new(gfx.top_screen.borrow_mut());
    ///
    /// // The maximum width for the top screen (without any alterations) is 50 characters.
    /// assert_eq!(top_console.max_width(), 50);
    /// #
    /// # Ok(())
    /// # }
    /// ```
    pub fn max_width(&self) -> u8 {
        match self.screen.as_raw() {
            ctru_sys::GFX_TOP => {
                if unsafe { ctru_sys::gfxIsWide() } {
                    100
                } else {
                    50
                }
            }
            ctru_sys::GFX_BOTTOM => 40,
            _ => unreachable!(),
        }
    }
}

impl Drop for Console<'_> {
    fn drop(&mut self) {
        unsafe {
            // Safety: We are about to deallocate the PrintConsole data pointed
            // to by libctru. Without this drop code libctru would have a
            // dangling pointer that it writes to on every print. To prevent
            // this we replace the console with an empty one if it was selected.
            // This is the same state that libctru starts up in, before
            // initializing a console. Writes to the console will not show up on
            // the screen, but it won't crash either.

            // Get the current console by replacing it with an empty one.
            let current_console = ctru_sys::consoleSelect(&mut EMPTY_CONSOLE);

            if std::ptr::eq(current_console, &*self.context) {
                // Console dropped while selected. We just replaced it with the
                // empty console so nothing more to do.
            } else {
                // Console dropped while a different console was selected. Put back
                // the console that was selected.
                ctru_sys::consoleSelect(current_console);
            }
        }
    }
}
