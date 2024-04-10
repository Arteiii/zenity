//! Collection of helper functions and classes related to colors

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
