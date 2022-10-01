use thiserror::Error;

#[derive(Error, Debug)]
pub enum MovingAverageError {
    #[error("`window` cannot be higher than array length")]
    WindowSize,
    #[error("array must contain at least one element")]
    EmptyArray,
}

pub fn moving_averages(array: &[f64], window: usize) -> Result<Vec<f64>, MovingAverageError> {
    if array.is_empty() {
        return Err(MovingAverageError::EmptyArray);
    }

    let array_length = array.len();

    if window > array_length {
        return Err(MovingAverageError::WindowSize);
    }

    let mut moving_averages = vec![];

    for i in 0..=array_length - window {
        let mut sum = 0.0;

        for j in 0..window {
            sum += array[i + j];
        }

        let moving_average = sum / window as f64;
        moving_averages.push(moving_average)
    }

    Ok(moving_averages)
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
}
