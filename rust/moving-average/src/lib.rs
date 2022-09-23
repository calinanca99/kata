pub fn moving_averages(array: &[f64], window: usize) -> Vec<f64> {
    let mut moving_averages = vec![];

    let array_length = array.len();

    for i in 0..array_length {
        let mut sum = 0.0;

        if i == array_length - window + 1 {
            break;
        }

        for j in 0..window {
            sum += array[i + j];
        }

        let moving_average = sum / window as f64;
        moving_averages.push(moving_average)
    }

    dbg!(&moving_averages);
    moving_averages
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
        assert_eq!(expected, sut);
    }

    #[test]
    fn compute_moving_average_with_empty_array() {
        // Arrange
        let array = [];
        let window = 5;

        // Act
        let sut = moving_averages(&array, window);

        // Assert
        let expected: Vec<f64> = vec![];
        assert_eq!(expected, sut);
    }

    #[test]
    fn compute_moving_average_with_windows_1_returns_the_same_array() {
        // Arrange
        let array = [1.0, 2.0, 3.0, 4.0, 5.0];
        let window = 1;

        // Act
        let sut = moving_averages(&array, window);

        // Assert
        let expected = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert_eq!(expected, sut);
    }

    #[test]
    #[should_panic]
    fn panics_when_window_size_is_larger_than_array_length() {
        // Arrange
        let array = [1.0, 2.0, 3.0, 4.0, 5.0];
        let window = 9;

        // Act
        let _sut = moving_averages(&array, window);
    }
}
