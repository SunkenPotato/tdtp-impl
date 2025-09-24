#include <iostream>
#include <vector>
#include <cmath>

class I2B {
public:
    int i = 0;
    std::vector<float> baseline;
    int baseline_len = 10000; // Testwert, wird noch dynamisch bestimmt
    bool verteilung_erstellt = false;
    static const int max_bins = 1024;  // 2^10;Testwert, wird noch dynamisch bestimmt
    std::vector<float> bins;
    int bin_nummer_in_bits;

    // Zähler für Sigtest nach Baseline
    int post_baseline_counter = 0;

    int take_intervall(float intervall);
    void bins_erstellen();
    int welcher_bin();
    void sigtest();
};

int I2B::take_intervall(float intervall) {
    if (i < baseline_len) {
        // Baseline füllen
        baseline.push_back(intervall);
        i++;
        return -1;  // noch kein Bit verfügbar
    } else {
        // Nach Baseline
        if (!verteilung_erstellt) {
            bins_erstellen();
        }

        welcher_bin();
        i++;

        // Zähler nach Baseline hochzählen
        post_baseline_counter++;
        if (post_baseline_counter % 10000 == 0) {
            sigtest();
        }

        return bin_nummer_in_bits;
    }
} 

