use thiserror::Error;

#[derive(Error, Debug)]
pub enum MovingAverageError {
    #[error("`window` cannot be higher than array length")]
    WindowSize,
    #[error("`array` must contain at least one element")]
    EmptyArray,
}

pub fn moving_averages(array: &[f64], window_size: usize) -> Result<Vec<f64>, MovingAverageError> {
    if array.is_empty() {
        return Err(MovingAverageError::EmptyArray);
    }

    let array_length = array.len();

    if window_size > array_length {
        return Err(MovingAverageError::WindowSize);
    }

    Ok(array.windows(window_size).map(average).collect::<Vec<_>>())
}

fn average(array: &[f64]) -> f64 {
    let len = array.len();
    let sum = array.iter().sum::<f64>();

    sum / len as f64
}

#[cfg(test)]
mod tests {
    use crate::moving_averages;

    #[test]
    fn computes_moving_average_with_arbitrary_window_size() {
        // Arrange
        let array = [1.0, 2.0, 3.0, 4.0, 5.0];
        let window = 4;

        // Act
        let sut = moving_averages(&array, window);

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
        let sut = moving_averages(&array, window);

        // Assert
        let expected = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert_eq!(expected, sut.unwrap());
    }

    #[test]
    fn errors_array_is_empty() {
        // Arrange
        let array = [];
        let window = 5;

        // Act
        let sut = moving_averages(&array, window);

        // Assert
        assert!(sut.is_err())
    }

    #[test]
    fn errors_when_window_size_is_larger_than_array_length() {
        // Arrange
        let array = [1.0, 2.0, 3.0, 4.0, 5.0];
        let window = 9;

        // Act
        let sut = moving_averages(&array, window);

        // Assert
        assert!(sut.is_err())
    }

    #[test]
    fn returns_inf_when_sum_of_a_windows_overflows() {
        // Arrange
        let array = [f64::MAX; 3];
        let window = 2;

        // Act
        let sut = moving_averages(&array, window);

        // Assert
        let expected = vec![f64::INFINITY, f64::INFINITY];
        assert_eq!(expected, sut.unwrap());
    }
}
