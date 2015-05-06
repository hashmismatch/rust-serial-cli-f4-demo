extern crate core;
use core::prelude::*;


#[doc(hidden)]
#[cfg(test)]
#[no_mangle]
pub extern fn breakpoint() { unimplemented!() }

/// Call the debugger.
#[cfg(not(test))]
#[no_mangle]
pub extern fn breakpoint() {
	unsafe { asm!("bkpt") }
}

/// Call the debugger and halts execution.
#[no_mangle]
pub extern fn abort() -> ! {
		breakpoint();
		loop {}
}

#[doc(hidden)]
#[no_mangle]
pub extern fn __aeabi_unwind_cpp_pr0() {
		abort();
}

#[doc(hidden)]
#[no_mangle]
pub extern fn __aeabi_unwind_cpp_pr1() {
		abort();
}

#[doc(hidden)]
#[no_mangle]
pub extern fn get_eit_entry() {
		abort();
}

#[cfg(not(test))]
#[inline(always)]
/// NOP instruction
pub fn nop() {
	unsafe { asm!("nop" :::: "volatile"); }
}

#[cfg(test)]
/// NOP instruction (mock)
pub fn nop() {
}

#[cfg(not(test))]
#[inline(always)]
/// WFI instruction
pub fn wfi() {
	unsafe { asm!("wfi" :::: "volatile"); }
}

#[cfg(test)]
/// WFI instruction (mock)
pub fn wfi() {
}

#[lang = "panic_fmt"]
fn panic_fmt(_: core::fmt::Arguments, _: &(&'static str, usize)) -> !  {
	loop {}
}
#[lang = "stack_exhausted"]
extern "C" fn stack_exhausted() { }
#[lang = "eh_personality"]
extern "C" fn eh_personality() { }


// Memory allocator support, via C's stdlib


#[repr(u8)]
pub enum c_void {
    __variant1,
    __variant2,
}

extern {
    pub fn malloc(size: u32) -> *mut c_void;
    pub fn realloc(p: *mut c_void, size: u32) -> *mut c_void;
    pub fn free(p: *mut c_void);	
}

#[no_mangle]
pub unsafe extern fn rust_allocate(size: usize, align: usize) -> *mut u8 {
	malloc(size as u32) as *mut u8
}

#[no_mangle]
pub unsafe extern fn rust_deallocate(ptr: *mut u8, old_size: usize, align: usize) {
	free(ptr as *mut c_void);
}

#[no_mangle]
pub unsafe extern fn rust_reallocate(ptr: *mut u8, old_size: usize, size: usize, align: usize) -> *mut u8 { 
	realloc(ptr as *mut c_void, size as u32) as *mut u8
}
