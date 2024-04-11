//! Collection of helper functions and classes related to colors
//! ```
//! use zenity::style::{Attribute, Attributes, combine_attributes, ContentStyle, StyledString};
//!
//! let attr1 = Attribute::Bold;
//! let attr2 = Attribute::Underlined;
//! let attr3 = Attribute::Italic;
//!
//! let combined_attr = combine_attributes(&[&attr1, &attr2, &attr3]);
//!
//! let style = ContentStyle {
//!     foreground_color: None,
//!     background_color: None,
//!     underline_color: None,
//!     attributes: combined_attr, // see docs on combine_attributes()
//! };
//!
//! let styled_string = StyledString {
//!     string: "example".to_string(),
//!     style,
//! };
//!
//! assert_eq!(styled_string.string, "example");
//! assert_eq!(styled_string.style.attributes.contains(Attribute::Bold), true);
//! assert_eq!(styled_string.style.attributes.contains(Attribute::Underlined), true);
//! assert_eq!(styled_string.style.attributes.contains(Attribute::Italic), true);
//! ```

pub use crossterm::style::*;

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
/// a `style::Attributes` instance representing the combined attributes
///
/// ```
/// use zenity::style::{Attribute, Attributes, combine_attributes};
///
/// let attr1 = Attribute::Bold;
/// let attr2 = Attribute::Underlined;
/// let attr3 = Attribute::Italic;
///
/// let combined_attr = combine_attributes(&[&attr1, &attr2, &attr3]);
/// #
/// # assert_eq!(combine_attributes(&[&attr1, &attr2, &attr3]), combined_attr);
/// ```
pub fn combine_attributes(attr_list: &[&Attribute]) -> Attributes {
    attr_list.iter().fold(Attributes::default(), |acc, &attr| {
        acc | Attributes::from(*attr)
    })
}

/// Represents the style of a string, including attributes and colors
///
/// represents a string along with their style
///
/// the style is a representation of ``ContentStyle``
///
/// the ``ContentStyle`` is defined like this:
/// 
/// ```
/// # use zenity::style::{Attribute, combine_attributes};
/// # let combined_attr = combine_attributes(&[&Attribute::Bold, &Attribute::Underlined]);
/// use zenity::style::ContentStyle;
/// 
/// let style = ContentStyle {
///     foreground_color: None,
///     background_color: None,
///     underline_color: None,
///     attributes: combined_attr, // see docs on combine_attributes()
/// };
/// ```
/// 
/// for more info on combined attributes look up \[combine_attributes\]
/// 
/// 
///```
/// # use zenity::style::{Attribute, combine_attributes, ContentStyle};
/// # let combined_attr = combine_attributes(&[&Attribute::Bold, &Attribute::Underlined]);
/// # let style = ContentStyle {
/// #    foreground_color: None,
/// #    background_color: None,
/// #    underline_color: None,
/// #    attributes: combined_attr,
/// # };
/// use zenity::style::StyledString;
/// 
/// let styled = StyledString {
///    string: "example".to_string(),
///    style,
/// };
/// # 
/// # assert_eq!(styled.string, "example");
/// # assert_eq!(styled.style.attributes.contains(Attribute::Bold), true);
/// # assert_eq!(styled.style.attributes.contains(Attribute::Underlined), true);
/// ```
pub struct StyledString {
    /// the string
    pub string: String,
    /// the ContentStyle to apply to the string
    pub style: ContentStyle,
}
