# Dokumentation der Batteriesimulation

# WICHIGE VOR INFORMATION:
Um dieses Programm verwenden zu können, braucht man rust lokal installiert zu haben. Hier gehts zum download: [Download](https://www.rust-lang.org/tools/install).
Dazu muss man die korrekten Input Daten an den korrekten ort plazieren. Die Daten sollten in /data/input plaziert sein. Die Excel Dateien sollten die namen:
- input_srl.xlsx
- input_wirkleistung.xlsx
haben.


## 1. Projektübersicht

Dieses Projekt implementiert eine Batteriesimulationssoftware, die darauf abzielt, die Leistung eines netzgekoppelten Batteriespeichersystems zu modellieren. Die Simulation berücksichtigt verschiedene Betriebsmodi wie Spitzenlastglättung (Peak Shaving) und die Teilnahme am SRL-Regelenergiemarkt. Es werden auch ökonomische Aspekte, Transformatorgrenzwerte und die Interaktion mit dem Stromnetz erfasst.

Das Hauptziel ist es, ein Verständnis dafür zu entwickeln, wie die Batterie unter verschiedenen Bedingungen agiert, welche Einnahmen erzielt oder Kosten eingespart werden können und wie sich die Batterienutzung (z.B. Zyklenzahl) auswirkt.

Die Daten werden aus Excel-Dateien importiert, auf eine einheitliche 1-Minuten-Auflösung interpoliert, zusammengeführt und anschliessend in der Simulation verarbeitet. Die Ergebnisse und eine Zusammenfassung werden in CSV-Dateien exportiert.

## 2. Projektstruktur

Das Projekt ist in mehrere Module gegliedert:

* **`excel`**: Verantwortlich für das Laden von Daten aus Excel-Dateien.
* **`model`**: Definiert die Datenstrukturen für Zeitreihen und Simulationsergebnisse.
* **`simulation`**: Enthält die Kernlogik der Batteriesimulation, Konfiguration und Ergebniszusammenfassung.
* **`utils`**: Bietet Hilfsfunktionen für Datenverarbeitung, Export und Zeitstempel-Parsing.

## 3. Modulbeschreibungen und Dateidetails

### 3.1. `src/main.rs`

Dies ist der Haupteinstiegspunkt der Anwendung. Er orchestriert den gesamten Simulationsablauf:

1.  **Datenimport und -vorbereitung:**
    * Prüft, ob eine bereits zusammengeführte CSV-Datei (`merged_timeseries.csv`) vorhanden ist.
    * Falls nicht vorhanden:
        * Lädt SRL-Daten (`input_srl.xlsx`) und Lastgangdaten (`input_wirkleistung.xlsx`) aus Excel.
        * Generiert ein 1-Minuten-Zeitraster basierend auf dem Bereich der Lastgangdaten.
        * Interploliert sowohl die Lastgang- als auch die SRL-Daten auf die 1-Minuten-Auflösung.
        * Führt die interpolierten Daten zu einer einzigen `MergedTick`-Serie zusammen.
        * Speichert die bereinigten Lastgang-, SRL- und die zusammengeführten Daten in CSV-Dateien im Verzeichnis `data/output/`.
2.  **Simulationsausführung:**
    * Initialisiert eine `SimulationConfig` mit Standardwerten und passt diese ggf. an (z.B. `initial_soc_frac`, `reserve_fraction`).
    * Führt die Simulation mit den zusammengeführten Daten (`merged_entries`) und der Konfiguration aus.
3.  **Ergebnisausgabe:**
    * Speichert die detaillierten Simulationsergebnisse pro Zeitschritt in `simulation.results.csv`.
    * Fasst die Simulationsergebnisse zusammen und gibt eine detaillierte Zusammenfassung auf der Konsole aus.

### 3.2. `src/excel/mod.rs`

Dieses Modul dient als Einstiegspunkt für alle Excel-bezogenen Importer. Es exportiert die Untermodule `load_importer` und `srl_importer`.

### 3.3. `src/excel/load_importer.rs`

Verantwortlich für das Laden von Lastgangdaten aus einer Excel-Datei.

* **Funktion:** `load_load_curve(path: &str) -> Result<Vec<LoadEntry>>`
* **Beschreibung:** Öffnet die Excel-Datei am angegebenen `path`. Es erwartet ein Arbeitsblatt namens "Lastgang". Es iteriert über die Zeilen (beginnend ab der zweiten Zeile, um Header zu überspringen) und parst den Zeitstempel (Spalte A) und die Leistung in kW (Spalte B). Die Daten werden als Vektor von `LoadEntry`-Strukturen zurückgegeben.
* **Abhängigkeiten:** `calamine`, `anyhow`, `crate::model::timeseries::LoadEntry`, `crate::utils::{parse_number, parse_timestamp_ymd}`.

### 3.4. `src/excel/srl_importer.rs`

Verantwortlich für das Laden von SRL-Daten (Sekundärregelleistung) aus einer Excel-Datei.

* **Funktion:** `load_srl(path: &str) -> Result<Vec<SRLEntry>>`
* **Beschreibung:** Öffnet die Excel-Datei am angegebenen `path`. Es erwartet ein Arbeitsblatt namens "Zeitreihen0h15". Es iteriert über die Zeilen (beginnend ab der dritten Zeile, um zwei Headerzeilen zu überspringen) und parst den Zeitstempel (Spalte A), positive Energie in kWh (Spalte G), negative Energie in kWh (Spalte H), positiven Preis in EUR/MWh (Spalte V) und negativen Preis in EUR/MWh (Spalte W). Die Daten werden als Vektor von `SRLEntry`-Strukturen zurückgegeben.
* **Abhängigkeiten:** `calamine`, `anyhow`, `crate::model::srl::SRLEntry`, `crate::utils::{parse_number, parse_timestamp_dmy}`.

### 3.5. `src/model/mod.rs`

Dieses Modul ist ein Container für alle Datenmodelle des Projekts. Es exportiert die Untermodule `srl`, `timeseries` und `mergedseries`.

### 3.6. `src/model/mergedseries.rs`

Definiert die Datenstruktur für zusammengeführte Zeitreihendaten, die als Eingabe für die Simulation dienen.

* **Struktur:** `MergedTick`
    * `timestamp`: Zeitpunkt des Ticks.
    * `power_kw`: Reale Last (Verbrauch positiv, Einspeisung negativ).
    * `srl_pos_kwh`: Angeforderte positive SRL-Energie (Abgabe).
    * `srl_neg_kwh`: Angeforderte negative SRL-Energie (Aufnahme).
    * `srl_pos_price_eur_mwh`: Preis für positive SRL.
    * `srl_neg_price_eur_mwh`: Preis für negative SRL.
* **Ableitungen:** `Debug`, `Serialize`, `Deserialize` (für CSV-Export/Import).

### 3.7. `src/model/srl.rs`

Definiert die Datenstruktur für eine einzelne SRL-Zeitreihe.

* **Struktur:** `SRLEntry`
    * `timestamp`: Zeitpunkt des Eintrags.
    * `pos_energy_kwh`: Positive SRL-Energie (Abgabe).
    * `neg_energy_kwh`: Negative SRL-Energie (Aufnahme).
    * `pos_price_eur_mwh`: Preis für positive SRL.
    * `neg_price_eur_mwh`: Preis für negative SRL.
* **Ableitungen:** `Debug`, `Serialize`.

### 3.8. `src/model/timeseries.rs`

Definiert die Datenstruktur für einen einzelnen Lastgang-Zeitreiheneintrag.

* **Struktur:** `LoadEntry`
    * `timestamp`: Zeitpunkt des Eintrags.
    * `power_kw`: Leistung in Kilowatt.
* **Ableitungen:** `Debug`, `Serialize`.

### 3.9. `src/simulation/mod.rs`

Dieses Modul ist ein Container für alle simulationsbezogenen Komponenten. Es exportiert die Untermodule `config`, `engine`, `summary` und `tick_result`.

### 3.10. `src/simulation/config.rs`

Definiert Konfigurationsparameter für die Simulation und die Struktur für die Simulationszusammenfassung.

* **Struktur:** `SimulationConfig`
    * `capacity_kwh`: Batteriekapazität in kWh.
    * `c_rate`: Lade-/Entladeleistung im Verhältnis zur Kapazität (z.B. 1.0 = 1C).
    * `efficiency`: Wirkungsgrad der Batterie (z.B. 0.95 für 95%).
    * `min_soc_frac`: Minimaler Ladezustand als Bruch (z.B. 0.1 für 10%).
    * `initial_soc_frac`: Initialer Ladezustand als Bruch (z.B. 0.5 für 50%).
    * `reserve_fraction`: Anteil der nutzbaren Kapazität, der für SRL reserviert ist (z.B. 0.3 für 30%).
    * `transformer_limit_kw`: Maximale erlaubte Netzbezugs/-einspeisegrenze in kW.
    * `timestep_minutes`: Zeitschritt der Simulation in Minuten (normalerweise 1.0).
    * `battery_price_per_kwh_chf`: Batteriekosten pro kWh in CHF (für Amortisationsrechnung).
    * `operating_cost_rate`: Betriebs-/Wartungskostenrate pro Jahr (% des Investments).
    * Implementiert `Default` für einfache Initialisierung.
* **Struktur:** `SimulationSummary`
    * Fasst die Gesamtergebnisse der Simulation zusammen, einschliesslich Energieflüsse, SoC-Extremwerte, Transformatorverletzungen, Erlöse, Einsparungen, Zyklenzahl und Amortisationszeit.
* **Ableitungen:** `Debug`, `Clone`, `Serialize`, `Deserialize` für `SimulationConfig`.

### 3.11. `src/simulation/engine.rs`

Enthält die Kernlogik der Batteriesimulation, die über jeden Zeitschritt iteriert.

* **Funktion:** `run_simulation(ticks: &[MergedTick], config: &SimulationConfig) -> Vec<SimulationTickResult>`
* **Beschreibung:**
    * Initialisiert den Batteriespeicher (Ladezustand, minimale/maximale Werte, SRL-Reserve).
    * Iteriert durch jeden `MergedTick`:
        * **SRL-Reaktion:** Berechnet, wie viel Energie für positive (Entladung) oder negative (Ladung) SRL geliefert oder aufgenommen werden kann, unter Berücksichtigung der maximalen Batteriekapazität und der für SRL reservierten Kapazität. Der SoC wird entsprechend angepasst.
        * **Erlöse aus SRL:** Berechnet die potenziellen Einnahmen aus der SRL-Teilnahme.
        * **Spitzenlastglättung (Peak Shaving):** Entlädt die Batterie, um Netzbezug zu reduzieren (wenn `power_kw > 0`), oder lädt die Batterie, um überschüssige Einspeisung zu absorbieren (wenn `power_kw < 0`). Dies muss die SoC-Grenzen und die SRL-Reserve beachten.
        * **SoC-Update und Grenzen:** Der Ladezustand wird nach beiden Schritten aktualisiert und innerhalb der definierten minimalen/maximalen Werte gehalten.
        * **Netzinteraktion:** Berechnet die resultierende Nettoleistung am Netzanschlusspunkt (`grid_net_kw`).
        * **Transformatorgrenzen:** Prüft, ob die Transformatorgrenze überschritten wurde.
        * Erfasst alle relevanten Werte für den aktuellen Zeitschritt in einer `SimulationTickResult`-Struktur.
    * Gibt einen Vektor aller `SimulationTickResult`-Strukturen zurück.
* **Abhängigkeiten:** `super::config::SimulationConfig`, `super::tick_result::SimulationTickResult`, `crate::model::mergedseries::MergedTick`.

### 3.12. `src/simulation/summary.rs`

Berechnet und druckt eine Zusammenfassung der gesamten Simulationsergebnisse.

* **Funktion:** `summarize(ticks: &[SimulationTickResult], config: &SimulationConfig) -> SimulationSummary`
* **Beschreibung:**
    * Initialisiert eine `SimulationSummary`-Struktur.
    * Aggregiert über alle `SimulationTickResult`-Einträge:
        * Gesamte SRL- und Peak-Shaving-Energieflüsse.
        * Minimale und maximale Ladezustände (SoC).
        * Anzahl der Transformatorverletzungen.
        * Gesamter SRL-Umsatz.
        * Berechnet monatliche Spitzenwerte vor und nach der Batteriesimulation, um die Einsparungen durch Peak Shaving zu ermitteln (basierend auf einem fixen Tarif von 10 CHF/kW/Monat).
        * Berechnet die geschätzte Batterielebensdauer in vollen Zyklen basierend auf dem Gesamtdurchsatz und der nutzbaren Kapazität.
        * Schätzt die Amortisationszeit in Jahren, basierend auf Investitionskosten (Batteriepreis pro kWh * Kapazität), Betriebskosten und den erzielten Gesamteinnahmen (SRL + Peak Shaving).
    * Gibt die ausgefüllte `SimulationSummary`-Struktur zurück.
* **Methode:** `SimulationSummary::print()`
    * Gibt die wichtigsten Kennzahlen der Simulation (Energieflüsse, SoC, Verstösse, Wirtschaftlichkeit, Zyklen) formatiert auf der Konsole aus.
* **Abhängigkeiten:** `super::config::{SimulationConfig, SimulationSummary}`, `super::tick_result::SimulationTickResult`, `chrono::Datelike`, `std::collections::HashMap`.

### 3.13. `src/simulation/tick_result.rs`

Definiert die Datenstruktur für die detaillierten Ergebnisse eines einzelnen Simulations-Zeitschritts.

* **Struktur:** `SimulationTickResult`
    * `timestamp`: Zeitpunkt des Ergebnisses.
    * **Inputs:** `original_power_kw`, `srl_pos_kwh`, `srl_neg_kwh`.
    * **Batterieverhalten:** `battery_in_kw` (Ladung), `battery_out_kw` (Entladung).
    * **SRL-Reaktion:** `srl_energy_in_kwh` (aufgenommene Energie), `srl_energy_out_kwh` (gelieferte Energie).
    * **Ladezustand:** `soc_kwh`, `soc_percent`.
    * **Netzleistung:** `grid_net_kw` (Nettoleistung am Netzanschlusspunkt nach Batterie und SRL).
    * **Grenzwert-Flag:** `transformer_violation` (true, wenn die Transformatorgrenze überschritten wurde).
    * **SRL-Erlös:** `srl_revenue_pos_chf`, `srl_revenue_neg_chf`.
    * `original_grid_kw`, `final_grid_kw`: Ursprüngliche und finale Netzleistung.
* **Ableitungen:** `Debug`, `Serialize` (für CSV-Export).

### 3.14. `src/utils/mod.rs`

Dieses Modul ist ein Container für verschiedene Hilfsfunktionen. Es exportiert die Untermodule `csv_export`, `datetime`, `interpolation` und `merging_csv`.

* **Funktion:** `file_exists(path: &str) -> bool`
    * **Beschreibung:** Prüft, ob eine Datei unter dem angegebenen Pfad existiert.
* **Funktion:** `parse_timestamp_dmy(cell: &DataType) -> Result<DateTime<Utc>>`
    * **Beschreibung:** Parst einen Zeitstempel aus einer Calamine `DataType`-Zelle. Erwartet das Format `TT.MM.JJJJ HH:MM` (Tag.Monat.Jahr Stunde:Minute) oder eine Excel-numerische Datumsdarstellung.
* **Funktion:** `parse_timestamp_ymd(cell: &DataType) -> Result<DateTime<Utc>>`
    * **Beschreibung:** Parst einen Zeitstempel aus einer Calamine `DataType`-Zelle. Erwartet das Format `JJJJ-MM-TT HH:MM` (Jahr-Monat-Tag Stunde:Minute) oder eine Excel-numerische Datums-/Zeitdarstellung.
* **Funktion:** `parse_number(cell: &DataType) -> Result<f64>`
    * **Beschreibung:** Parst einen Fliesskommazahl aus einer Calamine `DataType`-Zelle. Behandelt sowohl numerische als auch String-Werte, wobei Kommas als Dezimaltrennzeichen in Punkte umgewandelt werden.
* **Abhängigkeiten:** `chrono`, `calamine`, `anyhow`, `std::path::Path`.

### 3.15. `src/utils/csv_export.rs`

Stellt eine generische Funktion zum Speichern von Vektoren serieller Daten in einer CSV-Datei bereit.

* **Funktion:** `save_to_csv<T: Serialize>(path: &str, entries: &[T]) -> Result<()>`
* **Beschreibung:** Erstellt eine neue CSV-Datei am angegebenen `path`. Es iteriert über einen Vektor von Einträgen, die die `Serialize`-Eigenschaft implementieren, und schreibt jeden Eintrag als Zeile in die CSV-Datei.
* **Abhängigkeiten:** `std::fs::File`, `anyhow`, `csv`, `serde::Serialize`.

### 3.16. `src/utils/interpolation.rs`

Enthält Funktionen zur Generierung von Zeitrastern und zur linearen Interpolation von Zeitreihendaten.

* **Funktion:** `generate_time_grid(start: DateTime<Utc>, end: DateTime<Utc>, step_minutes: i64) -> Vec<DateTime<Utc>>`
    * **Beschreibung:** Erzeugt einen Vektor von `DateTime<Utc>`-Objekten in festen Minutenintervallen zwischen einem Start- und Endzeitpunkt.
* **Funktion:** `interpolate_scalar(t0: DateTime<Utc>, t1: DateTime<Utc>, v0: f64, v1: f64, target: DateTime<Utc>) -> f64`
    * **Beschreibung:** Führt eine lineare Interpolation eines skalaren Wertes zwischen zwei Zeitstempeln durch.
* **Funktion:** `interpolate_load_to_1min(input: &[LoadEntry], target_timestamps: &[DateTime<Utc>]) -> Vec<LoadEntry>`
    * **Beschreibung:** Interpoliert eine `LoadEntry`-Serie auf eine 1-Minuten-Auflösung, basierend auf den bereitgestellten Zielzeitstempeln. Es findet die nächstgelegenen vorherigen und nachfolgenden Punkte und interpoliert den `power_kw`-Wert.
* **Funktion:** `interpolate_srl_to_1min(input: &[SRLEntry], target_timestamps: &[DateTime<Utc>]) -> Vec<SRLEntry>`
    * **Beschreibung:** Interpoliert eine `SRLEntry`-Serie (Energie und Preise) auf eine 1-Minuten-Auflösung, basierend auf den bereitgestellten Zielzeitstempeln. Ähnlich wie bei der Lastgang-Interpolation werden alle relevanten Felder interpoliert.
* **Abhängigkeiten:** `chrono`, `crate::model::timeseries::LoadEntry`, `crate::model::srl::SRLEntry`.

### 3.17. `src/utils/merging_csv.rs`

Bietet eine Funktion zum Zusammenführen von interpolierten Last- und SRL-Zeitreihen.

* **Funktion:** `merge_1min_series(load: &[LoadEntry], srl: &[SRLEntry]) -> Vec<MergedTick>`
* **Beschreibung:** Nimmt einen Vektor von `LoadEntry`s und einen Vektor von `SRLEntry`s (beide müssen auf die gleiche Zeitauflösung, z.B. 1 Minute, interpoliert sein) und kombiniert sie zeilenweise zu einem Vektor von `MergedTick`-Strukturen. Dabei wird davon ausgegangen, dass die Zeitstempel der Eingangsvektoren übereinstimmen.
* **Abhängigkeiten:** `crate::model::timeseries::LoadEntry`, `crate::model::srl::SRLEntry`, `crate::model::mergedseries::MergedTick`.

## 4. Kernkonzepte und Ablauf

### 4.1. Datenimport und -aufbereitung

Das System beginnt mit dem Import von zwei Hauptdatensätzen aus Excel-Dateien:

* **Lastgangdaten (`input_wirkleistung.xlsx`):** Repräsentieren den tatsächlichen Energieverbrauch oder die Einspeisung.
* **SRL-Daten (`input_srl.xlsx`):** Enthalten Informationen über angeforderte Regelenergie (positive und negative) und deren Preise.

Diese Daten liegen oft in unterschiedlichen Zeitauflösungen vor (z.B. 15-Minuten-Intervalle). Die `interpolation`- und `merging_csv`-Module stellen sicher, dass alle Daten auf eine konsistente 1-Minuten-Auflösung gebracht und zu einer einzigen Zeitreihe (`MergedTick`) zusammengeführt werden, die als Eingabe für die Simulation dient.

### 4.2. Batteriesimulation (`simulation/engine.rs`)

Die Simulation läuft Zeitschritt für Zeitschritt ab. Für jeden 1-Minuten-Tick werden die folgenden Schritte ausgeführt:

1.  **SRL-Beantwortung:** Priorisiert die Reaktion auf SRL-Anfragen. Die Batterie versucht, die angeforderte positive (Entladung) oder negative (Ladung) Energie zu liefern, wobei die aktuelle Ladezustands-Grenzen (`min_soc_frac`, `capacity_kwh`) und eine für SRL reservierte Kapazität (`reserve_fraction`) berücksichtigt werden. Die Effizienz (`efficiency`) der Batterie wird angewendet.
2.  **Spitzenlastglättung (Peak Shaving):** Nach der SRL-Antwort wird die verbleibende Batteriekapazität für Peak Shaving genutzt. Wenn der Netzbezug hoch ist, entlädt die Batterie, um den Bezug zu reduzieren. Wenn eine Überschusseinspeisung vorliegt, lädt die Batterie, um diese zu absorbieren. Auch hier werden SoC-Grenzen und die verbleibende Leistung beachtet.
3.  **Ladezustands-Update:** Der Ladezustand (`soc_kwh`, `soc_percent`) der Batterie wird basierend auf allen Lade- und Entladevorgängen aktualisiert.
4.  **Netzwirkung und Transformatorgrenze:** Die Nettoleistung am Netzanschlusspunkt (`grid_net_kw`) wird berechnet. Es wird geprüft, ob diese den definierten `transformer_limit_kw` überschreitet, und Verstösse werden gezählt.
5.  **Ökonomische Erfassung:** Erlöse aus der SRL-Teilnahme werden erfasst.

### 4.3. Ergebniszusammenfassung (`simulation/summary.rs`)

Nachdem alle Zeitschritte simuliert wurden, wird eine Zusammenfassung erstellt:

* **Energieflüsse:** Gesamtmengen an Energie, die für SRL und Peak Shaving entladen oder geladen wurden.
* **Batteriezustand:** Minimale und maximale erreichte Ladezustände.
* **Netzqualität:** Anzahl der Überschreitungen der Transformatorgrenze.
* **Wirtschaftlichkeit:**
    * Gesamter SRL-Umsatz.
    * Einsparungen durch Peak Shaving, berechnet anhand der Reduzierung monatlicher Leistungsspitzen.
    * Geschätzte Amortisationszeit der Batterieanlage.
* **Batterielebensdauer:** Geschätzte Anzahl der vollen Batteriezyklen, die während der Simulationsperiode durchlaufen wurden.

### 4.4. Datenexport (`utils/csv_export.rs`)

Alle Zwischen- und Endergebnisse können in CSV-Dateien exportiert werden, um eine weitere Analyse oder Visualisierung zu ermöglichen. Dazu gehören die bereinigten Eingangsdaten und die detaillierten Simulationsergebnisse pro Tick.

## 5. Konfiguration

Die Simulation kann über die `SimulationConfig`-Struktur angepasst werden. Wichtige Parameter umfassen:

* **Batterieeigenschaften:** Kapazität, C-Rate, Effizienz, minimale und initiale Ladezustände.
* **Betriebsstrategie:** Der `reserve_fraction` ist entscheidend, da er definiert, wie viel von der nutzbaren Batteriekapazität ausschliesslich für SRL-Dienste reserviert wird und somit nicht für Peak Shaving zur Verfügung steht.
* **Anlagenbegrenzungen:** Der `transformer_limit_kw` definiert die maximal zulässige Leistung am Netzanschlusspunkt.
* **Wirtschaftlichkeit:** Der Batteriepreis und die Betriebskostenrate beeinflussen die Amortisationsberechnung.

## 6. Datenformate

* **Input-Excel-Dateien:** Erwarten spezifische Blattnamen ("Lastgang", "Zeitreihen0h15") und Spaltenzuordnungen (siehe Importer-Module).
* **Output-CSV-Dateien:** Werden direkt aus den Rust-Strukturen serialisiert, was eine einfache Lesbarkeit und Weiterverarbeitung ermöglicht.

## 7. Fehlerbehandlung

Das Projekt verwendet `anyhow::Result` für eine robuste Fehlerbehandlung, um Probleme beim Dateizugriff, Daten-Parsing oder anderen Operationen effektiv zu melden.

## 8. Bibliotheken

* `calamine`: Zum Lesen von Excel-Dateien.
* `chrono`: Für Datums- und Zeitoperationen.
* `serde`: Zum Serialisieren/Deserialisieren von Datenstrukturen (insbesondere für CSV-Ein- und -Ausgabe).
* `csv`: Zum Arbeiten mit CSV-Dateien.
* `anyhow`: Für vereinfachtes Fehlerhandling.
