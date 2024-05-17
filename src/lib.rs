//! # `pack_bools`: an easy way to pack all bools in your struct
//!
//!
//! `pack_bools` transforms structs with boolean fields into a struct containing an integer with bit flags for each boolean
//! value:
//!
//! ```rust
//! use pack_bools::pack_bools;
//!
//! #[pack_bools]
//! #[derive(Debug, Clone)]
//! struct Config<'a> {
//!     output_name: &'a str,
//!     verbose: bool,
//!     pub use_colors: bool,
//!     original_file: &'a std::path::Path,
//!     legacy_mode: bool
//! }
//! ```
//!
//! gets transformed into something like this:
//!
//! ```rust
//! #[derive(Debug, Clone)]
//! struct Config<'a> {
//!     output_name: &'a str,
//!     original_file: &'a std::path::Path,
//!     packed_bools: u8
//! }
//!
//! impl<'a> Config<'a> {
//!     fn get_verbose(&self) -> bool {
//!         self.packed_bools & 1 << 0 != 0
//!     }
//!
//!     fn set_verbose(&mut self, value: bool) {
//!         let val = self.packed_bools;
//!         self.packed_bools = if value {
//!             val | 1 << 0
//!         } else {
//!             val & !(1 << 1)
//!         };
//!     }
//!
//!     pub fn get_use_colors(&self) -> bool {
//!         self.packed_bools & 1 << 0 != 0
//!     }
//!
//!     pub fn set_use_colors(&mut self, value: bool) {
//!         let val = self.packed_bools;
//!         self.packed_bools = if value {
//!             val | 1 << 0
//!         } else {
//!             val & !(1 << 1)
//!         };
//!     }
//!
//!     /* getters and setters for legacy_mode omitted */
//! }
//! ```
//!
//! ## Usage
//!
//! Simply run `cargo add pack_bools` in your project directory, `use pack_bools::pack_bools;` and add the `#[pack_bools]`
//! macro on top of your struct. By default, this will behave as the example above: it will replace all fields of type
//! `bool` with a single numeric field `packed_bools` and add getters and setters for each field. By default, both the
//! getter and setter will inherit their visibility from the field, so if the field is declared `pub(super)`, the getters
//! and setters will too.
//!
//! There are lots of customization available, by adding options to the `#[pack_bools(...)]` attribute. Additionally, every
//! boolean field can have a `#[pack_bools(...)]` attribute with additional options.
//!
//! Global options available when using `#[pack_bools(...)]` on a struct:
//!
//! * `#[pack_bools(getters = [vis] <name>)]` changes the name (and possibly visibility) of the getters. Use
//!   `%` as a substitution for the field name. Using `#[pack_bools(getters = pub get_field_%)]` will make all getters
//!   public named `get_field_<field-name>`. The visibility modifier is optional. Aliased as `get`/`getter`. For setters,
//!   use `#[pack_bools(set/setter/setters = ...)]` instead.
//! * `#[pack_bools(no_getters)]` will not generate getters (aliased as `no_get`/`no_getter`), similarly
//!   `#[pack_bools(no_set/no_setter/no_setters)]` will not generate setters.
//! * `#[pack_bools(type = u16)]` will use `u16` as the data type for the bit flags. Available options are `u8`/`u16`/`u32`/
//!   `u64`/`u128`/`auto`, where `auto` (the default option) automatically use the smallest of those types that can fit all
//!   the bools in the struct.
//! * `#[pack_bools(field = <name>)]` will set the name of the field containing the bitflags, by default `packed_bools`.
//! * `#[pack_bools(inline)]` will use the inline pattern for the bitflag field, i.e. create fields of the pattern
//!   `packed_bools: u8`. This is the default option. Compare to `newtype` below.
//! * `#[pack_bools(newtype [= name])]` will make a new single-valued tuple struct for holding the bitflags, similar to
//!   `struct MyStructPackedBools(u8);`. If a name is specified, the newtype struct will be defined with that name,
//!   otherwise `PackedBools` will be suffixed to the name of the struct. The example at the top of this document,
//!   with `#[pack_bools(newtype)]` will be compiled into:
//!
//! ```rust
//! #[derive(Debug, Clone)]
//! struct Config<'a> {
//!     output_name: &'a str,
//!     original_file: &'a std::path::Path,
//!     packed_bools: ConfigPackedBools
//! }
//!
//! #[derive(Copy, Clone, Debug)]
//! #[repr(transparent)]
//! struct ConfigPackedBools(u8);
//!
//! impl<'a> Config<'a> {
//!     fn get_verbose(&self) -> bool {
//!         self.packed_bools.0 & 1 << 0 != 0
//!     }
//!
//!     fn set_verbose(&mut self, value: bool) {
//!         let val = self.packed_bools.0;
//!         self.packed_bools.0 = if value {
//!             val | 1 << 0
//!         } else {
//!             val & !(1 << 1)
//!         };
//!     }
//!
//!     /* additional getters and setters omitted */
//! }
//! ```
//!
//! You may add the `#[pack_bools(...)]` attribute on fields of type `bool` to configure the output of that specific field.
//! Available options are:
//!
//! * `#[pack_bools(skip)]` excludes that field from being packed with the other bools.
//! * `#[pack_bools(getter = [vis] <name>)` changes the name (and possibly visibility) of the getter to that field.
//!   `#[pack_bools(getter = pub debug_mode)]` added to a field `debug: bool` will create a getter like
//!   `pub fn debug_mode(&self) -> bool { ... }`. Aliased as `get`. For setters, use
//!   `#[pack_bools(set/setter = [vis] <name>)]`.
//! * `#[pack_bools(no_getter)]` skips generating a getter for that field. Aliased as `no_get`. For setters, use
//!   `#[pack_bools(no_set/no_setter)]`.
//! * `#[pack_bools(default = <true/false>)]` sets the default value for the field. If set to `true`, the `newtype` pattern
//!   must be used, but then a `impl Default` will be generated for that newtype with this field set to `true`. If all other
//!   fields of the struct has appropriate default values, this will allow you to use `#[derive(Default)]` on the struct,
//!   while having some boolean values set to `true`. Defaults to `false`.
use proc_macro::TokenStream;

use syn::{parse_macro_input, ItemStruct};

use self::pack_bools::config::GlobalConfig;

mod pack_bools;

#[proc_macro_attribute]
/// Packs the bools in this struct into one numeric field
pub fn pack_bools(config: TokenStream, tokens: TokenStream) -> TokenStream {
    let item_struct = parse_macro_input!(tokens as ItemStruct);
    let config = parse_macro_input!(config as GlobalConfig);
    pack_bools::pack_bools(config, item_struct).into()
}
