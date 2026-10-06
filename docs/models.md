# External, pinned models

Use only artifacts whose four SHA256 values match [src/server/models.rs](../src/server/models.rs).
The directory names are profiles; `MODEL_DIR` is their parent. The loader does
not download models, extract archives, or automatically upgrade exports.

Classic original source:
https://huggingface.co/csukuangfj/sherpa-onnx-streaming-zipformer-de-kroko-2025-08-06/tree/887db3d083240198c2d2b99fb66cfcfe6948ced8

## Classic für eine frische Unraid-Installation vorbereiten

**Vor dem ersten Containerstart ausfuehren.** Es gibt keinen automatischen
Modelldownload. Die Lizenz der Gewichte ist unabhaengig von Apache-2.0; fuer
Classic wird keine freie Weiterverteilungsfreigabe behauptet. Du musst die
Bedingungen der Originalquelle vor dem Download prüfen und akzeptieren.
Bei unklaren Nutzungsrechten hole eine Klärung vom Rechteinhaber ein. Die Befehle
laden nur fuer deinen eigenen lokalen Modellordner direkt von der gepinnten
Upstream-Quelle; das Projekt liefert keine Gewichte aus.

Die Dateien in dieser Revision heissen bereits encoder.onnx, decoder.onnx,
joiner.onnx und tokens.txt. Kein Archiv, Konverter oder weiteres Runtime-Paket
ist fuer Classic erforderlich. Folgende Befehle im Unraid-Terminal ausfuehren
(benoetigt curl und sha256sum, beide auf dem getesteten Unraid vorhanden):

```sh
(
set -eu
umask 022
model_dir=/mnt/user/appdata/kroko-stt-german-wyoming/models/classic
source_url=https://huggingface.co/csukuangfj/sherpa-onnx-streaming-zipformer-de-kroko-2025-08-06/resolve/887db3d083240198c2d2b99fb66cfcfe6948ced8
mkdir -p "$model_dir"
# Vorhandene Dateien und Downloads niemals stillschweigend ersetzen.
for file in encoder.onnx decoder.onnx joiner.onnx tokens.txt; do
  for path in "$model_dir/$file" "$model_dir/$file.download"; do
    if [ -e "$path" ] || [ -L "$path" ]; then
      echo "Abbruch: Datei bereits vorhanden: $path" >&2
      exit 1
    fi
  done
done
for file in encoder.onnx decoder.onnx joiner.onnx tokens.txt; do
  curl --fail --location --retry 3 --proto '=https' --proto-redir '=https' \
    "$source_url/$file" -o "$model_dir/$file.download"
done
cd "$model_dir"
sha256sum --check <<'SHA256'
6e83993d6967ec7a3498b055b7e85ace85b5d64d1b1e8773cb29a43a11f5edb5  encoder.onnx.download
94a29592b403c53fa2231b478637da1ab4abcef7f5e46e432098416a4a3ed562  decoder.onnx.download
28356bff070aea51ab1d725a3278e81d19f9300f860d3248a7014292264df15a  joiner.onnx.download
86e8370994ff2c01149ba8c4f8709aa93cdc18914b27a717e291e96faf39a6eb  tokens.txt.download
SHA256
for file in encoder.onnx decoder.onnx joiner.onnx tokens.txt; do
  # Noclobber verhindert auch ein Überschreiben bei einem parallelen Download.
  (set -C; cat "$file.download" > "$file")
  chmod 644 "$file"
  rm -- "$file.download"
done
chmod 755 "$model_dir" "$(dirname "$model_dir")"
echo "Classic bereit: vier SHA256-Prüfungen erfolgreich; Modelle lesbar für UID 99 / GID 100."
)
```

Alle vier Prüfungen müssen **OK** melden. `set -e` bricht bei einem Download-
oder Prüfsummenfehler ab, bevor Dateien unter ihren Laufzeitnamen installiert werden.
Die Klammern begrenzen die Shell-Einstellungen auf diesen Befehlsblock.
Bei Fehlern nicht starten: .download-Dateien sind keine installierten Modelle.
Der Block überschreibt keine vorhandenen Dateien. Nach einem abgebrochenen
Versuch nur die selbst angelegten unvollständigen .download-Dateien prüfen,
bevor du den Block erneut ausführst; vorhandene Modelle nicht blind löschen. Die Befehle
koennen keine Lizenzrechte erteilen. Sie richten keine automatische
Weiterverteilung ein. Falls vorhandene Elternordner den Zugriff verhindern,
deren Durchsuchbarkeit für UID 99 / GID 100 gezielt prüfen; weder pauschal
Appdata-Rechte ändern noch chmod 777 verwenden. Der Block setzt ausschließlich
die beiden eingebundenen Modellordner auf 755 und die vier Dateien auf 644.
Ein chown auf UID 99 / GID 100 ist dafür nicht nötig.

