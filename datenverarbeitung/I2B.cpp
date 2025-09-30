#include <iostream>
#include <vector>
#include <random>
#include <cmath>
#include <numeric>
#include <algorithm>
using namespace std;
#include <fstream>

float randomFloat(float min = 0.0f, float max = 10.0f) {
    static thread_local std::mt19937 gen(std::random_device{}()); // ein Generator, nicht bei jedem Aufruf neu seeden
    std::uniform_real_distribution<float> dist(min, max);
    float r = dist(gen);
    return std::round(r * 10000.0f) / 10000.0f;
}

std::string to_binary_fixed(uint64_t x, size_t bit_len) {
    std::string out;
    out.reserve(bit_len);

    for (size_t i = 0; i < bit_len; ++i) {
        size_t shift = bit_len - 1 - i;
        out.push_back(((x >> shift) & 1ULL) ? '1' : '0');
    }
    return out;
}

class I2B {
public:
    int bit_länge;
    I2B(int bit_länge) : bit_länge(bit_länge) {}
    int i = 0; // Iterator für Länge der vergleichsdaten
    std::vector<float> vergleichsdaten;
    int vergleichsdaten_len = 100;
    int max_bins;
    int bin_nummer;
    std::vector<double> quantiles;
    std::vector<float> intervalle;

    int post_vergleichsdaten_zähler = 0;

    int take_intervall(float intervall);
    void bins_erstellen();
    int welcher_bin(float intervall);
    bool sigtest();
};

int I2B::take_intervall(float intervall) {
    if (i < vergleichsdaten_len)
    {
        vergleichsdaten.push_back(intervall);
        i++;
        return -1;
    }
    else
    {
        intervalle.push_back(intervall);
        if (quantiles.empty())
            bins_erstellen();
        bin_nummer = welcher_bin(intervall);
        i++;
        post_vergleichsdaten_zähler++;
        if (post_vergleichsdaten_zähler % 10000 == 0)
        {
            if (sigtest())
            {
                i = 0;
                quantiles.clear();
                intervalle.clear();
                vergleichsdaten.clear();
                return -1;
            }
        }

        auto bits = to_binary_fixed(bin_nummer, bit_länge);
        
        return bits;
    }
}

void I2B::bins_erstellen() {
    max_bins = round(sqrt(vergleichsdaten_len));

    // 1. Lambda aus vergleichsdaten schätzen
    double mean = std::accumulate(vergleichsdaten.begin(), vergleichsdaten.end(), 0.0) / vergleichsdaten.size();
    double lambda_hat = 1.0 / mean;

    // 2. Quantile für gleichwahrscheinliche Bins
    quantiles.resize(max_bins);
    for (int k = 1; k <= max_bins; ++k)
    {
        double p = static_cast<double>(k) / max_bins; // p = k/n
        quantiles[k - 1] = -std::log(1.0 - p) / lambda_hat;
    }

    // Exportieren der Bins
    std::ofstream file("bins.csv");
    file << lambda_hat << "\n";
    for (auto q : quantiles)
        file << q << "\n";
    file.close();
}

int I2B::welcher_bin(float intervall) {
    int num_bins = static_cast<int>(quantiles.size()) + 1;
    int bits_needed = static_cast<int>(std::ceil(std::log2(num_bins)));

    int index;
    if (intervall > quantiles.back())
    {
        index = num_bins - 1;
    }
    else
    {
        auto it = std::upper_bound(quantiles.begin(), quantiles.end(), intervall);
        index = static_cast<int>(std::distance(quantiles.begin(), it));
    }

    return index;
}

bool I2B::sigtest() {
    double mean_base = std::accumulate(vergleichsdaten.begin(), vergleichsdaten.end(), 0.0) / vergleichsdaten.size();
    double mean_interv = std::accumulate(intervalle.begin(), intervalle.end(), 0.0) / intervalle.size();

    double var_base = 0.0;
    for (double x : vergleichsdaten)
        var_base += (x - mean_base) * (x - mean_base);
    var_base /= (vergleichsdaten.size() - 1);

    double var_interv = 0.0;
    for (double x : intervalle)
        var_interv += (x - mean_interv) * (x - mean_interv);
    var_interv /= (intervalle.size() - 1);

    double t = std::abs(mean_base - mean_interv) / std::sqrt(var_base / vergleichsdaten.size() + var_interv / intervalle.size());

    const double t_crit = 2.58; // ungefähr 99% Konfidenz
    cout << (t > t_crit) << endl;
    return t > t_crit;
}

int main() {
    cout << sizeof(5);
    I2B converter;
    std::vector<int> zufallszahlen;
    for (int i = 0; i <= 100000; i++)
    {
        float intervall = randomFloat();
        zufallszahlen.push_back(converter.take_intervall(intervall));
    }
}


// TODO:
// WIE SOLLEN DIE BITS ZURÜCKGEGEBEN WERDEN
// CODE AUF DEUTSCH SCHREIBEN
// NAMEN DER VARIABLEN VERBESSERN
