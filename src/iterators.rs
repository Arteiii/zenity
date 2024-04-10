//! collection of reusable iterators

/// iterates over a collection of vectors in a balanced manner based on indices
///
/// it allows users to retrieve values from each vector at specified index positions
/// if a vector is shorter than the specified index, it will wrap around and start again from the beginning
///
/// # Arguments
///
/// * `index` - the index position to retrieve values from
/// * `vectors` - a slice reference to a collection of vectors
///
/// # Returns
///
/// a vector containing the values from each vector at the specified index position
///
pub(crate) fn balanced_iterator<T>(index: usize, vectors: &[Vec<T>]) -> Vec<Option<&T>> {
    vectors
        .iter()
        .map(|vec| {
            if !vec.is_empty() {
                let modulo_index = index % vec.len();
                vec.get(modulo_index)
            } else {
                None
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_balanced_iterator() {
        let vec1 = vec![10, 20, 30];
        let vec2 = vec![100];
        let vec3 = vec![1, 2, 3, 4, 5, 6];

        let binding = [vec1, vec2, vec3];
        let result = balanced_iterator(2, &binding);

        assert_eq!(result, vec![Some(&30), Some(&100), Some(&3)]);
    }

    #[test]
    fn test_empty_vectors() {
        let empty_vec: Vec<Vec<i32>> = Vec::new();
        let result = balanced_iterator(2, &empty_vec);
        assert_eq!(result, Vec::<Option<&i32>>::new());
    }

    #[test]
    fn test_index_out_of_bounds() {
        let vec1 = vec![10, 20, 30];
        let vec2 = vec![100];
        let vec3 = vec![1, 2, 3, 4, 5, 6];

        let binding = [vec1, vec2, vec3];
        let result = balanced_iterator(10, &binding);

        assert_eq!(result, vec![Some(&20), Some(&100), Some(&5)]);
    }
}
