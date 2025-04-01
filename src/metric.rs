use ndarray::Array1;
use crate::ModelError;
use statrs::distribution::{Discrete, Hypergeometric};
use std::collections::HashMap;

/// Calculates the Mean Squared Error (MSE) of a set of values.
///
/// The MSE is calculated as the average of the squared differences between each value
/// and the mean of all values. It represents the variance of the dataset.
///
/// # Arguments
/// * `y` - A slice of f64 values for which to calculate the MSE
///
/// # Returns
/// * `f64` - The mean squared error as a f64 value, returns 0.0 if the input slice is empty
///
/// # Examples
///
/// ```
/// use rustyml::math::mean_squared_error;
///
/// let values = [1.0, 2.0, 3.0];
/// let mse = mean_squared_error(&values);
/// // Mean is 2.0, so MSE = ((1-2)^2 + (2-2)^2 + (3-2)^2) / 3 = (1 + 0 + 1) / 3 ≈ 0.66667
/// assert!((mse - 0.6666667).abs() < 1e-6);
/// ```
pub use crate::math::mean_squared_error;

/// Calculates the Root Mean Squared Error (RMSE) between predicted and actual values.
///
/// RMSE is the square root of the Mean Squared Error (MSE), providing a metric that
/// has the same units as the original data, making it more interpretable.
///
/// # Arguments
/// * `predictions` - A slice of f64 values containing the predicted values
/// * `targets` - A slice of f64 values containing the actual/target values
///
/// # Returns
/// - `Ok(f64)` - The RMSE as a f64 value on success
/// - `Err(ModelError::InputValidationError)` - If input does not match expectation
///
/// # Examples
///
/// ```
/// use rustyml::metric::root_mean_squared_error;
///
/// let predictions = [2.0, 3.0, 4.0];
/// let targets = [1.0, 2.0, 3.0];
/// let rmse = root_mean_squared_error(&predictions, &targets).unwrap();
/// // RMSE = sqrt(((2-1)^2 + (3-2)^2 + (4-3)^2) / 3) = sqrt(3/3) = 1.0
/// assert!((rmse - 1.0).abs() < 1e-6);
/// ```
pub fn root_mean_squared_error(predictions: &[f64], targets: &[f64]) -> Result<f64, ModelError> {
    // Check if inputs are empty
    if predictions.is_empty() || targets.is_empty() {
        return Err(ModelError::InputValidationError("Input arrays cannot be empty".to_string()));
    }

    // Check if arrays have matching lengths
    if predictions.len() != targets.len() {
        return Err(ModelError::InputValidationError(
            format!("Prediction and target arrays must have the same length. Predicted: {}, Actual: {}", predictions.len(), targets.len())
        ));
    }

    // Calculate sum of squared errors
    let mut sum_squared_errors = 0.0;
    for i in 0..predictions.len() {
        let error = predictions[i] - targets[i];
        sum_squared_errors += error * error;
    }

    // Calculate mean squared error
    let mse = sum_squared_errors / predictions.len() as f64;

    // Return the square root of MSE
    Ok(mse.sqrt())
}

/// Calculates the Mean Absolute Error (MAE) between predicted and actual values.
///
/// MAE measures the average magnitude of errors between paired observations, without
/// considering their direction. It's calculated as the average of absolute differences
/// between predicted and target values.
///
/// # Arguments
/// * `predictions` - A slice of f64 values containing the predicted values
/// * `targets` - A slice of f64 values containing the actual/target values
///
/// # Returns
/// - `Ok(f64)` - The MAE as a f64 value on success
/// - `Err(ModelError::InputValidationError)` - If input does not match expectation
///
/// # Examples
///
/// ```
/// use rustyml::metric::mean_absolute_error;
///
/// let predictions = [2.0, 3.0, 4.0];
/// let targets = [1.0, 2.0, 3.0];
/// let mae = mean_absolute_error(&predictions, &targets).unwrap();
/// // MAE = (|2-1| + |3-2| + |4-3|) / 3 = (1 + 1 + 1) / 3 = 1.0
/// assert!((mae - 1.0).abs() < 1e-6);
/// ```
pub fn mean_absolute_error(predictions: &[f64], targets: &[f64]) -> Result<f64, ModelError> {
    // Check if inputs are empty
    if predictions.is_empty() || targets.is_empty() {
        return Err(ModelError::InputValidationError("Input cannot be empty".to_string()));
    }

    // Check if arrays have matching lengths
    if predictions.len() != targets.len() {
        return Err(ModelError::InputValidationError(
            format!("Prediction and target arrays must have the same length. Predicted: {}, Actual: {}", predictions.len(), targets.len())
        ));
    }

    // Calculate sum of absolute errors
    let sum_absolute_errors: f64 = predictions.iter()
        .zip(targets.iter())
        .map(|(p, t)| (p - t).abs())
        .sum();

    // Calculate mean absolute error
    let mae = sum_absolute_errors / predictions.len() as f64;

    Ok(mae)
}

