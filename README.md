<p align="center">
  <img src="icons/kroko-stt-german-wyoming.png" alt="Kroko STT German Wyoming icon" width="160" height="160">
</p>

# Kroko STT German Wyoming

German streaming speech-to-text with Kroko, sherpa-onnx, Rust and Wyoming.
Built for Home Assistant and efficient CPU-only operation.

This project runs German online speech recognition locally on linux/amd64,
including efficient systems such as the Intel N100. Audio chunks are decoded
as they arrive using Rust and sherpa-onnx's native OnlineRecognizer. No Python
service, GPU or second inference runtime is required. Inference needs no network.
The classic model and the existing Wyoming service have been tested with Home Assistant.

```text
Microphone / Home Assistant
        ↓
Wyoming
        ↓
Rust server
        ↓
sherpa-onnx OnlineRecognizer
        ↓
Kroko streaming model
```

## Models

Model weights are external and are never included in this repository or image.
Set `KROKO_MODEL` to one of these profiles; the default is `classic`:

| Selection | Directory inside container | Model |
| --- | --- | --- |
| `classic` | `/models/classic/` | sherpa-onnx-streaming-zipformer-de-kroko-2025-08-06 |
| `community-64` | `/models/community-64/` | Kroko-DE-Community-64-L-Streaming-001 |
| `community-128` | `/models/community-128/` | Kroko-DE-Community-128-L-Streaming-001 |

Each directory must provide these validated runtime artifacts:
`encoder.onnx`, `decoder.onnx`, `joiner.onnx`, `tokens.txt`.
Only one recognizer/model is loaded. Startup verifies every selected file against
pinned SHA256 values in [src/server/models.rs](src/server/models.rs).
Missing files, wrong hashes and unknown selections cause a clear startup error.
Arbitrary newer exports are not accepted.

Classic export revision: `887db3d083240198c2d2b99fb66cfcfe6948ced8`.
Community source revision: `d45212aeb212dd66083dd22710c9954f40ff8cc1`.
See [docs/models.md](docs/models.md) for sources and extraction requirements.

To switch models, change the environment variable and recreate the container.
`docker restart` alone does not change its environment. The Wyoming name remains
`kroko`, so selecting another model does not require three separate integrations.

## Local Docker build and run

The Dockerfile pins Rust 1.90.0, sherpa-onnx 1.13.8, the native archive checksum
and base image digests. Cargo dependencies are locked. Building requires internet
access; runtime inference does not. The final distroless image runs as UID 65532,
contains no models, audio, Python or build tools, and uses CPU only.
This is a local build path: binary/image redistribution requires the license
review described in [docs/licensing.md](docs/licensing.md).

```sh
./scripts/build-image.sh
MODELS_DIR=/absolute/path/to/your/models ./scripts/run-local.sh
```

The helper defaults to loopback port 10321 and refuses to replace an existing
container. For a remote Home Assistant, choose a free port and a reachable bind
address explicitly, for example `BIND_ADDRESS=0.0.0.0 HOST_PORT=10321`.
Restrict exposure to your trusted LAN: Wyoming TCP has no authentication or TLS.
Models are mounted read-only; no Docker socket or privileged execution is used.

Add **Wyoming Protocol** in Home Assistant using the Docker host's reachable
address and chosen host port. Select `kroko` as the STT provider in your Assist
pipeline. This project never configures Home Assistant automatically.

| Environment | Container default | Meaning |
| --- | --- | --- |
| `KROKO_MODEL` | `classic` | Selected profile |
| `MODEL_DIR` | `/models` | Parent of the three model directories |
| `HOST` | `0.0.0.0` | Container listener address |
| `PORT` | `10321` | Wyoming TCP port |
| `NUM_THREADS` | `1` | Inference threads, allowed 1–4 |
| `LANGUAGE` | `de` | `de` or `de-DE`; models are monolingual German |
| `LOG_LEVEL` | `info` | `error`, `warn`, `info`, `debug` |
| `TZ` | `Europe/Berlin` | Container timezone |

Direct executable defaults differ: `MODEL_DIR=models`, `HOST=127.0.0.1`,
`PORT=10300`. Native development needs the matching sherpa library directory:
`SHERPA_ONNX_LIB_DIR=/absolute/path/to/native/lib cargo test --locked`, followed
by `cargo build --release --locked --bins`.

## Protocol, streaming and privacy

The server answers `describe` with `info`, accepts `transcribe`, `audio-start`,
multiple `audio-chunk` events, then `audio-stop`, and returns `transcript`.
Audio must be 16 kHz, mono, signed 16-bit little-endian PCM. No resampler is included.
TCP fragmentation is handled independently of event boundaries.
Audio is processed incrementally and discarded; the service does not record it.
There is no internal VAD: the client must end the request with `audio-stop`.
One active inference is serialized; connections are bounded. Graceful shutdown
may wait up to the connection inactivity timeout before workers finish.

Partial results are internal diagnostics, not proprietary Home Assistant events.
At `info`, final transcripts appear in logs; `debug` also logs changing partials.
Use `LOG_LEVEL=warn` to avoid normal transcript logs, and consider Docker log
retention when handling private speech. Logs are not audio recordings.

The included `wyoming-probe` is a healthcheck and optional local WAV smoke client:

```sh
docker exec kroko-stt-german-wyoming-local /usr/local/bin/wyoming-probe
# With a separately read-only-mounted 16 kHz mono PCM16 WAV:
# wyoming-probe --smoke 127.0.0.1:10321 /test/audio.wav
```

## Reference performance

These are our own Intel N100 control measurements with one inference thread,
not guaranteed performance. Hardware, audio, threading and build configuration
change the results. RTF is inference time divided by audio duration.

| Model | RTF | RSS after startup |
| --- | --- | --- |
| classic | ~0.06–0.07 | ~131–133 MiB |
| community-64 | ~0.082 | ~246 MiB |
| community-128 | ~0.122 | ~245 MiB |

Classic is currently our recommended N100 variant. These short control runs do
not establish superiority on unrestricted speech or guarantee command accuracy.

## Licensing and acknowledgements

Our original source code is Apache-2.0; see [LICENSE](LICENSE) and [NOTICE](NOTICE).
Dependency and model licenses are separate; see [docs/licensing.md](docs/licensing.md).
In particular, classic weights are **not claimed to be freely redistributable**.
Community model notices also require review of the exact applicable terms.
No model weights may be inferred to inherit this project's Apache license.

Thanks to [Kroko / Banafo](https://github.com/Banafo),
[sherpa-onnx](https://github.com/k2-fsa/sherpa-onnx),
[Wyoming](https://github.com/rhasspy/wyoming) and
[Home Assistant](https://www.home-assistant.io/).
These credits imply no official partnership, support or endorsement.
