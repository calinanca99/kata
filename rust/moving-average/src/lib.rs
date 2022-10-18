use conv::ValueFrom;

/// Returns a vector whose elements represent the arithmetic mean of each n
/// consecutive elements from `array` (i.e.: a window).
///
///
/// ## Notes
/// - If the window size is larger than the length of the array, the window size
/// becomes the length of the array => MIN(`n`, `array.len()`) is used
/// as window size.
///
/// - If the input array is empty `None` is returned.
///
/// - If the window size cannot be safely converted to `f64`
///  then `None` is returned.
///
/// - If one of the sums of a window overflows then the average for that window is
/// infinity (`f64::INFINITY`).
pub fn moving_average(array: &[f64], n: usize) -> Option<Vec<f64>> {
    let window_size = usize::min(n, array.len());

    if array.is_empty() || f64::value_from(window_size).is_err() {
        None
    } else {
        Some(
            array
                .windows(window_size)
                .map(|window| average(window, window_size))
                .collect::<Vec<_>>(),
        )
    }
}

fn average(window: &[f64], window_size: usize) -> f64 {
    // `unwrap()` is safe since condition has been checked previously
    let n = f64::value_from(window_size).unwrap();
    let sum = window.iter().sum::<f64>();

    sum / n
}

#[cfg(test)]
mod tests {
    use crate::moving_average;

    #[test]
    fn computes_moving_average_with_arbitrary_window_size() {
        // Arrange
        let array = [1.0, 2.0, 3.0, 4.0, 5.0];
        let window = 4;

        // Act
        let sut = moving_average(&array, window);

        // Assert
        let expected = vec![2.5, 3.5];
        assert_eq!(expected, sut.unwrap());
    }

    #[test]
    fn compute_moving_average_with_window_1_returns_the_same_array() {
        // Arrange
        let array = [1.0, 2.0, 3.0, 4.0, 5.0];
        let window = 1;

        // Act
        let sut = moving_average(&array, window);

        // Assert
        let expected = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert_eq!(expected, sut.unwrap());
    }

    #[test]
    fn returns_none_when_array_is_empty() {
        // Arrange
        let array = [];
        let window = 5;

        // Act
        let sut = moving_average(&array, window);

        // Assert
        assert!(sut.is_none())
    }

    // Test fails due to the following error:
    //
    // `error: values of the type `[f64; 18446744073709551615]` are too big for the current architecture`
    //
    // #[test]
    // fn returns_none_when_window_size_cannot_be_converted_to_f64() {
    //     // Arrange
    //     let array = [0.0; usize::MAX];
    //     let window = usize::MAX;

    //     // Act
    //     let sut = moving_average(&array, window);

    //     // Assert
    //     assert!(sut.is_none())
    // }

    #[test]
    fn uses_length_of_array_as_window_size_when_window_size_is_greater_than_length_of_the_array() {
        // Arrange
        let array = [1.0, 2.0, 3.0, 4.0, 5.0];
        let window = 9;

        // Act
        let sut = moving_average(&array, window);

        // Assert
        let expected = vec![3.0];
        assert_eq!(expected, sut.unwrap())
    }

    #[test]
    fn returns_inf_when_sum_of_a_windows_overflows() {
        // Arrange
        let array = [f64::MAX; 3];
        let window = 2;

        // Act
        let sut = moving_average(&array, window);

        // Assert
        let expected = vec![f64::INFINITY, f64::INFINITY];
        assert_eq!(expected, sut.unwrap());
    }
}
