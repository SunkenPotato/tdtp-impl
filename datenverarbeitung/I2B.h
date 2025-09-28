#ifndef I2B_H
#define I2B_H

#include <vector>

class I2B {
public:
    int i = 0; // Iterator für Länge der Baseline
    std::vector<float> baseline;
    int baseline_len = 10000;
    static const int max_bins = 1024;
    std::vector<float> bins;
    int bin_nummer_in_bits;
    std::vector<double> quantiles;
    std::vector<float> intervalle;

    int post_baseline_counter = 0;

    int take_intervall(float intervall);
    void bins_erstellen();
    std::vector<int> welcher_bin(float intervall);
    bool sigtest();
};

#endif
