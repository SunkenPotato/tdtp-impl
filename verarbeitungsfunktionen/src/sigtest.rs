
use statrs::distribution::{FisherSnedecor, StudentsT, ContinuousCDF};

/// Mittelwert berechnen
fn mean(data: &[f64]) -> f64 {
    data.iter().sum::<f64>() / (data.len() as f64)
}

/// Stichprobenvarianz (mit n-1 im Nenner)
fn variance(data: &[f64]) -> f64 {
    let m = mean(data);
    data.iter().map(|x| (x - m).powi(2)).sum::<f64>() / ((data.len() - 1) as f64)
}

/// F-Test auf Varianzgleichheit
fn f_test(x: &[f64], y: &[f64]) -> f64 {
    let var_x = variance(x);
    let var_y = variance(y);
    let (nx, ny) = (x.len() as f64, y.len() as f64);

    // F-Statistik
    let f_stat = if var_x > var_y { var_x / var_y } else { var_y / var_x };
    let df1 = (nx - 1.0).round() as f64;
    let df2 = (ny - 1.0).round() as f64;

    let f_dist = FisherSnedecor::new(df1, df2).unwrap();
    // zweiseitiger Test: p = 2 * min(P(F<=f), P(F>=f))
    let p_left = f_dist.cdf(f_stat);
    let p_right = 1.0 - p_left;
    2.0 * p_left.min(p_right)
}

/// t-Test für Mittelwertgleichheit (Student oder Welch)
fn t_test(x: &[f64], y: &[f64], equal_var: bool) -> f64 {
    let mean_x = mean(x);
    let mean_y = mean(y);
    let var_x = variance(x);
    let var_y = variance(y);
    let (nx, ny) = (x.len() as f64, y.len() as f64);

    let (se, df) = if equal_var {
        let pooled_var = ((nx - 1.0) * var_x + (ny - 1.0) * var_y) / (nx + ny - 2.0);
        let se = (pooled_var * (1.0 / nx + 1.0 / ny)).sqrt();
        (se, nx + ny - 2.0)
    } else {
        let se = (var_x / nx + var_y / ny).sqrt();
        let df = ( (var_x / nx + var_y / ny).powi(2) )
            / ( (var_x.powi(2) / (nx.powi(2) * (nx - 1.0))) + (var_y.powi(2) / (ny.powi(2) * (ny - 1.0))) );
        (se, df)
    };

    let t_stat = (mean_x - mean_y) / se;
    let t_dist = StudentsT::new(0.0, 1.0, df).unwrap();
    // zweiseitig
    2.0 * (1.0 - t_dist.cdf(t_stat.abs()))
}

/// Bestimmt, ob sich zwei Datensätze signifikant voneinander unterscheiden
/// # Argumente
/// * `x`- Datenset 1
/// * `y`- Datenset 2
/// * `alpha` - Signifikanzniveau (Wahrscheinlichkeit, Nullhypothese fälschlicherweise abzulehnen)
/// #### In unserem Fall bedeutet das, das wir keine neue Baseline erstellen müssen und die Daten annehmen oder eben nicht.
pub fn significant_difference(x: &[f64], y: &[f64], alpha: f64) -> bool {
    let p_f = f_test(x, y);
    let equal_var = p_f > alpha;
    let p_t = t_test(x, y, equal_var);

    p_f < alpha || p_t < alpha
}

