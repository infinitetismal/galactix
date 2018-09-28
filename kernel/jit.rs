// jit.rs
// a lisp compiler written in rust
// -------------------------------
// This is a really basic compiler written in Rust. It was designed to give nukleus
// a standard lisp implementation with a couple of goodies (native rust calling, easy-to-call aseembly),
// and to be fast. (Remember kids, speed is key).
//
// It reads the given file into a hashmap. Then, it reads each line of the hashmap into a String where it is interpeted the first time,
// giving the later stages of the compilers warnings like when assembly is going to be called, when Rust is going to be called, when a
// variable or function is going to be called, etc.
//
// The compiler then uses these warnings to easier execute the code instead of having to break the string down again.
//
//                                                      file
//                                                    /      
//                                                hashmap -  compiler
//                                                /               \
//                                              string    -   initial match function, pushes to string and gives warnings

extern crate average;
extern crate cpuio;
extern crate hashmap_core;
extern crate libm;
extern crate memadvise;
extern crate ux;

#[cfg_attr(feature = "std")]
use std::fs::File;
use std::io;
use std::io::prelude::*;

// stdlib required
mod bitwise;
mod fs;

#[cfg(nukleus_build)] mod persistence;
#[cfg(nukleus_build)] mod memory;

#![cfg_attr(not(feature = "std")]
pub const STD: i2 = 0;

#![cfg_attr(feature = "std")]
pub const STD: i2 = 1;

pub fn lisp() {
	let mut pos = 0;
	loop {
		// init file reading
		if read == 0 {
			if STD == 1 {
				let file = File::open(&filename)?;
    			let reader = io::BufReader::new(file);
	    		for line in reader.lines() {		
					let line = line?;
					file.insert(pos, line);
					pos = pos + 1;
				}
				
				read = 0;
				pos = 0;
			} else { break; }
		}
		
		pos = pos + 1;
		
		// stdlib
		// TODO: document bitwise operations
		match keyword {
			// define functions/vars
			"defun" => {
				line.push_str("defun");
				defun.push_str(pos);
				next_warning.push(pos);
			}
			"asm" => {
				line.push_str("asm");
				asm.push_str(pos);
				next_warning.push(pos);
			}
			"(" => {
				line.push_str("(");
				start.push(pos);
				next_warning.push(pos)
			}
			")" => { 
				line.push_str(")");
				end.push(pos);
				next_warning.push(pos);
			}
	
			// mathematical operations
			"-" => {
				line.push_str("-");
				next_equal_targets.push(pos);
				next_warning.push(pos);
				
			}
			"+" => {
				line.push_str("+");
				next_equal_targets.push(pos);
				next_warning.push(pos);
			}
			"/" => {
				line.push_str("/");
				next_equal_targets.push(pos);
				next_warning.push(pos);
			}
			"*" => {
				line.push_str("*");
				next_equal_targets.push(pos);
				next_warning.push(pos);
			}
			"**" => {
				line.push_str("**");
				next_equal_targets.push(pos);
				next_warning.push(pos);
			}
	
			// bitwise operations
			"&" => {
				line.push_str("&");
				next_equal_targets.push(pos);
				mext_warning.push(pos);
			}
			"|" => {
				line.push_str("|");
				next_equal_targets.push(pos);
				next_warning.push(pos);
			}
			"^" => {
				line.push_str("^");
				next_equal_targets.push(pos);
				next_warning.push(pos);
			}
			"<<" => {
				line.push_str("<<");
				next_equal_targets.push(pos);
				next_warning.push(pos);
			}
			">>" => [
				line.push_str(">>");
				next_equal_targets.push(pos);
				next_warning.push(pos);
			}
			"~" => {
				line.push_str("~");
				next_equal_targets.push(pos);
				next_warning.push(pos);
			}
		}
		
		if pos == 1 {
			let first_line = line;
		}
		
		// repeat to end of file
	}
	// close file
	// TODO
	
	let exec_line = file.get(&first_line);
	
	loop {
		if next_warning == 0 {
			if warning_type == "function" {
			
			}
		
			if warning_type == "asm" {
				unsafe {
					pub const ASM: Vec<u64> = Vec!("BEGINNING");
					call_assembler!(1);
				}
			}
			
			// if warning_type == "rust" {
			// 	  unsafe {
			//		pub const RUST: Vec<u64> = Vec!("BEGINNING");
			//      call_rustJIT!(1);
			//    }
			// }
		
			if warning_type == "next_equal_targets" {
				// math
				if type == "add" {
					if orphan == 1 {
						
					}
					
					line_result.push(res);
				}
				
				if type == "subtract" {
					if orphan == 1 {
						
					}
					
					line_result.push(res);	
				}
				
				if type == "multiply" {
					if orphan == 1 {
						
					}
					
					line_result.push(res);
				}
				
				if type == "division" {
					if orphan == 1 {
						
					}
					
					line_result.push(res);
				}
				
				if type == "power" {
					if orphan == 1 {
						
					}
					
					line_result.push(res);
				}
				
				// bitwise operations
				if type == " " {
					if orphan == 1 {
						
					}
					
					line_result.push(res);
				}
				
				if type == " " {
					if orphan == 1 {
						
					}
					
					line_result.push(res);
				}
				
				if type == " " {
					if orphan == 1 {
						
					}
					
					line_result.push(res);
				}
				
				if type == " " {
					if orphan == 1 {
						
					}
					
					line_result.push(res);
				}
		
				if type == " " {
					if orphan == 1 {
						
					}
					
					line_result.push(res);
				}
				
				if type == " " {
					if orphan == 1 {
						
					}
					
					line_result.push(res);
				}
			}
		}
	}
}
