<img src="icons/kroko-stt-german-wyoming.png" alt="Kroko STT German Wyoming Icon" width="160" height="160">

# Kroko STT German Wyoming

Deutsche Streaming-Spracherkennung mit Kroko, sherpa-onnx, Rust und Wyoming.
Entwickelt für Home Assistant und einen effizienten CPU-Betrieb.

Dieses Projekt ermöglicht lokale deutsche Online-Spracherkennung auf linux/amd64,
auch auf sparsamen Systemen wie dem Intel N100. Audioblöcke werden direkt beim
Eintreffen mit Rust und dem nativen OnlineRecognizer von sherpa-onnx verarbeitet.
Ein Python-Dienst, eine GPU oder eine zweite Inferenz-Laufzeit werden nicht benötigt.
Für die Spracherkennung selbst ist keine Netzwerkverbindung erforderlich.
Das Classic-Modell und der Wyoming-Dienst wurden mit Home Assistant getestet.

**Unterstützte Sprache:** ausschließlich Deutsch (`de` / `de-DE`).

```text
Mikrofon / Home Assistant
        ↓
Wyoming
        ↓
Rust-Server
        ↓
sherpa-onnx OnlineRecognizer
        ↓
Kroko Streaming-Modell
```

## Modelle

Die Modellgewichte werden extern bereitgestellt und sind weder Bestandteil dieses
Repositories noch des Images. Über `KROKO_MODEL` wird eines der folgenden Profile
ausgewählt; Standard ist `classic`:

| Auswahl | Verzeichnis im Container | Modell |
| --- | --- | --- |
| `classic` | `/models/classic/` | sherpa-onnx-streaming-zipformer-de-kroko-2025-08-06 |
| `community-64` | `/models/community-64/` | Kroko-DE-Community-64-L-Streaming-001 |
| `community-128` | `/models/community-128/` | Kroko-DE-Community-128-L-Streaming-001 |

Jedes Verzeichnis muss die geprüften Laufzeitdateien
`encoder.onnx`, `decoder.onnx`, `joiner.onnx` und `tokens.txt` enthalten.
Es wird immer nur ein Recognizer bzw. Modell geladen. Beim Start werden alle
Dateien des ausgewählten Profils anhand der in
[src/server/models.rs](src/server/models.rs) hinterlegten SHA256-Werte geprüft.
Fehlende Dateien, falsche Prüfsummen oder unbekannte Modellnamen führen zu einer
klaren Fehlermeldung und zum Abbruch. Beliebige neuere Exporte werden nicht
automatisch akzeptiert.

Classic-Export-Revision: `887db3d083240198c2d2b99fb66cfcfe6948ced8`.
Community-Quellrevision: `d45212aeb212dd66083dd22710c9954f40ff8cc1`.
Quellen und Hinweise zur Bereitstellung der Modelle stehen in
[docs/models.md](docs/models.md).

Für eine frische Unraid-Installation ist `classic` das empfohlene Standardmodell.
Die Modellanleitung enthält einen vollständigen Copy-&-Paste-Befehlsblock für
den manuellen Download der vier Dateien aus der exakt gepinnten Originalquelle,
ihre SHA256-Prüfung und Leserechte für UID 65532. Diesen Schritt **vor dem ersten
Containerstart** ausführen; weder Image noch Server laden Modelle automatisch.
Nutzer müssen die Bedingungen der Originalquelle prüfen und akzeptieren.
Bei unklaren Nutzungsrechten ist eine Klärung mit dem Rechteinhaber erforderlich;
eine freie Weiterverteilung der Classic-Gewichte wird nicht behauptet.

`community-64` und `community-128` benötigen separat vorbereitete vollständige,
hashgeprüfte Modellkomponenten. Eine `.data`-Datei allein genügt nicht; der Server
extrahiert oder installiert sie nicht automatisch. Alle Modellgewichte bleiben
außerhalb des Repositories und Docker-Images im persistenten Modellordner.

Zum Wechseln des Modells muss die Umgebungsvariable geändert und der Container
neu erstellt werden. Ein `docker restart` allein ändert die Umgebung nicht.
Der Wyoming-Name bleibt `kroko`, sodass für die Modellprofile keine getrennten
Home-Assistant-Integrationen erforderlich sind.

## Lokaler Docker-Build und Start

