use core::prelude::*;
use core::slice;
use collections::Vec;

extern {
	fn stm32_delay(millis: u32);
	fn usart2_send_string(str: *const u8, len: u16);
	fn usart2_send_byte(byte: u8);
	fn usart2_try_get_byte() -> i16;
	fn stm32_toggle_led(led: u8);
	fn stm32_enable_led(led: u8);
	fn stm32_disable_led(led: u8);
}

pub fn delay(millis: u32) {
	unsafe {
		stm32_delay(millis);
	}
}


#[derive(Copy, Clone)]
pub enum STM32_USART_Device {
	USART2
}

#[derive(Copy, Clone)]
pub struct STM32_USART {
	device: STM32_USART_Device
}


impl STM32_USART {
	pub fn new(device: STM32_USART_Device) -> STM32_USART {
		STM32_USART {
			device: device
		}
	}

	pub fn print(&self, str: &str) {
		let bytes = str.bytes().collect::<Vec<u8>>();
		self.print_bytes(bytes.as_slice());
	}

	pub fn print_bytes(&self, bytes: &[u8]) {
		unsafe {
			match self.device {
				STM32_USART_Device::USART2 => usart2_send_string(bytes.as_ptr(), bytes.len() as u16)
			}
		}
	}

	pub fn println(&self, str: &str) {
		self.print(str);
		self.print("\r\n");
	}

	pub fn send_byte(&self, byte: u8) {
		unsafe {
			match self.device {
				STM32_USART_Device::USART2 => usart2_send_byte(byte)
			}
		}
	}

	pub fn try_read_byte(&self) -> Option<u8> {
		unsafe {
			let r = usart2_try_get_byte();
			if r == -1 { return None; }
			return Some(r as u8);
		}
	}
}

use cli::{CliTerminal, CliPromptTerminal};
impl CliTerminal for STM32_USART {
	fn output_line(&mut self, line: &str) {
		self.println(line);
	}
}

impl CliPromptTerminal for STM32_USART {
	fn print_bytes(&self, bytes: &[u8]) {
		self.print_bytes(bytes);
	}
}

pub enum STM32Led {
	Red,
	Green,
	Blue,
	Orange
}

impl STM32Led {
	fn to_api(&self) -> u8 {
		match *self {
			STM32Led::Red => 1,
			STM32Led::Green => 2,
			STM32Led::Blue => 3,
			STM32Led::Orange => 0
		}
	}
}

pub fn stm32_toggle(led: STM32Led) {
	unsafe {
		stm32_toggle_led(led.to_api());
	}
}

pub fn stm32_enable(led: STM32Led) {
	unsafe {
		stm32_enable_led(led.to_api());
	}
}

pub fn stm32_disable(led: STM32Led) {
	unsafe {
		stm32_disable_led(led.to_api());
	}
}