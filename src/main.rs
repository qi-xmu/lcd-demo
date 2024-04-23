use display_interface_spi::SPIInterfaceNoCS;
use embedded_graphics::draw_target::DrawTarget;
// use embedded_graphics::geometry::Dimensions;
// use embedded_graphics::mono_font::ascii::FONT_10X20;
// use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::{Point, RgbColor};
// use embedded_graphics::text::Text;
// use embedded_graphics::Drawable;
use rppal::gpio::Gpio;
use rppal::spi::Spi;
use std::process::ExitCode;

mod slint_backend;
fn main() -> ExitCode {
    // GPIO
    let gpio = Gpio::new().unwrap();
    // reset
    let rst = gpio.get(27).unwrap().into_output();
    // backlight
    let mut backlight = gpio.get(24).unwrap().into_output();
    // data/commend
    let dc = gpio.get(25).unwrap().into_output();
    // spi
    let spi = Spi::new(
        rppal::spi::Bus::Spi0,
        rppal::spi::SlaveSelect::Ss0,
        60_000_000,
        rppal::spi::Mode::Mode0,
    )
    .unwrap();
    let di = SPIInterfaceNoCS::new(spi, dc);

    let mut delay = rppal::hal::Delay::new();
    let mut display = mipidsi::Builder::st7735s(di)
        .with_display_size(128, 128)
        .with_orientation(mipidsi::Orientation::Landscape(true))
        .with_invert_colors(mipidsi::ColorInversion::Inverted)
        .init(&mut delay, Some(rst))
        .unwrap();

    // Alternating color
    let colors = [Rgb565::RED, Rgb565::GREEN, Rgb565::BLUE];

    // Clear the display initially
    display.clear(colors[0]).unwrap();

    backlight.set_low();

    display.clear(Rgb565::BLACK).unwrap();

    ExitCode::SUCCESS
}
