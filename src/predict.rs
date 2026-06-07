//! Prediction-based delta encoding.

pub type Predictor = fn(&[i64]) -> i64;

/// Encode using a predictor function.
/// Each value is encoded as the difference between the actual value and the predicted value.
pub fn encode(values: &[i64], predictor: Predictor) -> Vec<i64> {
    if values.is_empty() {
        return Vec::new();
    }

    let mut residuals = Vec::with_capacity(values.len());
    residuals.push(values[0]); // base value

    for i in 1..values.len() {
        let predicted = predictor(&values[..i]);
        residuals.push(values[i] - predicted);
    }

    residuals
}

/// Decode using a predictor function.
pub fn decode(residuals: &[i64], predictor: Predictor) -> Vec<i64> {
    if residuals.is_empty() {
        return Vec::new();
    }

    let mut values = Vec::with_capacity(residuals.len());
    values.push(residuals[0]);

    for &residual in residuals.iter().skip(1) {
        let predicted = predictor(&values);
        values.push(residual + predicted);
    }

    values
}

/// Predictor: predict the next value equals the last value (zero-order hold).
pub fn predict_last(values: &[i64]) -> i64 {
    *values.last().unwrap_or(&0)
}

/// Predictor: linear prediction (extrapolate from last two values).
pub fn predict_linear(values: &[i64]) -> i64 {
    if values.len() < 2 {
        return predict_last(values);
    }
    let n = values.len();
    2 * values[n - 1] - values[n - 2]
}

/// Predictor: average of all previous values.
pub fn predict_average(values: &[i64]) -> i64 {
    if values.is_empty() {
        return 0;
    }
    values.iter().sum::<i64>() / values.len() as i64
}

/// Predictor: double exponential smoothing.
pub fn predict_double_exponential(values: &[i64], alpha: f64, beta: f64) -> i64 {
    if values.is_empty() {
        return 0;
    }
    if values.len() == 1 {
        return values[0];
    }

    let mut level = values[0] as f64;
    let mut trend = (values[1] - values[0]) as f64;

    for &val in values.iter().skip(1) {
        let new_level = alpha * val as f64 + (1.0 - alpha) * (level + trend);
        let new_trend = beta * (new_level - level) + (1.0 - beta) * trend;
        level = new_level;
        trend = new_trend;
    }

    (level + trend).round() as i64
}

/// Compute the mean squared error of residuals.
pub fn mse(residuals: &[i64]) -> f64 {
    if residuals.is_empty() {
        return 0.0;
    }
    let sum: f64 = residuals.iter().map(|&r| (r as f64).powi(2)).sum();
    sum / residuals.len() as f64
}

/// Compare different predictors and return their MSE values.
pub fn compare_predictors(values: &[i64]) -> [(&'static str, f64); 3] {
    let predictors: [(&str, Predictor); 3] = [
        ("last", predict_last),
        ("linear", predict_linear),
        ("average", predict_average),
    ];

    let mut results = [("last", 0.0), ("linear", 0.0), ("average", 0.0)];

    for (i, (name, pred)) in predictors.iter().enumerate() {
        let residuals = encode(values, *pred);
        let err = mse(&residuals[1..]);
        results[i] = (*name, err);
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        assert!(encode(&[], predict_last).is_empty());
        assert!(decode(&[], predict_last).is_empty());
    }

    #[test]
    fn test_last_predictor_roundtrip() {
        let values = vec![10, 20, 30, 25, 40];
        let residuals = encode(&values, predict_last);
        let decoded = decode(&residuals, predict_last);
        assert_eq!(decoded, values);
    }

    #[test]
    fn test_linear_predictor_roundtrip() {
        let values = vec![10, 20, 30, 40, 50];
        let residuals = encode(&values, predict_linear);
        let decoded = decode(&residuals, predict_linear);
        assert_eq!(decoded, values);
    }

    #[test]
    fn test_linear_predictor_perfect() {
        // Perfect linear sequence: residuals should all be zero (except base)
        let values = vec![10, 20, 30, 40, 50];
        let residuals = encode(&values, predict_linear);
        // First residual is base value (10)
        // Second residual: predicted = 10 (last), actual = 20, residual = 10 (not zero, because linear needs 2 values)
        // Third onward: linear predicts correctly, residual = 0
        assert!(residuals[2..].iter().all(|&r| r == 0));
    }

    #[test]
    fn test_average_predictor_roundtrip() {
        let values = vec![5, 10, 15, 20, 25];
        let residuals = encode(&values, predict_average);
        let decoded = decode(&residuals, predict_average);
        assert_eq!(decoded, values);
    }

    #[test]
    fn test_compare_predictors() {
        let values = vec![10, 20, 30, 40, 50];
        let results = compare_predictors(&values);
        // Linear should be best for this data
        assert!(results[1].1 <= results[0].1); // linear <= last
    }
}
