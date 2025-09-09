# Laborbuch 
Das Laborbuch dient als fortlaufende, detaillierte Dokumentation aller Arbeitsschritte, Beobachtungen, Messungen, Experimente und Zwischenergebnisse des Projekts ***"Τύχη - Zufall durch Zerfall"***. Ziel ist es, die Vorgehensweise nachvollziehbar zu machen, Probleme und Lösungen zu dokumentieren und eine transparente Basis für die spätere Auswertung und Präsentation zu schaffen.
## Datum: 2. September 2025
- **Erkenntnisse:** 
    - Um die Intervalle in Bits umzuwandeln, sodass 0en und 1en gleich wahrscheinlich sind, könnten wir die ***Quantil-basierte Zufallsbit-Extraktion*** verwenden, bei der man eine *Vergleichsverteilung* erstellt. Jedes neue Intervall, das detektiert wird, wird in ein Bin der *Verteilungsfunktion* dieser *Vergleichsverteilung* eingeteilt. Die Wahrscheinlichkeit, dass ein neues Intervall in einem der Bins landet ist bei allen Bins gleich, nämlich $1/2^n$, wobei $n$ die Anzahl der sicheren Bits ist pro Intervall. Sein Wert muss noch anderweitig berechnet werden.
- **Plan:**
    - Structure schreiben, der man ein Intervall gibt und die dann die Bits returned. 
- **Mögliche Probleme:**
    - Es könnte sein, dass sich λ über die Zeit ändert - Lösungsmöglichkeit: Alle z.B. 1000 Intervalle wird das Lamba aus der *Vergleichsverteilung* mit dem aus diesen 1000 Intervallen verglichen. Wenn sie sich zu stark unterscheiden, wird eine neue *Vergleichsverteilung* erstellt.
## Datum: 7. September 2025
- **Heute geschafft:**
    - Geigerzähler ist vor einigen Tagen angekommen
    - ESP32 ist vor wenigen Tagen angekommen
    - Spannungsteiler ist vor einigen Tagen angekommen
    - Es fehlen noch ein paar Jumper-Wires
    - Es fehlt noch ein ***Sync HDMI-Mini to USB-C Kabel***  
    - Dann Anschluss: 
    Geiger OUT (5V)  ──► RXI (HV side) [ST1167]
                      RXO (LV side) ──► IO17 (ESP32)   <- Signal zum ESP32
    ESP32 3.3V ──► LV (ST1167)
    ESP32 5V   ──► HV (ST1167)
    ESP32 GND  ──┬─► GND (ST1167)
                └─► GND (Geiger)

