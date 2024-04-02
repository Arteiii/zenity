use crossterm::style;

/// combines multiple attributes into a single `style::Attributes` instance
///
/// this function takes a slice of attribute references and combines them into a single
/// `style::Attributes` instance using bitwise OR (`|`) operation
///
/// # Arguments
///
/// * `attr_list` - a slice containing references to the attributes to be combined
///
/// # Returns
///
/// A `style::Attributes` instance representing the combined attributes
///
/// # Example
///
/// ```
/// use zenity::{style::{Attribute, Attributes}, combine_attributes};
///
/// let attributes = combine_attributes(&[
///     &Attribute::Bold,
///     &Attribute::Underlined,
///     &Attribute::Italic,
/// ]);
/// ```
pub fn combine_attributes(attr_list: &[&style::Attribute]) -> style::Attributes {
    attr_list
        .iter()
        .fold(style::Attributes::default(), |acc, &attr| {
            acc | style::Attributes::from(*attr)
        })
}
