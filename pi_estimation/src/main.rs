fn main() {

    // Write a program that computes the sum of an alternating series where 
    // each element of the series is an expression of the form ((−1)k+1)/(2*k−1) 
    // for each value of k from 1 to a million, multiplied by 4. 
    // Or, in more mathematical notation
    //      10^6
    //  4 . ∑   (−1)^(k+1)/2k−1 = 4⋅(1−1/3+1/5−1/7+1/9−1/11…).
    //      k=1
    
    let n = 1_000_000;
    let mut sum = 0.0;

    for k in 1..=n {
        let term = (-1.0_f64).powi(k + 1) / (2 * k - 1) as f64;
        sum += term;
    }

    let pi_estimate = 4.0 * sum;

    println!("Estimate of Pi using {} terms is: {}", n, pi_estimate);
}
