# Public-source preparation: local verification

Checked 2026-10-05 on Intel N100, CPU only, with the pinned classic export.
The existing Home Assistant service was not changed. This is a short compatibility
check, not a new benchmark or a publication/license approval.

| Check | Result |
| --- | --- |
| Rust | 1.90.0, cargo test --locked: 5 passed; release build --locked successful |
| Native runtime | sherpa-onnx 1.13.8 / ONNX Runtime 1.28.2 |
| Docker | linux/amd64; ~58.21 MiB image; UID 65532; no model weights |
| Isolation | Separate container, network=none, no host port, read-only model/audio mounts |
| Describe | info response; ASR and model name kroko; de/de-DE |
| Model loading | Once; 3.074 seconds excluding preceding hash verification; RSS 132.38 MiB |
| Streaming | Incremental decode and partials before audio-stop; deliberately fragmented TCP writes |
| Shutdown | SIGTERM, shutdown logged, exit code 0 |

| Expected command | Transcript | Paced end-to-end | Last chunk to transcript |
| --- | --- | --- | --- |
| Schalte das Licht im Wohnzimmer ein. | Schalte das Licht im Wohnzimmer ein | 2.309 s | 69.1 ms |
| Stelle die Temperatur im Wohnzimmer auf zweiundzwanzig Grad. | Stelle die Temperatur im Wohnzimmer auf zweiundzwanzig Grad | 3.806 s | 65.8 ms |

Both sequential requests passed normalized expected-text checks. Paced durations
include audio transmission, so they are not throughput RTF measurements.
Docker's idle memory measurement was approximately 123 MiB; that metric differs
from process RSS and the historical reference measurements in README.md.

Historical warnings in this run: the probe's shared protocol module produced
unused-code warnings (now scoped out in that client only). An initial probe
before startup completed received connection refused;
Describe and STT passed after the container became healthy. The host shell also
reports an unavailable C.UTF-8 locale. None was an ASR runtime error.

The clean build downloads pinned dependencies. Base digests, lockfile and native
archive SHA256 constrain inputs; this is not a claim of bit-identical builds.
See [docs/licensing.md](licensing.md) for outstanding native binary and model
redistribution review.
