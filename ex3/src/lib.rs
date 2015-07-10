// 1) Implement the `saturating_scale` functions, so that it passes the tests
//    when run with `cargo test`.
//
// 2) Remove the #[cfg(skip)] lines above `brighten` and `test_brighten`, and
//    then make that test pass.


// Return the value v, scaled by the factor f: simply v*f. But, if the result is
// negative, return 0; if it's greater than 255, return 255.
//
// You'll need to use "v as f32" and "... as u8" to convert between the numeric
// types; Rust does no automatic conversions.
//
// Remember that floating-point constants must be written with a decimal point:
// 0. or 255.
pub fn saturating_scale(v: u8, f: f32) -> u8 {
    return 0;
}

#[test]
fn test_saturating_scale() {
    assert_eq!(saturating_scale(10, 0.), 0);
    assert_eq!(saturating_scale(10, 4.), 40);
    assert_eq!(saturating_scale(10, -1.), 0);
    assert_eq!(saturating_scale(10, 100.), 255);
}


#[derive(PartialEq, Eq, Debug)]
pub enum Color {
    RGB   { r: u8, g:  u8, b:  u8 },
    YCbCr { y: u8, cb: u8, cr: u8 }
}

use Color::{ RGB, YCbCr };


// Return a color equal to `c` with its brightness scaled by the factor `f`.
// For RGB colors, this means scaling each component.
// For YCbCr colors, this means scaling only the y component, and returning the
// cb and cr components unchanged.
//
// You'll need to use a match statement to handle each variant of `Color` that
// `c` might be, and then return a value of the same variant, with its
// components adjusted. Return a fresh Color; don't try to modify `c` in place.
#[cfg(skip)]
pub fn brighten(c: Color, f: f32) -> Color {
}

#[cfg(skip)]
#[test]
fn test_brighten_rgb() {
    assert_eq!(brighten(RGB { r: 20, g: 30, b: 40 }, 0.),
               RGB { r: 0, g: 0, b: 0 });
    assert_eq!(brighten(RGB { r: 20, g: 30, b: 40 }, 0.5),
               RGB { r: 10, g: 15, b: 20 });
    assert_eq!(brighten(RGB { r: 20, g: 30, b: 40 }, 1.5),
               RGB { r: 30, g: 45, b: 60 });
}

#[cfg(skip)]
#[test]
fn test_brighten_ycbcr() {
{
    let color = YCbCr { y: 80, cb: 10, cr: 100 };
    assert!(match brighten(color, 0.) {
        // Any value with a zero luma will do.
        YCbCr { y: 0, cb: _, cr: _ } => true,
        _ => false
    });

    let color2 = YCbCr { y: 80, cb: 100, cr: 200 };
    assert_eq!(brighten(color2, 2.),
               YCbCr { y: 160, cb: 100, cr: 200 });
}
