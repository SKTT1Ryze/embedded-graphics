//! `Drawable` trait and helpers
use crate::{draw_target::DrawTarget, geometry::Point, pixelcolor::PixelColor};

/// Marks an object as "drawable". Must be implemented for all graphics objects
///
/// The `Drawable` trait describes how a particular graphical object is drawn. A `Drawable` object
/// can define its `draw` method as a collection of graphical primitives or as an iterator
/// over pixels being rendered with [`DrawTarget`]'s [`draw_iter`] method.
///
/// ```rust
/// use embedded_graphics::{
///     mono_font::{ascii::Font6x9, MonoTextStyle},
///     pixelcolor::{BinaryColor, PixelColor, Rgb888},
///     prelude::*,
///     primitives::{Rectangle, PrimitiveStyle},
///     text::Text,
/// };
///
/// struct Button<'a, C: PixelColor> {
///     top_left: Point,
///     size: Size,
///     bg_color: C,
///     fg_color: C,
///     text: &'a str,
/// }
///
/// impl<C> Drawable for Button<'_, C>
/// where
///     C: PixelColor + From<BinaryColor>,
/// {
///     type Color = C;
///     type Output = ();
///
///     fn draw<D>(&self, display: &mut D) -> Result<Self::Output, D::Error>
///     where
///         D: DrawTarget<Color = C>,
///     {
///         Rectangle::new(self.top_left, self.size)
///             .into_styled(PrimitiveStyle::with_fill(self.bg_color))
///             .draw(display)?;
///
///         Text::new(self.text, Point::new(6, 13))
///             .into_styled(MonoTextStyle::new(Font6x9, self.fg_color))
///             .draw(display)
///     }
/// }
///
/// let mut button = Button {
///     top_left: Point::zero(),
///     size: Size::new(60, 20),
///     bg_color: Rgb888::RED,
///     fg_color: Rgb888::BLUE,
///     text: "Click me!",
/// };
///
/// # use embedded_graphics::mock_display::MockDisplay;
/// # let mut display = MockDisplay::default();
/// # display.set_allow_overdraw(true);
/// button.draw(&mut display)?;
/// # Ok::<(), core::convert::Infallible>(())
/// ```
///
/// [`DrawTarget`]: draw_target/trait.DrawTarget.html
/// [`draw_iter`]: draw_target/trait.DrawTarget.html#tymethod.draw_iter
pub trait Drawable {
    /// The pixel color type.
    type Color: PixelColor;

    // TODO: Remove `ignore` from example code when the `Text` drawable is updated
    // CC https://github.com/embedded-graphics/embedded-graphics/pull/552#discussion_r576955191
    /// The return type of the [`draw`] method.
    ///
    /// The `Output` type can be used to return results and values produced from the drawing of the
    /// current item. For example, rendering two differently styled text items next to each other
    /// can make use of a returned value, allowing the next text to be positioned after the first:
    ///
    /// ```rust,ignore
    /// let next_point = Text::new("Label ", Point::new(10, 20))
    ///     .into_styled(label_style)
    ///     .draw(&mut display)?;
    ///
    /// Text::new("Value", next_point).into_styled(value_style).draw(&mut display)?;
    /// ```
    ///
    /// Use `()` if no value should be returned.
    type Output;

    /// Draw the graphics object using the supplied DrawTarget.
    fn draw<D>(&self, display: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Self::Color>;
}

/// A single pixel.
///
/// `Pixel` objects are used to specify the position and color of drawn pixels.
///
/// # Examples
///
/// The [`Drawable`] trait is implemented for `Pixel` which allows single pixels
/// to be drawn to a [`DrawTarget`]:
/// ```
/// use embedded_graphics::{pixelcolor::BinaryColor, prelude::*};
/// # use embedded_graphics::mock_display::MockDisplay;
/// # let mut display = MockDisplay::new();
///
/// Pixel(Point::new(1, 2), BinaryColor::On).draw(&mut display)?;
/// # Ok::<(), core::convert::Infallible>(())
/// ```
///
/// Iterators with `Pixel` items can also be drawn:
///
/// ```
/// use embedded_graphics::{pixelcolor::BinaryColor, prelude::*};
/// # use embedded_graphics::mock_display::MockDisplay;
/// # let mut display = MockDisplay::new();
///
/// (0..32)
///     .map(|i| Pixel(Point::new(i, i * 2), BinaryColor::On))
///     .draw(&mut display)?;
/// # Ok::<(), core::convert::Infallible>(())
/// ```
///
/// [`Drawable`]: trait.Drawable.html
/// [`DrawTarget`]: draw_target/trait.DrawTarget.html
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Pixel<C>(pub Point, pub C)
where
    C: PixelColor;

impl<C> Drawable for Pixel<C>
where
    C: PixelColor,
{
    type Color = C;
    type Output = ();

    fn draw<D>(&self, display: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = C>,
    {
        display.draw_iter(core::iter::once(*self))
    }
}

#[cfg(test)]
mod tests {
    // NOTE: `crate` cannot be used here due to circular dependency resolution behavior.
    use embedded_graphics::{
        geometry::Point, mock_display::MockDisplay, pixelcolor::BinaryColor, Drawable, Pixel,
    };

    #[test]
    fn draw_pixel() {
        let mut display = MockDisplay::new();
        Pixel(Point::new(0, 0), BinaryColor::On)
            .draw(&mut display)
            .unwrap();
        Pixel(Point::new(2, 1), BinaryColor::On)
            .draw(&mut display)
            .unwrap();
        Pixel(Point::new(1, 2), BinaryColor::On)
            .draw(&mut display)
            .unwrap();

        display.assert_pattern(&[
            "#  ", //
            "  #", //
            " # ", //
        ]);
    }
}
