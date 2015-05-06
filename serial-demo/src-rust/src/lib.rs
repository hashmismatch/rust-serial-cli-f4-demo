#![feature(no_std)]
#![feature(macro_reexport)]
#![feature(unboxed_closures)]
#![feature(lang_items, asm)]
#![feature(convert)]

#![no_std]
#![feature(core, alloc, collections)]

#![allow(dead_code)]
#![allow(non_snake_case)]


extern crate core;
extern crate alloc;

use core::prelude::*;

pub mod runtime_support;
pub mod serial_demo_api;

#[macro_use]
#[macro_reexport(vec, format)]
pub extern crate collections;
//use collections::Vec;
//use collections::String;
//use collections::prelude::*;

//#[macro_use]
//#[macro_reexport(vec, format)]
//extern crate "collections" as core_collections;

use serial_demo_api::*;

#[no_mangle]
pub extern fn serial_demo_main_loop() -> ! {
	let usart2 = STM32_USART::new(STM32_USART_Device::USART2);
	let mut i = 1;

	loop {
		usart2.println(format!("Hello world #{}!", i).as_str());
		i += 1;

		delay(1000);
	}
}
