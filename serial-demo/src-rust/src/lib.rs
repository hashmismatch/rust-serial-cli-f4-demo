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
extern crate cli;

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
use cli::*;
use collections::string::*;
use collections::Vec;
use alloc::boxed::Box;


#[no_mangle]
pub extern fn serial_demo_main_loop() -> ! {
	let mut usart2 = STM32_USART::new(STM32_USART_Device::USART2);
	// http://wiki.bash-hackers.org/scripting/terminalcodes
	let mut usart2_cli = CliPromptAutocompleteBuffer::new("\x1b[36mclidemo\x1b[39m \x1b[1m#\x1b[0m ".to_string());

	usart2.println("");
	usart2.println("");
	usart2.println("STM32 Rust CLI demo");
	usart2_cli.print_prompt(&usart2);

	// cli commands
	let mut commands = {
		let mut cmds = Vec::new();

		{
			let help = CliCommandKeyword {	
				keyword: "help".to_string(),
				action: |line, cli| {
					cli.output_line("Hey, help here!");
				}
			};

			cmds.push(Box::new(help) as Box<CliCommand>);
		}

		{
			let some_int = CliPropertyVar {
				var_name: "some_int".to_string(),
				val_hint: "positive integer".to_string(),
				var_value: 100,
				var_output: |v| {
					v.to_string()
				},
				var_input: |v| {
					let n = v.parse::<i32>().ok();
					match n {
						Some(n) if n >= 0 => Some(n),
						_ => None
					}
				}
			};

			cmds.push(Box::new(some_int) as Box<CliCommand>);
		}

		cmds
	};


	// note: the input handling would usually be implemented in an interrupt,
	// but for demonstration purposes a loop is good enough. interrupts would
	// require at least some mutexes (when handling multiple terminals with a
    // single global CLI command set).

	loop {

		let u2_byte = usart2.try_read_byte();
		match u2_byte {
			Some(b) => {
				usart2_cli.handle_received_byte(b, &usart2.clone(), commands.as_mut_slice(), &mut usart2);
			}
			_ => {}
		}

		delay(1);
	}
}
