#![feature(plugin_registrar)]

extern crate bindgen;
extern crate rustc;
extern crate syntax;

use rustc::plugin::Registry;

mod macro;

#[doc(hidden)]
#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("bindgen", macro::bindgen_macro);
}
