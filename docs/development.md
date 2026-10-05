# Development and local checks

The public application consists of the server modules under `src/server/` and
`src/bin/wyoming-probe.rs`. There are two explicitly declared Cargo binaries;
historical local benchmark code is excluded and automatic example discovery is
disabled. Dependencies remain pinned by Cargo.lock. No additional runtime was added.

The Docker build runs `cargo test --locked` and `cargo build --release --locked
--bins` using the matching native sherpa archive. Five unit tests cover protocol
framing/limits and model selection. The probe additionally verifies Describe and
can send existing WAV audio in paced 20 ms chunks with intentionally fragmented
TCP writes. No regression audio is distributed because its redistribution
provenance has not been established.

For local STT checks, mount your own 16 kHz mono PCM16 WAV directory read-only
at `/test` in a separate test container, then run:

```sh
docker exec YOUR_TEST_CONTAINER /usr/local/bin/wyoming-probe --smoke 127.0.0.1:10321 /test/01.wav
```

Wait for Docker's healthy state before probing: startup includes file hashing and
model loading. `--smoke-fast` performs an unpaced throughput check. File names
`01.wav` and `03.wav` have built-in expected command checks; use other names for
arbitrary audio and inspect the returned transcript.

Public source preparation was verified with two consecutive requests, model
loading once, partial results before end-of-input, and graceful SIGTERM shutdown.
This check does not replace the previous real Home Assistant test.

Local model weights, audio, results, research, caches, vendored snapshots and
historical scripts are intentionally ignored. Do not use `git add -f` on them.
No publish workflow, credentials, remote, release or registry upload is configured.
