use core::prelude::*;
use core::slice;
use collections::Vec;

extern {
	fn stm32_delay(millis: u32);
	fn usart2_send_string(str: *const u8, len: u16);
	fn usart2_send_byte(byte: u8);
	fn usart2_try_get_byte() -> i16;
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
}