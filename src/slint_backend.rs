use embedded_graphics::{draw_target::DrawTarget, geometry::Point, pixelcolor::raw::RawU16, Pixel};
use rppal::gpio::Gpio;
use slint::platform::software_renderer as renderer;
use std::{cell::RefCell, rc::Rc};

type RenderPixel = embedded_graphics::pixelcolor::Rgb565;
type TargetPixel = slint::platform::software_renderer::Rgb565Pixel;
struct DrawBuffer<Display: DrawTarget<Color = RenderPixel>> {
    display: Display,
    buffer: &'static mut [TargetPixel],
}

/// Implement the `LineBufferProvider` trait for the `DrawBuffer` type.
/// This allows the `DrawBuffer` type to be used as a line buffer provider for the software renderer.
impl<Display: DrawTarget<Color = RenderPixel>> renderer::LineBufferProvider
    for &mut DrawBuffer<Display>
{
    type TargetPixel = TargetPixel;

    fn process_line(
        &mut self,
        line: usize,
        range: core::ops::Range<usize>,
        render_fn: impl FnOnce(&mut [Self::TargetPixel]),
    ) {
        render_fn(&mut self.buffer[range.clone()]);
        let mut set_buffer = vec![];
        for x in range {
            let c = RawU16::new(self.buffer[x].0).into();
            let p = Point::new((x + 1) as i32, (line + 2) as i32);
            let pixel = Pixel(p, c);
            set_buffer.push(pixel);
        }
        let _a = self.display.draw_iter(set_buffer.iter().cloned());
    }
}

struct RaspberryBackend<Display>
where
    Display: DrawTarget<Color = RenderPixel>,
{
    window: RefCell<Option<Rc<renderer::MinimalSoftwareWindow>>>,
    window_size: slint::PhysicalSize,
    buffer_provider: RefCell<DrawBuffer<Display>>,
    start_time: std::time::Instant,
    keys: RefCell<Option<Vec<KeyPress>>>,
}
impl<Display: DrawTarget<Color = RenderPixel>> RaspberryBackend<Display> {
    fn new(
        display: Display,
        window_size: slint::PhysicalSize,
        keys: Option<Vec<KeyPress>>,
    ) -> Self {
        Self {
            window: Default::default(),
            window_size,
            buffer_provider: RefCell::new(DrawBuffer {
                display: display,
                buffer: vec![TargetPixel::default(); window_size.width as _].leak(),
            }),
            start_time: std::time::Instant::now(),
            keys: RefCell::new(keys),
        }
    }
}

pub fn init<Display: DrawTarget<Color = RenderPixel> + 'static>(
    display: Display,
    window_size: slint::PhysicalSize,
    keys: Option<Vec<KeyPress>>,
) {
    let raspberr_backend = RaspberryBackend::new(display, window_size, keys);
    slint::platform::set_platform(Box::new(raspberr_backend)).expect("backend already initialized");
}

impl<Display: DrawTarget<Color = RenderPixel>> slint::platform::Platform
    for RaspberryBackend<Display>
{
    fn create_window_adapter(
        &self,
    ) -> Result<Rc<dyn slint::platform::WindowAdapter>, slint::PlatformError> {
        let window =
            renderer::MinimalSoftwareWindow::new(renderer::RepaintBufferType::SwappedBuffers);
        window.set_size(self.window_size);
        self.window.replace(Some(window.clone()));
        Ok(window)
    }
    fn duration_since_start(&self) -> std::time::Duration {
        self.start_time.elapsed()
    }

    fn run_event_loop(&self) -> Result<(), slint::PlatformError> {
        loop {
            slint::platform::update_timers_and_animations();
            if let Some(window) = self.window.borrow().clone() {
                window.draw_if_needed(|renderer| {
                    let mut buffer_provider = self.buffer_provider.borrow_mut();
                    renderer.render_by_line(&mut *buffer_provider);
                });
                // 设置按键事件
                if let Some(keys) = self.keys.borrow().as_ref() {
                    for key in keys {
                        key.key_event_detect(window.clone());
                    }
                }
                // 检查是否有活动的动画
                if window.has_active_animations() {
                    continue;
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct KeyPress {
    pin: rppal::gpio::InputPin,
    text: slint::SharedString,
    pressed: RefCell<bool>,
    key_time: RefCell<std::time::Instant>,
}
impl KeyPress {
    pub fn new(pin: u8, key: slint::SharedString) -> Self {
        Self {
            pin: Gpio::new().unwrap().get(pin).unwrap().into_input_pullup(),
            text: key,
            pressed: RefCell::new(false),
            key_time: RefCell::new(std::time::Instant::now()),
        }
    }

    fn key_event_detect(&self, window: Rc<renderer::MinimalSoftwareWindow>) {
        // slint::platform::Key::Shift.into()
        let pressed = self.pressed.borrow().to_owned();
        if self.pin.is_low() && !pressed {
            window.dispatch_event(slint::platform::WindowEvent::KeyPressed {
                text: self.text.clone(),
            });
            println!("{} pressed", self.text);
            // pressed = true;
            *self.pressed.borrow_mut() = true;
            *self.key_time.borrow_mut() = std::time::Instant::now();
        }
        // 连续按键检测
        if self.pin.is_low() && pressed && self.key_time.borrow().elapsed().as_millis() > 300 {
            *self.key_time.borrow_mut() = std::time::Instant::now();
            window.dispatch_event(slint::platform::WindowEvent::KeyPressed {
                text: self.text.clone(),
            });
            println!("{} pressed", self.text);
        }
        if self.pin.is_high() && pressed {
            window.dispatch_event(slint::platform::WindowEvent::KeyReleased {
                text: self.text.clone(),
            });
            // pressed = false;
            println!("{} released", self.text);
            *self.pressed.borrow_mut() = false;
        }
    }
}
