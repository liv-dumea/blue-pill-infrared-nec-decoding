//! Output a PWM with a duty cycle of ~6% on all the channels of TIM3

#![deny(unsafe_code)]
#![deny(warnings)]
#![feature(proc_macro)]
#![no_std]

extern crate blue_pill;
extern crate cortex_m_rtfm as rtfm;

use blue_pill::{Channel, Pwm};
use blue_pill::time::Hertz;
use blue_pill::prelude::*;
use rtfm::app;

const FREQUENCY: Hertz = Hertz(1_000);

app! {
    device: blue_pill::stm32f103xx,
}

fn init(p: init::Peripherals) {
    let pwm = Pwm(p.TIM3);

    pwm.init(FREQUENCY.invert(), p.AFIO, None, p.GPIOA, p.RCC);
    let duty = pwm.get_max_duty() / 16;

    const CHANNELS: [Channel; 2] = [Channel::_1, Channel::_2];

    for c in &CHANNELS {
        pwm.set_duty(*c, duty);
    }

    for c in &CHANNELS {
        pwm.enable(*c);
        rtfm::bkpt();
    }
}

fn idle() -> ! {
    loop {
        rtfm::wfi();
    }
}
