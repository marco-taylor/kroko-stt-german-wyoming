# External, pinned models

Use only artifacts whose four SHA256 values match [src/server/models.rs](../src/server/models.rs).
The directory names are profiles; `MODEL_DIR` is their parent. The loader does
not download models, extract archives, or automatically upgrade exports.

Classic original source:
https://huggingface.co/csukuangfj/sherpa-onnx-streaming-zipformer-de-kroko-2025-08-06/tree/887db3d083240198c2d2b99fb66cfcfe6948ced8

Community original source:
https://huggingface.co/Banafo/Kroko-ASR/tree/d45212aeb212dd66083dd22710c9954f40ff8cc1

The existing validated classic INT8 encoder/decoder/joiner have been renamed
locally to `encoder.onnx`, `decoder.onnx`, `joiner.onnx`; tokens remain `tokens.txt`.
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
