#include "I2B.h"
#include <numeric>
#include <cmath>
#include <algorithm>

int I2B::take_intervall(float intervall) {
    intervalle.push_back(intervall);
    if (i < baseline_len) {
        baseline.push_back(intervall);
        i++;
        return -1;
    } else {
        if (bins.empty()) bins_erstellen();
        welcher_bin(intervall);
        i++;
        post_baseline_counter++;
        if (post_baseline_counter % 10000 == 0) {
            if (sigtest()) return -1;
        }
        return bin_nummer_in_bits;
    }
}

void I2B::bins_erstellen() {
    double sum = std::accumulate(baseline.begin(), baseline.end(), 0.0);
    double mean = sum / baseline.size();
    double lambda_hat = 1.0 / mean;

    quantiles.resize(max_bins);
    for (int k = 1; k <= max_bins; ++k) {
        quantiles[k-1] = - (1.0 / lambda_hat) * std::log((double)(max_bins - k) / max_bins);
    }
}

std::vector<int> I2B::welcher_bin(float intervall) {
    int num_bins = static_cast<int>(quantiles.size()) + 1;
    int bits_needed = static_cast<int>(std::ceil(std::log2(num_bins)));

    int index;
    if (intervall > quantiles.back()) {
        index = num_bins - 1;
    } else {
        auto it = std::upper_bound(quantiles.begin(), quantiles.end(), intervall);
        index = static_cast<int>(std::distance(quantiles.begin(), it));
    }

    std::vector<int> bits(bits_needed, 0);
    for (int i = bits_needed - 1; i >= 0; --i) {
        bits[i] = index & 1;
        index >>= 1;
    }
    return bits;
}

bool I2B::sigtest() {
    double mean_base = std::accumulate(baseline.begin(), baseline.end(), 0.0) / baseline.size();
    double mean_interv = std::accumulate(intervalle.begin(), intervalle.end(), 0.0) / intervalle.size();

    double var_base = 0.0;
    for(double x : baseline) var_base += (x - mean_base) * (x - mean_base);
    var_base /= (baseline.size() - 1);

    double var_interv = 0.0;
    for(double x : intervalle) var_interv += (x - mean_interv) * (x - mean_interv);
    var_interv /= (intervalle.size() - 1);

    double t = std::abs(mean_base - mean_interv) / std::sqrt(var_base / baseline.size() + var_interv / intervalle.size());

    const double t_crit = 2.58; // ungefähr 99% Konfidenz
    return t > t_crit;
}
