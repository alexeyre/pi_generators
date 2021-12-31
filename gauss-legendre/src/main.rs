
fn main() {
    let a: f64 = 1.0;
    let b: f64 = 2.0;
    let b = b.sqrt();
    let b = b.recip();
    let t = 0.25;
    let p = 1.0;
    iterate(a, b, t, p, 0.0);
}

fn iterate(a: f64, b: f64, t: f64, p: f64, pi: f64) {
    let a_n = (a + b) / 2.0;
    let b_n: f64 = a * b;
    let b_n = b_n.sqrt();
    let t_n = t - p * (a - a_n).powf(2.0);
    let p_n = 2.0 * p;
    let pi_n = ((a_n + b_n).powf(2.0)) / (4.0 * t_n);
    if !pi.is_nan() {
        if pi_n == pi {
        } else {
            println!("{}", pi);
            iterate(a_n, b_n, t_n, p_n, pi_n);
        }
    }
}