Das Dockerfile verwendet festgelegte Versionen bzw. Prüfsummen für Rust 1.90.0,
sherpa-onnx 1.13.8, dessen Quellarchiv und die Basis-Images. Cargo-Abhängigkeiten
sind gesperrt. Für den Build ist Internetzugriff erforderlich, für die eigentliche
Spracherkennung zur Laufzeit nicht. Das finale Distroless-Image läuft als UID
65532, enthält keine Modelle, Audiodateien, Python-Laufzeit oder Build-Werkzeuge
und nutzt ausschließlich die CPU.

Dieser Weg ist derzeit für lokale Builds vorgesehen. Für die öffentliche
Weiterverteilung eines fertigen Binaries bzw. Images ist die Lizenzprüfung aus
[docs/licensing.md](docs/licensing.md) zu beachten.

Der native Build deaktiviert TTS und Sprecherdiarisierung und verwendet das
offizielle Rust-Feature `shared`. eSpeak NG, Piper-Phonemisierung und ucd-tools
werden nicht eingebunden. Das Image bewahrt die Lizenztexte der Abhängigkeiten
und enthält die exakten glibc-/Eigen-Quellen als Lizenz-Compliance-Material unter
`/usr/share/licenses/kroko/dependencies/corresponding-source`.
Diese Quellarchive sind keine ASR-Modelle oder kompilierten Build-Artefakte.

```sh
./scripts/build-image.sh
MODELS_DIR=/absolute/path/to/your/models ./scripts/run-local.sh
```

Das Hilfsskript verwendet standardmäßig den Loopback-Port 10321 und ersetzt
keinen bereits vorhandenen Container. Soll Home Assistant von einem anderen
Rechner zugreifen, müssen ein freier Port und eine erreichbare Bind-Adresse
angegeben werden, zum Beispiel `BIND_ADDRESS=0.0.0.0 HOST_PORT=10321`.

Wyoming TCP bietet selbst keine Authentifizierung und kein TLS. Der Dienst sollte
daher nur im vertrauenswürdigen lokalen Netzwerk erreichbar sein. Modelle werden
schreibgeschützt eingebunden; Docker-Socket und privilegierte Ausführung werden
nicht benötigt.

In Home Assistant die Integration **Wyoming Protocol** mit der erreichbaren
Adresse des Docker-Hosts und dem gewählten Port hinzufügen. Anschließend `kroko`
als STT-Anbieter in der Assist-Pipeline auswählen. Dieses Projekt verändert die
Home-Assistant-Konfiguration nicht automatisch.

| Umgebungsvariable | Container-Standard | Bedeutung |
| --- | --- | --- |
| `KROKO_MODEL` | `classic` | Ausgewähltes Modellprofil |
| `MODEL_DIR` | `/models` | Übergeordnetes Verzeichnis der drei Modellordner |
| `HOST` | `0.0.0.0` | Listener-Adresse im Container |
| `PORT` | `10321` | Wyoming-TCP-Port |
| `NUM_THREADS` | `1` | Inferenz-Threads, zulässig sind 1–4 |
| `LANGUAGE` | `de` | `de` oder `de-DE`; die Modelle sind ausschließlich deutschsprachig |
| `LOG_LEVEL` | `info` | `error`, `warn`, `info`, `debug` |
| `TZ` | `Europe/Berlin` | Zeitzone des Containers |

Die Standardwerte bei direkter Ausführung unterscheiden sich:
`MODEL_DIR=models`, `HOST=127.0.0.1`, `PORT=10300`.
Für die native Entwicklung wird das passende sherpa-Bibliotheksverzeichnis benötigt:

```sh
SHERPA_ONNX_LIB_DIR=/absolute/path/to/native/lib cargo test --locked
SHERPA_ONNX_LIB_DIR=/absolute/path/to/native/lib cargo build --release --locked --bins
```

Verwende die TTS-freien Shared-Bibliotheken aus dem Docker-Build. Beim nativen
Start muss ihr Verzeichnis außerdem über `LD_LIBRARY_PATH` erreichbar sein;
im Container ist dies bereits konfiguriert.

## Protokoll, Streaming und Datenschutz

Der Server beantwortet `describe` mit `info`, akzeptiert `transcribe`,
`audio-start`, mehrere `audio-chunk`-Ereignisse und anschließend
`audio-stop`. Danach wird `transcript` zurückgegeben.

Audio muss als 16 kHz, Mono, vorzeichenbehaftetes 16-Bit-Little-Endian-PCM
vorliegen. Ein Resampler ist nicht enthalten. TCP-Fragmentierung wird unabhängig
von den Ereignisgrenzen verarbeitet. Audio wird fortlaufend verarbeitet und
anschließend verworfen; der Dienst zeichnet keine Audiodaten auf.