Erwartete Struktur:

```text
/mnt/user/appdata/kroko-stt-german-wyoming/models/
  classic/
    encoder.onnx
    decoder.onnx
    joiner.onnx
    tokens.txt
```

Im Unraid-Template diesen uebergeordneten models-Ordner **read-only** nach
/models mappen und KROKO_MODEL=classic belassen. Vor Apply einen freien
Hostport auswaehlen; Container-Port bleibt 10321/tcp. Anschliessend auf healthy
warten und Home Assistant manuell ueber Wyoming mit Unraid-IP und Hostport
verbinden. Keine WebUI. Ein vorhandener Dienst darf nicht verdraengt werden.

Wenn du bereits die vier richtigen Dateien besitzt, ist kein Download noetig:
am gewaehlten Ziel in classic/ bereitstellen und mit den vier obigen Hashes
(ohne .download-Suffix) pruefen. Beim Umzug nie bestehende Dateien blind ersetzen.
Bei fehlendem Ordner/Datei oder SHA256-Abweichung bricht der Dienst mit dem
konkreten Pfad im Log ab. Modelle bleiben bei Container-/Image-Updates erhalten.

## Manuell vorbereitete Community-Alternativen

Community original source:
https://huggingface.co/Banafo/Kroko-ASR/tree/d45212aeb212dd66083dd22710c9954f40ff8cc1

The pinned classic source already provides the validated INT8 encoder/decoder/joiner
as `encoder.onnx`, `decoder.onnx`, `joiner.onnx`; tokens remain `tokens.txt`.
Names alone do not establish compatibility: the startup hash check is authoritative.

Community downloads are `Kroko-DE-Community-64-L-Streaming-001.data` and
`Kroko-DE-Community-128-L-Streaming-001.data`. Their original SHA256 values are:

| File | SHA256 |
| --- | --- |
| 64-L .data | `0013d4b3e1216b4e0c15f18f345aa58ca8bb2cdbeff7a7d55a513f6b400862a5` |
| 128-L .data | `8d39babf998aba69446b1c1e3ac780bf5f90887ad95b0d60eb4bd79d63998462` |

They are containers, not ONNX files. Our local compatibility work extracted the
original encoder/decoder/joiner bytes and vocabulary without changing ONNX graphs.
Only the extracted four validated files are runtime inputs. The raw .data file
is not required by the server. This public source preparation does not distribute
an extraction tool or grant permission to distribute the resulting model files.
Use the independently prepared, hash-verified files; obtain clarification of model
terms before sharing them. See [docs/licensing.md](licensing.md).

Fuer Community-Alternativen zuerst die vier separat vorbereiteten, passenden
Dateien in community-64/ bzw. community-128/ unter dem models-Ordner ablegen
und gegen die Hashes in src/server/models.rs pruefen. Die .data-Datei allein
genuegt nicht; der v0.1.0-Server entpackt sie nicht. Das Dropdown im CA-Template
installiert keine Modelle. Fuer neue Nutzer ist daher Classic mit der obigen
direkten Downloadanleitung der vollstaendig dokumentierte Standardweg.

Classic and community-64 use encoder `decode_chunk_len=128`, `T=141` and a
1.28-second feature step. Community-128 uses `decode_chunk_len=256`, `T=269`,
a 2.56-second feature step. These are algorithmic model steps, not network chunk
sizes. All accept 16 kHz mono PCM16 through this server with 80-dimensional
features. The profiles use different validated final padding lengths.

The documented community container format is: a little-endian uint32 JSON-header
length, that header (`type=zipformer2`, `free=true`), followed by four
little-endian uint32 length-prefixed blocks in encoder/decoder/joiner/tokens order.
Reject truncated data, non-free/encrypted packages and trailing bytes. Verify both
the download checksum and each extracted file against the profile hashes.
The format was checked against Banafo's official
[ModelData.cc](https://github.com/kroko-ai/Kroko-ONNX/blob/f572b3b9494bef692142e05f3e1f0120845109b5/sherpa-onnx/csrc/ModelData.cc).
No custom inference runtime is needed once the exact components are extracted.
