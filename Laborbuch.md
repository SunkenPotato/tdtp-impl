# Laborbuch 
Das Laborbuch dient als fortlaufende, detaillierte Dokumentation aller Arbeitsschritte, Beobachtungen, Messungen, Experimente und Zwischenergebnisse des Projekts ***"Τύχη - Zufall durch Zerfall"***. Ziel ist es, die Vorgehensweise nachvollziehbar zu machen, Probleme und Lösungen zu dokumentieren und eine transparente Basis für die spätere Auswertung und Präsentation zu schaffen.
## Datum: 2. September 2025
- **Erkenntnisse:** 
    - Um die Intervalle in Bits umzuwandeln, sodass 0en und 1en gleich wahrscheinlich sind, könnten wir die ***Quantil-basierte Zufallsbit-Extraktion*** verwenden, bei der man eine *Vergleichsverteilung* erstellt und jedes neue Intervall, das detektiert wird, wird in einen der $2^n$ Bins, die alle ein Integral von $2^-n$ haben, dieser *Vergleichsverteilung* einteilt

## Datum 24.September 2025
- **Geschafft:**
    -Erste grundlegende Implementation der Datenverarbeitung. Mathematische Funktionen müssen noch ergänzt werden. Man kann die Klasse I2B initialisieren und take_intervall aufruft, kriegt man Bits zurück, nachdem die Baseline, also Vergleichsverteilung voll ist.