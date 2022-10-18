/// Returns a vector whose elements represent the arithmetic mean of each n
/// consecutive elements from `array` (i.e.: a window), where n == `window_size`.
///
///
/// ## Notes
/// - If the input array is empty `None` is returned.
///
/// - If the window size is larger than the length of the array, the window size
/// becomes the length of the array.
///
/// - If one of the sums of a window overflows then the average for that window is
/// infinity (`f64::INFINITY`).
pub fn moving_average(array: &[f64], window_size: usize) -> Option<Vec<f64>> {
    if array.is_empty() {
        return None;
    }

    let array_length = array.len();

    let window_size = if window_size > array_length {
        array_length
    } else {
        window_size
    };

    Some(array.windows(window_size).map(average).collect::<Vec<_>>())
}

fn average(array: &[f64]) -> f64 {
    let len = array.len() as f64;
    let sum = array.iter().sum::<f64>();

    sum / len
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
