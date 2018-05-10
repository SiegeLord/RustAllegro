use ffi::*;
use std::ffi::{CStr, CString};
use std::ptr;

/**
Allegro configuration.

Wraps ALLEGRO_CONFIG.
*/
pub struct Config
{
	allegro_config: *mut ALLEGRO_CONFIG,
	owned: bool,
}

impl Config
{
	/// Creates an empty configuration.
	pub fn new() -> Config
	{
		unsafe { Config::wrap(al_create_config(), true) }
	}

	pub unsafe fn wrap(config: *mut ALLEGRO_CONFIG, own: bool) -> Config
	{
		Config {
			allegro_config: config,
			owned: own,
		}
	}

	/// Loads a config from a path.
	pub fn load(path: &str) -> Result<Config, ()>
	{
		let path = CString::new(path.as_bytes()).unwrap();
		unsafe {
			let allegro_config = al_load_config_file(path.as_ptr());
			if allegro_config.is_null()
			{
				Err(())
			}
			else
			{
				Ok(Config::wrap(allegro_config, true))
			}
		}
	}

	/// Merge two configs into 1.
	pub fn merge(cfg1: &Config, cfg2: &Config) -> Config
	{
		unsafe { Config::wrap(al_merge_config(cfg1.allegro_config, cfg2.allegro_config), true) }
	}

	/// Returns the wrapped ALLEGRO_CONFIG.
	pub fn get_allegro_config(&self) -> *mut ALLEGRO_CONFIG
	{
		self.allegro_config
	}

	/// Loads a config from a path.
	pub fn save(&self, path: &str) -> Result<(), ()>
	{
		let path = CString::new(path.as_bytes()).unwrap();
		let ret = unsafe { al_save_config_file(path.as_ptr(), self.allegro_config) };
		if ret != 0
		{
			Ok(())
		}
		else
		{
			Err(())
		}
	}

	/// Adds an empty section with the specified name.
	pub fn add_section(&mut self, name: &str)
	{
		let name = CString::new(name.as_bytes()).unwrap();
		unsafe {
			al_add_config_section(self.allegro_config, name.as_ptr());
		}
	}

	/// Removes a section with the specified name.
	pub fn remove_section(&mut self, name: &str) -> bool
	{
		let name = CString::new(name.as_bytes()).unwrap();
		unsafe { al_remove_config_section(self.allegro_config, name.as_ptr()) != 0 }
	}

	/// Sets the value of a key in a section.
	pub fn set_value(&mut self, section: &str, key: &str, value: &str)
	{
		let section = CString::new(section.as_bytes()).unwrap();
		let key = CString::new(key.as_bytes()).unwrap();
		let value = CString::new(value.as_bytes()).unwrap();
		unsafe {
			al_set_config_value(self.allegro_config, section.as_ptr(), key.as_ptr(), value.as_ptr());
		}
	}

	/// Removes a key with the specified name.
	pub fn remove_key(&mut self, section: &str, key: &str) -> bool
	{
		let section = CString::new(section.as_bytes()).unwrap();
		let key = CString::new(key.as_bytes()).unwrap();
		unsafe { al_remove_config_key(self.allegro_config, section.as_ptr(), key.as_ptr()) != 0 }
	}

	/// Adds a comment to a section.
	pub fn add_comment(&mut self, section: &str, comment: &str)
	{
		let section = CString::new(section.as_bytes()).unwrap();
		let comment = CString::new(comment.as_bytes()).unwrap();
		unsafe {
			al_add_config_comment(self.allegro_config, section.as_ptr(), comment.as_ptr());
		}
	}

	/// Gets a value from a section.
	pub fn get_value(&self, section: &str, key: &str) -> Option<String>
	{
		let section = CString::new(section.as_bytes()).unwrap();
		let key = CString::new(key.as_bytes()).unwrap();
		unsafe {
			let value = al_get_config_value(self.allegro_config, section.as_ptr(), key.as_ptr());
			if value.is_null()
			{
				None
			}
			else
			{
				Some(CStr::from_ptr(value).to_string_lossy().into_owned())
			}
		}
	}

	/// Merge in sections from a different config.
	pub fn merge_from(&mut self, source: &Config)
	{
		unsafe {
			al_merge_config_into(self.allegro_config, source.allegro_config);
		}
	}

	/// Returns an iterator over all the sections in the config. The first
	/// returned section will typically be the root section, even if it's empty.
	pub fn sections<'l>(&'l self) -> ConfigSection<'l>
	{
		let mut config_section = ConfigSection {
			_config: self,
			config_section: ptr::null_mut(),
			next_section: None,
		};
		unsafe {
			let next_section = al_get_first_config_section(self.allegro_config, &mut config_section.config_section);
			if !next_section.is_null()
			{
				config_section.next_section = Some(CStr::from_ptr(next_section).to_string_lossy().into_owned());
			}
		}
		config_section
	}

	/// Returns an iterator over all the entries in a particular section.
	pub fn keys<'l>(&'l self, section: &str) -> ConfigEntry<'l>
	{
		let section = CString::new(section.as_bytes()).unwrap();
		let mut config_entry = ConfigEntry {
			_config: self,
			config_entry: ptr::null_mut(),
			next_key: None,
		};
		unsafe {
			let next_key = al_get_first_config_entry(self.allegro_config, section.as_ptr(), &mut config_entry.config_entry);
			if !next_key.is_null()
			{
				config_entry.next_key = Some(CStr::from_ptr(next_key).to_string_lossy().into_owned());
			}
		};
		config_entry
	}
}

impl Clone for Config
{
	// Sadly this nukes the comments.
	fn clone(&self) -> Config
	{
		let mut ret = Config::new();
		ret.merge_from(self);
		ret
	}
}

impl Drop for Config
{
	fn drop(&mut self)
	{
		if self.owned
		{
			unsafe {
				al_destroy_config(self.allegro_config);
			}
		}
	}
}

unsafe impl Send for Config {}
unsafe impl Sync for Config {}

/**
Configuration section. Used for iterating through all the sections of a configuration.
 */
pub struct ConfigSection<'l>
{
	_config: &'l Config,
	config_section: *mut ALLEGRO_CONFIG_SECTION,
	next_section: Option<String>,
}

impl<'l> Iterator for ConfigSection<'l>
{
	type Item = String;

	fn next(&mut self) -> Option<String>
	{
		let ret = self.next_section.take();
		if ret.is_some()
		{
			self.next_section = unsafe {
				let next_section = al_get_next_config_section(&mut self.config_section as *mut _);
				if !next_section.is_null()
				{
					Some(CStr::from_ptr(next_section).to_string_lossy().into_owned())
				}
				else
				{
					None
				}
			};
		}
		ret
	}
}

/**
Configuration entry. Used for iterating through all the entries of a configuration.
 */
pub struct ConfigEntry<'l>
{
	_config: &'l Config,
	config_entry: *mut ALLEGRO_CONFIG_ENTRY,
	next_key: Option<String>,
}

impl<'l> Iterator for ConfigEntry<'l>
{
	type Item = String;

	fn next(&mut self) -> Option<String>
	{
		let ret = self.next_key.take();
		if ret.is_some()
		{
			self.next_key = unsafe {
				let next_key = al_get_next_config_entry(&mut self.config_entry as *mut _);
				if !next_key.is_null()
				{
					Some(CStr::from_ptr(next_key).to_string_lossy().into_owned())
				}
				else
				{
					None
				}
			};
		}
		ret
	}
}
