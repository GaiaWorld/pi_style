#![feature(if_let_guard)]
#![feature(adt_const_params)]

#[macro_use]
extern crate serde;
#[macro_use]
extern crate derive_deref;
#[macro_use]
extern crate pi_enum_default_macro;
extern crate lazy_static;
extern crate paste;

pub mod style;
pub mod style_parse;
pub mod style_type;

