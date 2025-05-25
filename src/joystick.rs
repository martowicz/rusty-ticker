use stm32f4xx_hal::{gpio::{Analog, Input}, pac, prelude::*};
use stm32f4xx_hal::adc::Adc;
use stm32f4xx_hal::gpio::gpioa::{PA1, PA2, PA3};


use panic_halt as _;

pub struct Joystick {
    pub x_pin: PA1<Analog>,  // Poprawka: Teraz nie trzeba podawać numeru pinu, to zależy od portu (PA0, PA1 itp.)
    pub y_pin: PA2<Analog>,  // Poprawka: Teraz nie trzeba podawać numeru pinu, to zależy od portu (PA0, PA1 itp.)
    pub button_pin: PA3<Input>, // Przycisk
    adc: Adc<pac::ADC1>,
}

impl Joystick {
    pub fn new(
        x: PA1<Analog>,
        y: PA2<Analog>,
        sw: PA3<Input>,
        adc: Adc<pac::ADC1>,
    ) -> Self {
        Joystick { x_pin: (x), y_pin: (y), button_pin: (sw), adc: (adc) }
    }
    pub fn read_x(&mut self) -> u16 {
        self.adc.read(&mut self.x_pin).unwrap()
    }

    pub fn read_y(&mut self) -> u16 {
        self.adc.read(&mut self.y_pin).unwrap()
    }

    pub fn button_pressed(&self) -> bool {
        !self.button_pin.is_high()  // Sprawdzenie stanu przycisku
    }

    pub fn next(&mut self) -> bool {
        let y = self.read_y();
        y<=5
    }

    pub fn previous(&mut self) -> bool {
        let y = self.read_y();
        y>=4090
    }

    pub fn up(&mut self) -> bool {
        let x = self.read_x();
        x<=5
    }

    pub fn down(&mut self) -> bool {
        let x = self.read_x();
        x>=4090
    }


}
