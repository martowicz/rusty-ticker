#![no_std]
#![no_main]

mod joystick;

use rtt_target::{rtt_init_print, rprintln};
use panic_halt as _;
use cortex_m_rt::entry;
use cortex_m::asm;
use stm32f4xx_hal::{
    adc, i2c::I2c, pac, prelude::*
};


use lcd_lcm1602_i2c::sync_lcd::Lcd;
use core::fmt::Write as FmtWrite;


#[entry]
fn main() -> ! {
    rtt_init_print!();
    
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(84.MHz()).freeze();

    let gpioa = dp.GPIOA.split();
    let gpiob = dp.GPIOB.split();

    let mut delay = cp.SYST.delay(&clocks);

    let mut buzzer = gpioa.pa5.into_push_pull_output();

    let adc = adc::Adc::adc1(dp.ADC1, true, adc::config::AdcConfig::default());

    let scl = gpiob.pb6.into_alternate_open_drain::<4>();
    let sda = gpiob.pb7.into_alternate_open_drain::<4>();

    let mut i2c = I2c::new(dp.I2C1, (scl, sda), 100.kHz(), &clocks);

    
    let mut lcd = Lcd::new(&mut i2c, &mut delay)
    .with_address(0x27) 
    .with_cursor_on(true) 
    .with_rows(1) 
    .init().unwrap();


    let x = gpioa.pa1.into_analog();
    let y = gpioa.pa2.into_analog();
    let sw = gpioa.pa3.into_pull_up_input(); 


    let mut joystick = joystick::Joystick::new(x,y,sw,adc);


    asm::delay(84_000_000);

    let mut time_digits = [0, 0, 0, 0, 0];
    let mut cursor_position = 0;
    update_cursor(&mut lcd, &cursor_position); 
    loop {
        if joystick.up() {
            rprintln!("Up pressed");
            if cursor_position == 0 || cursor_position == 3 {
                time_digits[cursor_position as usize] = (time_digits[cursor_position as usize] + 1) % 6;
            } else {
                time_digits[cursor_position as usize] = (time_digits[cursor_position as usize] + 1) % 10;
            }
            

        }
        if joystick.down() {
            rprintln!("Down pressed");
            if cursor_position == 0 || cursor_position == 3 {
                time_digits[cursor_position as usize] = (time_digits[cursor_position as usize] + 5) % 6;
            } else {
                time_digits[cursor_position as usize] = (time_digits[cursor_position as usize] + 9) % 10;
            }
            
        }
        if joystick.next() {
            rprintln!("Next pressed");
            if cursor_position < 4 {
                cursor_position+=1;
                update_cursor(&mut lcd, &cursor_position); 
            }
            



        }
        if joystick.previous() {
            rprintln!("Previous pressed");
            if cursor_position > 0 {
                cursor_position-=1;
                update_cursor(&mut lcd, &cursor_position); 
            }
        }
        if joystick.button_pressed() {
            rprintln!("Button pressed");
            run_timer(&mut lcd, &time_digits);
            loop{
                buzzer.set_high(); 
                rprintln!("Buzzer OFF");
                asm::delay(8_200_000); 
                buzzer.set_low(); 
                rprintln!("Buzzer ON");
                asm::delay(8_200_000);
                if joystick.button_pressed() {
                    asm::delay(4_200_000);
                    break;
                }
            }
        }
        
        write_time_to_lcd(&mut lcd, &time_digits);
        update_cursor(&mut lcd, &cursor_position); 

 
  
        asm::delay(6_000_000);
    }
}



fn write_time_to_lcd<'a, I, D>(
    lcd: &mut Lcd<'a, I, D>,
    digits: &[u8; 5],
) where
    I: embedded_hal::i2c::I2c,
    D: embedded_hal::delay::DelayNs,
{
    // Format: HH:MM
    let mut buffer = heapless::String::<8>::new();
    write!(buffer, "{}{}:{}{}", digits[0], digits[1], digits[3], digits[4]).unwrap();

    lcd.clear().unwrap();
    lcd.write_str(&buffer).unwrap();
}

fn update_cursor<'a, I, D>(lcd: &mut Lcd<'a, I, D>, cursor_position: &u8)
where
    I: embedded_hal::i2c::I2c,
    D: embedded_hal::delay::DelayNs,
{
    let pos = *cursor_position;
    lcd.set_cursor(0, pos).unwrap();
}


fn run_timer<'a, I, D>(lcd: &mut Lcd<'a, I, D>, digits: &[u8; 5])
where
    I: embedded_hal::i2c::I2c,
    D: embedded_hal::delay::DelayNs,
{
    let mut minutes = digits[0] * 10 + digits[1];
    let mut seconds = digits[3] * 10 + digits[4];

    loop {
        let mut buffer = heapless::String::<8>::new();
        write!(buffer, "{:02}:{:02}", minutes, seconds).unwrap();

        lcd.clear().unwrap();
        lcd.write_str(&buffer).unwrap();

        if minutes == 0 && seconds == 0 {
            break;
        }

        asm::delay(84_000_000);

        // Zmniejsz czas
        if seconds > 0 {
            seconds -= 1;
        } else {
            if minutes > 0 {
                minutes -= 1;
                seconds = 59;
            }
        }
    }

    lcd.clear().unwrap();
    lcd.write_str("00:00").unwrap();
    asm::delay(84_000_000);
    lcd.clear().unwrap();
    lcd.write_str("Koniec").unwrap();



}