/// Calculate the R-squared score
///
/// R² = 1 - (SSE / SST)
///
/// # Parameters
/// * `predicted` - Array of predicted values
/// * `actual` - Array of actual values
///
/// # Returns
///  - `Ok(f64)` - R-squared value, typically ranges from 0 to 1
///  - `Err(ModelError::InputValidationError)` - If input does not match expectation
///
/// # Notes
/// - Returns 0 if SST is 0 (when all actual values are identical)
/// - R-squared can theoretically be negative, indicating that the model performs worse than simply predicting the mean
/// - A value close to 1 indicates a good fit
///
/// # Examples
///
/// ```
/// use rustyml::metric::r2_score;
///
/// let predicted = [2.0, 3.0, 4.0];
/// let actual = [1.0, 3.0, 5.0];
/// let r2 = r2_score(&predicted, &actual).unwrap();
/// // For actual values [1,3,5], mean=3, SSE = 1+0+1 = 2, SST = 4+0+4 = 8, so R2 = 1 - (2/8) = 0.75
/// assert!((r2 - 0.75).abs() < 1e-6);
/// ```
pub fn r2_score(predicted: &[f64], actual: &[f64]) -> Result<f64, ModelError> {
    use crate::math::{sum_of_square_total, sum_of_squared_errors};
    let sse = sum_of_squared_errors(predicted, actual)?;
    let sst = sum_of_square_total(actual);

    // Prevent division by zero (when all actual values are identical)
    if sst == 0.0 {
        return Ok(0.0);
    }

    Ok(1.0 - (sse / sst))
}

/// # Confusion Matrix for binary classification evaluation
///
/// A confusion matrix is a table that is often used to describe the performance of a classification model
/// on a set of test data for which the true values are known. It allows visualization of the performance
/// of an algorithm and identification of common types of errors.
///
/// ## Fields
///
/// * `tp` - True Positive: The number of correct positive predictions when the actual class is positive
/// * `fp` - False Positive: The number of incorrect positive predictions when the actual class is negative (Type I error)
/// * `tn` - True Negative: The number of correct negative predictions when the actual class is negative
/// * `fn_` - False Negative: The number of incorrect negative predictions when the actual class is positive (Type II error)
///
/// ## Performance Metrics
///
/// This implementation provides methods to calculate common performance metrics including:
/// accuracy, precision, recall, specificity, and F1 score.
///
/// ## Example
///
/// ```
/// use ndarray::arr1;
/// use rustyml::metric::ConfusionMatrix;
///
/// // Create arrays for predicted and actual values
/// let predicted = arr1(&[0.9, 0.2, 0.8, 0.1, 0.7]);
/// let actual = arr1(&[1.0, 0.0, 1.0, 0.0, 1.0]);
///
/// // Create confusion matrix
/// let cm = ConfusionMatrix::new(&predicted, &actual).unwrap();
///
/// // Calculate performance metrics
/// println!("Accuracy: {:.2}", cm.accuracy());
/// println!("Precision: {:.2}", cm.precision());
/// println!("Recall: {:.2}", cm.recall());
/// println!("F1 Score: {:.2}", cm.f1_score());
///
/// // Get the confusion matrix components
/// let (tp, fp, tn, fn_) = cm.get_counts();
/// println!("TP: {}, FP: {}, TN: {}, FN: {}", tp, fp, tn, fn_);
///
/// // Print full summary
/// println!("{}", cm.summary());
/// ```
#[derive(Debug, Clone)]
pub struct ConfusionMatrix {
    /// True Positive (TP): The number of correct positive predictions
    /// when the actual class is positive (correctly identified positive cases)
    tp: usize,

    /// False Positive (FP): The number of incorrect positive predictions
    /// when the actual class is negative (Type I error, false alarm)
    fp: usize,

    /// True Negative (TN): The number of correct negative predictions
    /// when the actual class is negative (correctly identified negative cases)
    tn: usize,

