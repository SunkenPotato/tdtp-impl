#include <iostream>
#include <vector>
#include <random>
#include <cmath>
#include <numeric>
#include <algorithm>
using namespace std;
#include <fstream>


class I2B
{
public:
    int bit_länge;
    const int NOT_READY = -1; // Wird zurückgegeben, wenn noch nicht genug Informationen da sind
    int referenz_zähler_vergleichsdaten = 0; // Iterator für Länge der vergleichsdaten
    std::vector<double> vergleichsdaten;
    int vergleichsdaten_len = 7; // Testwert, kann parametrisiert werden
    int max_bins;
    int bin_nummer;
    std::vector<double> quantile;
    std::vector<double> intervalle;

    int post_vergleichsdaten_zähler = 0;

    int take_intervall(int intervall);
    void bins_erstellen();
    int welcher_bin(double intervall);
    bool sigtest();

    I2B()
    {
        max_bins = static_cast<int>(std::round(std::sqrt(vergleichsdaten_len))); // Sonst könnte man zu viel Information extrahieren, die eventuell nicht mehr zufällig ist
        vergleichsdaten.reserve(vergleichsdaten_len);
    }
};

int I2B::take_intervall(int intervall)
{
    if (referenz_zähler_vergleichsdaten < vergleichsdaten_len)
    {
        vergleichsdaten.push_back(intervall);
        referenz_zähler_vergleichsdaten++;
        return NOT_READY;
    }
    else
    {
        intervalle.push_back(intervall);
        if (quantile.empty())
            bins_erstellen();
        bin_nummer = welcher_bin(intervall);
        post_vergleichsdaten_zähler++;
        if (post_vergleichsdaten_zähler % 10000 == 0)
        {
            if (sigtest())
            {
                referenz_zähler_vergleichsdaten = 0;
                quantile.clear();
                intervalle.clear();
                vergleichsdaten.clear();
                return NOT_READY;
            }
        }

        return bin_nummer;
    }
}

// Nimmt die Vergleichsdatebn, schätzt damit das Lamda, also die Zerfallsrate der Dichtefunktion
// der Exponentialfunktion und teilt diese in "max_bins"
// quantile ein, die alle das Integral 1 / max_bins haben und speichert diese in dem Vektor "quantiles"
void I2B::bins_erstellen()
{
    // 1. Lambda aus vergleichsdaten schätzen
    double mean = std::accumulate(vergleichsdaten.begin(), vergleichsdaten.end(), 0.0) / vergleichsdaten.size();
    double lambda_hat = 1.0 / mean;

    // 2. Quantile für gleichwahrscheinliche Bins
    quantile.resize(max_bins);
    for (int k = 1; k <= max_bins; ++k)
    {
        double p = static_cast<double>(k) / max_bins; // p = k/n
        quantile[k - 1] = -std::log(1.0 - p) / lambda_hat;
    }

}

int I2B::welcher_bin(double intervall)
{
    int index;
    if (intervall > quantile.back())
    {
        index = max_bins - 1;
    }
    else
    {
        auto it = std::upper_bound(quantile.begin(), quantile.end(), intervall);
        index = static_cast<int>(std::distance(quantile.begin(), it));
    }

    return index;
}

bool I2B::sigtest()
{
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

// Testanwendung
int main()
{
    I2B converter;
    std::vector<int> intervalle = {12542, 87573, 90436, 87405, 12543, 76548, 89534, 65873, 17634, 78254, 90234, 15762, 87498};

    for (int intervall : intervalle) {
        int bin = converter.take_intervall(intervall);
        cout << bin << endl;
    }
}

// TODO:
// WIE SOLLEN DIE BITS ZURÜCKGEGEBEN WERDEN
// CODE AUF DEUTSCH SCHREIBEN
// NAMEN DER VARIABLEN VERBESSERN
