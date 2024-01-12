mod button;

pub use button::*;

use aom::Object;

pub trait Widget: std::fmt::Debug + Object {

}