    /// False Negative (FN): The number of incorrect negative predictions
    /// when the actual class is positive (Type II error, miss)
    fn_: usize,
}

impl ConfusionMatrix {
    /// Create a new confusion matrix
    ///
    /// # Parameters
    ///
    /// * `predicted` - Array of predicted labels, values >= 0.5 are considered positive class
    /// * `actual` - Array of actual labels, values >= 0.5 are considered positive class
    ///
    /// # Returns
    ///
    /// - `Ok(Self)` - A new confusion matrix if input arrays have the same length
    /// - `Err(ModelError::InputValidationError)` - Input does not match expectation
    pub fn new(predicted: &Array1<f64>, actual: &Array1<f64>) -> Result<Self, ModelError> {
        if predicted.len() != actual.len() {
            return Err(ModelError::InputValidationError(
                format!("Input arrays must have the same length. Predicted: {}, Actual: {}", predicted.len(), actual.len())
            ));
        }

        let mut tp = 0;
        let mut fp = 0;
        let mut tn = 0;
        let mut fn_ = 0;

        for (p, a) in predicted.iter().zip(actual.iter()) {
            let p_binary = if *p >= 0.5 { 1.0 } else { 0.0 };
            let a_binary = if *a >= 0.5 { 1.0 } else { 0.0 };

            match (p_binary, a_binary) {
                (1.0, 1.0) => tp += 1,
                (1.0, 0.0) => fp += 1,
                (0.0, 1.0) => fn_ += 1,
                (0.0, 0.0) => tn += 1,
                _ => unreachable!(), // Should not happen as we explicitly convert to binary values
            }
        }

        Ok(Self { tp, fp, tn, fn_ })
    }

    /// Get the components of the confusion matrix
    ///
    /// # Returns
    ///
    /// A tuple containing the four basic components of the confusion matrix:
    /// * `tp` - True Positive count
    /// * `fp` - False Positive count
    /// * `tn` - True Negative count
    /// * `fn_` - False Negative count
    pub fn get_counts(&self) -> (usize, usize, usize, usize) {
        (self.tp, self.fp, self.tn, self.fn_)
    }

    /// Calculate accuracy: (TP + TN) / (TP + TN + FP + FN)
    ///
    /// Accuracy measures the proportion of correct predictions among the total number of cases examined.
    ///
    /// # Returns
    ///
    /// * `f64` - A float value between 0.0 and 1.0 representing the accuracy. Returns 0.0 if there are no predictions (empty matrix).
    pub fn accuracy(&self) -> f64 {
        let total = self.tp + self.tn + self.fp + self.fn_;
        if total == 0 {
            return 0.0;
        }
        (self.tp + self.tn) as f64 / total as f64
    }

    /// Calculate error rate: (FP + FN) / (TP + TN + FP + FN) = 1 - Accuracy
    ///
    /// Error rate measures the proportion of incorrect predictions among the total number of cases examined.
    ///
    /// # Returns
    ///
    /// * `f64` - A float value between 0.0 and 1.0 representing the error rate. Returns 1.0 if there are no predictions (empty matrix).
    pub fn error_rate(&self) -> f64 {
        1.0 - self.accuracy()
    }

    /// Calculate precision: TP / (TP + FP)
    ///
    /// Precision measures the proportion of positive identifications that were actually correct.
    /// It answers the question: "Of all the instances predicted as positive, how many were actually positive?"
    ///
    /// # Returns
    ///
    /// * `f64` - A float value between 0.0 and 1.0 representing precision. Returns 0.0 if there are no positive predictions (TP + FP = 0).
    pub fn precision(&self) -> f64 {
        if self.tp + self.fp == 0 {
            return 0.0;
        }
        self.tp as f64 / (self.tp + self.fp) as f64
    }

    /// Calculate recall (sensitivity): TP / (TP + FN)
    ///
    /// Recall measures the proportion of actual positives that were correctly identified.
    /// It answers the question: "Of all the actual positive instances, how many were correctly predicted?"
    ///
    /// # Returns
    ///
    /// * `f64` - A float value between 0.0 and 1.0 representing recall. Returns 1.0 if there are no actual positive instances (TP + FN = 0).
    pub fn recall(&self) -> f64 {
        if self.tp + self.fn_ == 0 {
            return 1.0;
        }
        self.tp as f64 / (self.tp + self.fn_) as f64
    }

