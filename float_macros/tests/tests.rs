#![feature(plugin)]
#![allow(unstable)]

#[plugin] #[no_link]
extern crate float_macros;

static GOLDEN_RATIO: f32 = (1.0 + sqrt!(5.0)) / 2.0;

#[test]
fn main() {
    assert_eq!(GOLDEN_RATIO, 1.618034f32);
    assert_eq!(sqrt!(2.0), 1.414214f32);
    assert_eq!(sqrt!(32.0), 5.656854f32);

}
