// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

#![crate_name = "allegro_color"]
#![crate_type = "lib"]
#![allow(non_upper_case_globals)]

extern crate allegro;
extern crate allegro_color_sys;
extern crate allegro_util;
extern crate libc;

use allegro::Color;
use allegro_color_sys::*;
use libc::*;
use std::ffi::{CStr, CString};

pub trait ColorAddonExtensions
{
	#[doc(hidden)]
	fn get_color(&self) -> Color;

	fn from_hsv(hue: f32, saturation: f32, value: f32) -> Color
	{
		let mut r: c_float = 0.0;
		let mut g: c_float = 0.0;
		let mut b: c_float = 0.0;

		unsafe {
			al_color_hsv_to_rgb(hue as c_float, saturation as c_float, value as c_float, &mut r, &mut g, &mut b);
		}

		Color::from_rgb_f(r as f32, g as f32, b as f32)
	}

	fn from_hsl(hue: f32, saturation: f32, lightness: f32) -> Color
	{
		let mut r: c_float = 0.0;
		let mut g: c_float = 0.0;
		let mut b: c_float = 0.0;

		unsafe {
			al_color_hsl_to_rgb(hue as c_float, saturation as c_float, lightness as c_float, &mut r, &mut g, &mut b);
		}

		Color::from_rgb_f(r as f32, g as f32, b as f32)
	}

	fn from_css_name(name: &str) -> Color
	{
		let mut r: c_float = 0.0;
		let mut g: c_float = 0.0;
		let mut b: c_float = 0.0;
		let name = CString::new(name.as_bytes()).unwrap();

		unsafe {
			al_color_name_to_rgb(name.as_ptr(), &mut r, &mut g, &mut b);
		}

		Color::from_rgb_f(r as f32, g as f32, b as f32)
	}

	fn from_cmyk(cyan: f32, magenta: f32, yellow: f32, key: f32) -> Color
	{
		let mut r: c_float = 0.0;
		let mut g: c_float = 0.0;
		let mut b: c_float = 0.0;

		unsafe {
			al_color_cmyk_to_rgb(
				cyan as c_float,
				magenta as c_float,
				yellow as c_float,
				key as c_float,
				&mut r,
				&mut g,
				&mut b,
			);
		}

		Color::from_rgb_f(r as f32, g as f32, b as f32)
	}

	fn from_yuv(y: f32, u: f32, v: f32) -> Color
	{
		let mut r: c_float = 0.0;
		let mut g: c_float = 0.0;
		let mut b: c_float = 0.0;

		unsafe {
			al_color_yuv_to_rgb(y as c_float, u as c_float, v as c_float, &mut r, &mut g, &mut b);
		}

		Color::from_rgb_f(r as f32, g as f32, b as f32)
	}

	fn from_html_hex(html_hex: &str) -> Color
	{
		let mut r: c_float = 0.0;
		let mut g: c_float = 0.0;
		let mut b: c_float = 0.0;
		let html_hex = CString::new(html_hex.as_bytes()).unwrap();

		unsafe {
			al_color_html_to_rgb(html_hex.as_ptr(), &mut r, &mut g, &mut b);
		}

		Color::from_rgb_f(r as f32, g as f32, b as f32)
	}

	fn to_hsv(&self) -> (f32, f32, f32)
	{
		let mut h: c_float = 0.0;
		let mut s: c_float = 0.0;
		let mut v: c_float = 0.0;
		let (r, g, b) = self.get_color().to_rgb_f();

		unsafe {
			al_color_rgb_to_hsv(r as c_float, g as c_float, b as c_float, &mut h, &mut s, &mut v);
		}

		(h as f32, s as f32, v as f32)
	}

	fn to_hsl(&self) -> (f32, f32, f32)
	{
		let mut h: c_float = 0.0;
		let mut s: c_float = 0.0;
		let mut l: c_float = 0.0;
		let (r, g, b) = self.get_color().to_rgb_f();

		unsafe {
			al_color_rgb_to_hsl(r as c_float, g as c_float, b as c_float, &mut h, &mut s, &mut l);
		}

		(h as f32, s as f32, l as f32)
	}

	fn to_css_name(&self) -> String
	{
		let (r, g, b) = self.get_color().to_rgb_f();

		unsafe {
			let name = al_color_rgb_to_name(r as c_float, g as c_float, b as c_float);
			CStr::from_ptr(name).to_string_lossy().into_owned()
		}
	}

	fn to_cmyk(&self) -> (f32, f32, f32, f32)
	{
		let mut c: c_float = 0.0;
		let mut m: c_float = 0.0;
		let mut y: c_float = 0.0;
		let mut k: c_float = 0.0;
		let (r, g, b) = self.get_color().to_rgb_f();

		unsafe {
			al_color_rgb_to_cmyk(r as c_float, g as c_float, b as c_float, &mut c, &mut y, &mut m, &mut k);
		}

		(c as f32, y as f32, m as f32, k as f32)
	}

	fn to_yuv(&self) -> (f32, f32, f32)
	{
		let mut y: c_float = 0.0;
		let mut u: c_float = 0.0;
		let mut v: c_float = 0.0;
		let (r, g, b) = self.get_color().to_rgb_f();

		unsafe {
			al_color_rgb_to_yuv(r as c_float, g as c_float, b as c_float, &mut y, &mut u, &mut v);
		}

		(y as f32, u as f32, v as f32)
	}
}

impl ColorAddonExtensions for Color
{
	fn get_color(&self) -> Color
	{
		*self
	}
}

pub fn get_color_addon_version() -> i32
{
	unsafe { al_get_allegro_color_version() as i32 }
}