    /// Calculate specificity: TN / (TN + FP)
    ///
    /// Specificity measures the proportion of actual negatives that were correctly identified.
    /// It answers the question: "Of all the actual negative instances, how many were correctly predicted?"
    ///
    /// # Returns
    ///
    /// * `f64` - A float value between 0.0 and 1.0 representing specificity. Returns 1.0 if there are no actual negative instances (TN + FP = 0).
    pub fn specificity(&self) -> f64 {
        if self.tn + self.fp == 0 {
            return 1.0;
        }
        self.tn as f64 / (self.tn + self.fp) as f64
    }

    /// Calculate F1 score: 2 * (Precision * Recall) / (Precision + Recall)
    ///
    /// F1 score is the harmonic mean of precision and recall, providing a balance between the two metrics.
    /// It's particularly useful when you want to balance precision and recall and there's an uneven class distribution.
    ///
    /// # Returns
    ///
    /// * `f64` - A float value between 0.0 and 1.0 representing the F1 score. Returns 0.0 if either precision or recall is 0.0.
    pub fn f1_score(&self) -> f64 {
        let precision = self.precision();
        let recall = self.recall();

        if precision + recall == 0.0 {
            return 0.0;
        }

        2.0 * (precision * recall) / (precision + recall)
    }

    /// Generate a formatted summary of the confusion matrix and all performance metrics
    ///
    /// # Returns
    ///
    /// * `String` - A formatted string containing a visual representation of the confusion matrix and all calculated performance metrics with 4 decimal places of precision.
    pub fn summary(&self) -> String {
        format!(
            "Confusion Matrix:\n\
        |                | Predicted Positive | Predicted Negative |\n\
        |----------------|-------------------|--------------------|\n\
        | Actual Positive | TP: {}           | FN: {}             |\n\
        | Actual Negative | FP: {}           | TN: {}             |\n\
        \n\
        Performance Metrics:\n\
        - Accuracy: {:.4}\n\
        - Error Rate: {:.4}\n\
        - Precision: {:.4}\n\
        - Recall: {:.4}\n\
        - Specificity: {:.4}\n\
        - F1 Score: {:.4}",
            self.tp, self.fn_, self.fp, self.tn,
            self.accuracy(), self.error_rate(),
            self.precision(), self.recall(),
            self.specificity(), self.f1_score()
        )
    }
}

/// Calculate the accuracy of a classification model
///
/// Accuracy is defined as the proportion of correctly predicted samples
/// over the total number of samples.
/// For binary classification, predicted and actual values should be 0.0 or 1.0.
/// For multi-class classification, values should be numeric class labels.
///
/// # Parameters
/// * `predicted` - Array of predicted class labels
/// * `actual` - Array of actual class labels
///
/// # Returns
/// - `Ok(f64)` - The accuracy score between 0.0 and 1.0
/// - `Err(ModelError::InputValidationError)` - If input does not match expectation
///
/// # Examples
///
/// ```
/// use rustyml::metric::accuracy;
///
/// let predicted = [0.0, 1.0, 1.0];
/// let actual = [0.0, 0.0, 1.0];
/// let acc = accuracy(&predicted, &actual).unwrap();
///
/// // Two out of three predictions are correct: accuracy = 2/3 ≈ 0.6666666666666667
/// assert!((acc - 0.6666666666666667).abs() < 1e-6);
/// ```
pub fn accuracy(predicted: &[f64], actual: &[f64]) -> Result<f64, ModelError> {
    if predicted.len() != actual.len() {
        return Err(ModelError::InputValidationError(
            format!("Input arrays must have the same length. Predicted: {}, Actual: {}", predicted.len(), actual.len())
        ));
    }

    if predicted.is_empty() {
        return Ok(0.0);
    }

    let correct_predictions = predicted
        .iter()
        .zip(actual.iter())
        .filter(|&(p, a)| (p - a).abs() < f64::EPSILON)
        .count();

    Ok(correct_predictions as f64 / predicted.len() as f64)
}

