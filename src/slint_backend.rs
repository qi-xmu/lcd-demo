use embedded_hal::blocking::spi::Transfer;
use embedded_hal::digital::{InputPin, OutputPin};
use slint::platform::software_renderer;
use std::{cell::RefCell, convert::Infallible, rc::Rc};

const DISPLAY_SIZE: slint::PhysicalSize = slint::PhysicalSize::new(128, 128);

type TargetPixel = slint::platform::software_renderer::Rgb565Pixel;
struct DrawBuffer<Display, PioTransfer, Stolen> {
    display: Display,
    buffer: &'static mut [TargetPixel],
    pio: Option<PioTransfer>,
    stolen_pin: Stolen,
}
struct RaspberryBackend<DrawBuffer> {
    window: RefCell<Option<Rc<slint::platform::software_renderer::MinimalSoftwareWindow>>>,
    buffer: RefCell<DrawBuffer>,
}

// impl Default for RaspberryBackend {
//     fn default() -> Self {
//         // 这个地方初始化硬件

//         Self {
//             window: RefCell::new(None),
//             buffer: RefCell::new([0; DISPLAY_SIZE.width as usize * DISPLAY_SIZE.height as usize]),
//         }
//     }
// }

impl<
        DI: display_interface::WriteOnlyDataCommand,
        RST: OutputPin<Error = Infallible>,
        BL: OutputPin<Error = Infallible>,
        CS: OutputPin<Error = Infallible>,
        // CH: SingleChannel,
        // TO: WriteTarget<TransmittedWord = u8> + FullDuplex<u8>,
    > slint::platform::Platform for RaspberryBackend<DrawBuffer<st7789::ST7789<DI, RST, BL>>>
{
}

// impl slint::platform::Platform for RaspberryBackend {
//     fn create_window_adapter(
//         &self,
//     ) -> Result<Rc<dyn slint::platform::WindowAdapter>, slint::PlatformError> {
//         let window = slint::platform::software_renderer::MinimalSoftwareWindow::new(
//             slint::platform::software_renderer::RepaintBufferType::SwappedBuffers,
//         );
//         self.window.replace(Some(window.clone()));
//         Ok(window)
//     }

//     fn run_event_loop(&self) -> Result<(), slint::PlatformError> {
//         self.window
//             .borrow()
//             .as_ref()
//             .unwrap()
//             .set_size(slint::PhysicalSize::new(
//                 DISPLAY_SIZE.width as u32,
//                 DISPLAY_SIZE.height as u32,
//             ));
//         loop {
//             slint::platform::update_timers_and_animations();
//             if let Some(window) = self.window.borrow().clone() {
//                 window.draw_if_needed(|renderer| todo!());
//             }
//         }
//     }
// }
