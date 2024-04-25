use display_interface_spi::SPIInterfaceNoCS;

use embedded_graphics::{draw_target::DrawTarget, pixelcolor::Rgb565};
use rppal::gpio::Gpio;

pub fn get_st7735s_display(
    size: (u16, u16),
    rst: u8,
    bl: u8,
    dc: u8,
) -> impl DrawTarget<Color = Rgb565> {
    let gpio = Gpio::new().unwrap();
    let rst = gpio.get(rst).unwrap().into_output();
    let dc = gpio.get(dc).unwrap().into_output();
    let mut backlight = gpio.get(bl).unwrap().into_output();

    let spi = rppal::spi::Spi::new(
        rppal::spi::Bus::Spi0,
        rppal::spi::SlaveSelect::Ss0,
        60_000_000,
        rppal::spi::Mode::Mode0,
    )
    .unwrap();
    let di = SPIInterfaceNoCS::new(spi, dc);
    let mut delay = rppal::hal::Delay::new();
    let display = mipidsi::Builder::st7735s(di)
        .with_display_size(size.0, size.1)
        .with_orientation(mipidsi::Orientation::Landscape(true))
        .with_invert_colors(mipidsi::ColorInversion::Normal)
        .with_color_order(mipidsi::ColorOrder::Bgr)
        .init(&mut delay, Some(rst))
        .unwrap();

    backlight.set_high();
    return display;
}