/// Constructs a contingency matrix from two label arrays and returns:
/// - The contingency matrix
/// - Row sums (sizes of clusters in the ground truth)
/// - Column sums (sizes of clusters in the predicted labels)
fn contingency_matrix(
    labels_true: &[usize],
    labels_pred: &[usize],
) -> (Vec<Vec<usize>>, Vec<usize>, Vec<usize>) {
    let mut label_to_index_true = HashMap::new();
    let mut label_to_index_pred = HashMap::new();
    let mut current_index_true = 0;
    let mut current_index_pred = 0;

    for &label in labels_true {
        label_to_index_true.entry(label).or_insert_with(|| {
            let idx = current_index_true;
            current_index_true += 1;
            idx
        });
    }
    for &label in labels_pred {
        label_to_index_pred.entry(label).or_insert_with(|| {
            let idx = current_index_pred;
            current_index_pred += 1;
            idx
        });
    }

    let n_rows = label_to_index_true.len();
    let n_cols = label_to_index_pred.len();
    let mut matrix = vec![vec![0usize; n_cols]; n_rows];

    for (&l_true, &l_pred) in labels_true.iter().zip(labels_pred.iter()) {
        let i = label_to_index_true[&l_true];
        let j = label_to_index_pred[&l_pred];
        matrix[i][j] += 1;
    }

    let row_sums = matrix
        .iter()
        .map(|row| row.iter().sum())
        .collect::<Vec<usize>>();
    let col_sums = (0..n_cols)
        .map(|j| matrix.iter().map(|row| row[j]).sum())
        .collect::<Vec<usize>>();

    (matrix, row_sums, col_sums)
}

/// Computes the mutual information (MI) using the formula:
/// MI = Σ_{i,j} (n_ij/n) * ln((n * n_ij) / (a_i * b_j))
fn mutual_information(
    contingency: &Vec<Vec<usize>>,
    n: usize,
    row_sums: &Vec<usize>,
    col_sums: &Vec<usize>,
) -> f64 {
    let mut mi = 0.0;
    for (i, row) in contingency.iter().enumerate() {
        for (j, &n_ij) in row.iter().enumerate() {
            if n_ij > 0 {
                let n_ij_f = n_ij as f64;
                let a = row_sums[i] as f64;
                let b = col_sums[j] as f64;
                mi += (n_ij_f / n as f64) * ((n as f64 * n_ij_f) / (a * b)).ln();
            }
        }
    }
    mi
}

/// Computes the entropy H = - Σ_i (p_i * ln(p_i))
fn entropy_nats(counts: &Vec<usize>, n: usize) -> f64 {
    let mut h = 0.0;
    for &count in counts {
        if count > 0 {
            let p = count as f64 / n as f64;
            h -= p * p.ln();
        }
    }
    h
}

/// Computes the expected mutual information (EMI).
/// For each pair (a_i, b_j), assume that n_ij follows a Hypergeometric distribution with parameters:
/// - Total population size: n
/// - Number of successes: a_i
/// - Number of draws: b_j
///
/// EMI = Σ_{i,j} Σ_{k=max(0, a_i+b_j-n)}^{min(a_i, b_j)}
///       P(k) * (k/n) * ln((n * k) / (a_i * b_j))
fn expected_mutual_information(
    row_sums: &Vec<usize>,
    col_sums: &Vec<usize>,
    n: usize,
) -> f64 {
    let mut emi = 0.0;
    // For each pair of clusters (ground truth and predicted)
    for &a_i in row_sums {
        for &b_j in col_sums {
            // Create a hypergeometric distribution with parameters: total = n, successes = a_i, draws = b_j
            let hyper = match Hypergeometric::new(n as u64, a_i as u64, b_j as u64) {
                Ok(dist) => dist,
                Err(_) => continue, // Skip if parameters are invalid
            };
            // Valid range for k: from max(0, a_i+b_j-n) to min(a_i, b_j)
            let lower_bound = if a_i + b_j > n { a_i + b_j - n } else { 0 };
            // Skip k=0 since it contributes 0 to MI, so start from 1
            let lower = if lower_bound < 1 { 1 } else { lower_bound };
            let upper = std::cmp::min(a_i, b_j);
            for k in lower..=upper {
                let p = hyper.pmf(k as u64);
                let term = (k as f64 / n as f64)
                    * ((n as f64 * k as f64) / (a_i as f64 * b_j as f64)).ln();
                emi += p * term;
            }
        }
    }
    emi
}

