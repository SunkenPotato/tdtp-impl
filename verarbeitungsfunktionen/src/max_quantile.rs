/// Berechnet die maximale sichere Anzahl von Quantil-Bins für einen Geigerzähler
///
/// # Argumente
/// * `mean_interval` - mittleres Intervall zwischen Impulsen (in Sekunden)
/// * `delta_t` - minimale Zeitauflösung des Messgerätes (in Sekunden)
/// * `safety_factor` - Faktor zwischen 0.0 und 1.0, um Rauschen und Korrelationen zu kompensieren (z.B. 0.7)
///
/// # Rückgabe
/// * `M_safe` - sichere maximale Anzahl von Bins (Quantile)
pub fn max_safe_quantile_bins(mean_interval: f64, delta_t: f64, safety_factor: f64) -> u64 {
    let m_max = std::f64::consts::E * mean_interval / delta_t;
    let m_safe = m_max * safety_factor;
    m_safe.round() as u64
}