Es gibt keine interne VAD. Der Client muss die Anfrage mit `audio-stop`
abschließen. Es läuft höchstens eine aktive Inferenz; eine gleichzeitige zweite
STT-Anfrage erhält einen `busy`-Fehler. Die Zahl der Verbindungen ist begrenzt.
Beim sauberen Beenden kann der Server bis zum Verbindungs-Timeout
warten, bevor Worker beendet werden.

Teilergebnisse dienen internen Diagnosezwecken und sind keine proprietären
Home-Assistant-Ereignisse. Bei `info` werden finale Transkripte protokolliert;
`debug` protokolliert zusätzlich geänderte Teilergebnisse. Mit
`LOG_LEVEL=warn` lassen sich normale Transkript-Logs vermeiden. Bei privaten
Sprachdaten sollte zusätzlich die Docker-Log-Aufbewahrung berücksichtigt werden.
Die Logs enthalten keine Audioaufzeichnungen.

Der mitgelieferte `wyoming-probe` dient als Healthcheck und optional als lokaler
WAV-Smoke-Test:

```sh
docker exec kroko-stt-german-wyoming-local /usr/local/bin/wyoming-probe
# Mit einer separat schreibgeschützt eingebundenen 16-kHz-Mono-PCM16-WAV:
# wyoming-probe --smoke 127.0.0.1:10321 /test/01.wav
```

Der Smoke-Test prüft die bekannten Regressionssätze in `01.wav` (Licht im
Wohnzimmer) und `03.wav` (Temperatur auf zweiundzwanzig Grad). Er ist kein
allgemeines WAV-Transkriptionswerkzeug. Testaufnahmen werden nicht mitgeliefert.

## Referenzleistung

Die folgenden Werte stammen aus eigenen Kontrollmessungen auf einem Intel N100
mit einem Inferenz-Thread und sind keine Leistungsgarantie. Hardware, Audio,
Threading und Build-Konfiguration beeinflussen die Ergebnisse. RTF bezeichnet
das Verhältnis von Inferenzzeit zu Audiodauer.

| Modell | RTF | RSS nach dem Start |
| --- | --- | --- |
| classic | ~0.06–0.07 | ~131–133 MiB |
| community-64 | ~0.082 | ~246 MiB |
| community-128 | ~0.122 | ~245 MiB |

Für den Intel N100 ist `classic` derzeit unsere empfohlene Variante. Die kurzen
Kontrolltests belegen keine generelle Überlegenheit bei beliebiger Spracheingabe
und garantieren keine fehlerfreie Befehlserkennung.

## Lizenzierung

Der von diesem Projekt selbst entwickelte Quellcode steht unter Apache-2.0;
siehe [LICENSE](LICENSE) und [NOTICE](NOTICE).

Abhängigkeiten und Modelle besitzen eigene, davon unabhängige Lizenzbedingungen;
siehe [docs/licensing.md](docs/licensing.md). Insbesondere wird **nicht behauptet,
dass die Classic-Modellgewichte frei weiterverbreitet werden dürfen**. Auch bei
den Community-Modellen müssen die jeweils tatsächlich anwendbaren Bedingungen
beachtet werden. Modellgewichte übernehmen nicht automatisch die Apache-2.0-Lizenz
dieses Projekts.

## Danksagungen

Dieses Projekt baut auf der Arbeit mehrerer Open-Source-Projekte und Communities auf:

- **Kroko / Banafo** – für die deutschen Kroko-Streaming-Spracherkennungsmodelle.
- **sherpa-onnx / k2-fsa** – für die effiziente Streaming-ASR-Laufzeit und den
  OnlineRecognizer.
- **Wyoming** – für das leichtgewichtige Sprachassistenten-Protokoll zur
  Integration mit Home Assistant.
- **Home Assistant** – für die Open-Source-Heimautomatisierungsplattform und das
  Assist-Ökosystem.

Offizielle Projekte:
[Kroko / Banafo](https://github.com/Banafo) ·
[sherpa-onnx](https://github.com/k2-fsa/sherpa-onnx) ·
[Wyoming](https://github.com/OHF-Voice/wyoming) ·
[Home Assistant](https://www.home-assistant.io/)

Kroko/Banafo, sherpa-onnx, Wyoming und Home Assistant sind unabhängige Projekte.
Ihre Namen und Marken gehören den jeweiligen Rechteinhabern. Die Danksagungen
begründen keine Partnerschaft, Unterstützung oder Empfehlung durch diese Projekte.