/// Calculates the Normalized Mutual Information (NMI) between two cluster label assignments.(Unit: nat)
///
/// NMI measures the agreement between two cluster assignments, normalized by the geometric
/// mean of their entropies. The score ranges from 0 to 1, where 1 indicates perfect agreement
/// and 0 indicates no mutual information between the clusterings.
///
/// The formula used is:
/// NMI = MI(U, V) / sqrt(H(U) * H(V))
///
/// where:
/// - MI(U, V) is the mutual information between clusterings U and V
/// - H(U) and H(V) are the entropies of the clusterings
/// - sqrt represents the square root function
///
/// # Parameters
///
/// * `labels_true` - A slice of cluster assignments representing the ground truth or reference clustering
/// * `labels_pred` - A slice of cluster assignments representing the predicted or comparison clustering
///
/// # Returns
///
/// - `Ok(f64)` - The NMI score as a float between 0 and 1
/// - `Err(ModelError::InputValidationError)` - If the input slices have different lengths
///
/// # Examples
///
/// ```
/// use rustyml::metric::normalized_mutual_info;
///
/// let true_labels = vec![0, 0, 1, 1, 2, 2];
/// let pred_labels = vec![0, 0, 1, 2, 1, 2];
///
/// let nmi = normalized_mutual_info(&true_labels, &pred_labels).unwrap();
/// println!("Normalized Mutual Information: {:.4}", nmi);
/// ```
pub fn normalized_mutual_info(labels_true: &[usize], labels_pred: &[usize]) -> Result<f64, ModelError> {
    if labels_true.len() != labels_pred.len() {
        return Err(ModelError::InputValidationError(
            format!("Input arrays must have the same length. Predicted: {}, Actual: {}", labels_true.len(), labels_pred.len())
        ));
    }
    let n = labels_true.len();
    let (contingency, row_sums, col_sums) = contingency_matrix(labels_true, labels_pred);
    let mi = mutual_information(&contingency, n, &row_sums, &col_sums);
    let h_true = entropy_nats(&row_sums, n);
    let h_pred = entropy_nats(&col_sums, n);
    if h_true * h_pred == 0.0 {
        Ok(0.0)
    } else {
        Ok(mi / (h_true * h_pred).sqrt())
    }
}

/// Calculates the Adjusted Mutual Information (AMI) between two cluster label assignments.(Unit: nat)
///
/// AMI adjusts the Mutual Information score to account for chance, correcting the effect of
/// agreement solely due to chance between clustering assignments. AMI score equals 1.0 when
/// two clusterings are identical and approximately 0.0 for random independent clusterings.
/// Unlike NMI, AMI can yield negative values when the observed mutual information is less
/// than expected.
///
/// The formula used is:
/// AMI = (MI - E\[MI\]) / (avg(H(U), H(V)) - E\[MI\])
///
/// where:
/// - MI is the mutual information between clusterings
/// - E\[MI\] is the expected mutual information between random clusterings with same cluster counts
/// - H(U) and H(V) are the entropies of the clusterings
/// - avg represents the arithmetic mean
///
/// # Parameters
///
/// * `labels_true` - A slice of cluster assignments representing the ground truth or reference clustering
/// * `labels_pred` - A slice of cluster assignments representing the predicted or comparison clustering
///
/// # Returns
///
/// - `Ok(f64)` - The AMI score, typically between -1 and 1
/// - `Err(ModelError::InputValidationError)` - If the input slices have different lengths
///
/// # Examples
///
/// ```
/// use rustyml::metric::adjusted_mutual_info;
///
/// let true_labels = vec![0, 0, 1, 1, 2, 2];
/// let pred_labels = vec![0, 0, 1, 2, 1, 2];
///
/// let ami = adjusted_mutual_info(&true_labels, &pred_labels).unwrap();
/// println!("Adjusted Mutual Information: {:.4}", ami);
/// ```
pub fn adjusted_mutual_info(labels_true: &[usize], labels_pred: &[usize]) -> Result<f64, ModelError> {
    if labels_true.len() != labels_pred.len() {
        return Err(ModelError::InputValidationError(
            format!("Input arrays must have the same length. Predicted: {}, Actual: {}", labels_true.len(), labels_pred.len())
        ));
    }
    let n = labels_true.len();
    let (contingency, row_sums, col_sums) = contingency_matrix(labels_true, labels_pred);
    let mi = mutual_information(&contingency, n, &row_sums, &col_sums);
    let h_true = entropy_nats(&row_sums, n);
    let h_pred = entropy_nats(&col_sums, n);
    let emi = expected_mutual_information(&row_sums, &col_sums, n);
    let denominator = ((h_true + h_pred) / 2.0) - emi;
    if denominator.abs() < 1e-10 {
        Ok(1.0)
    } else {
        Ok((mi - emi) / denominator)
    }
}