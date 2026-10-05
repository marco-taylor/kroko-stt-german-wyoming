# Licensing and redistribution review

Reviewed 2026-10-05. This document separates original source code, dependencies,
model weights and compiled artifacts. It is not a legal clearance for publication.

## Original project source

The original Rust server, probe, scripts and documentation use Apache-2.0.
The review found no imported implementation in the proposed public source set
requiring a different source license. Protocol specifications and upstream API
examples were consulted; upstream repositories and research snapshots remain
outside the public file set. Preserve LICENSE and NOTICE when distributing source.

Apache-2.0 for our own work does not relicense dependencies or model weights.
The locked Rust dependencies' declared licenses are listed in
[docs/dependency-licenses.md](dependency-licenses.md). Their original license
texts/copyright notices are available through the exact versioned crate sources.
A future binary distribution must retain the applicable third-party notices and
satisfy source availability obligations where required (where applicable).

## sherpa-onnx and native libraries

[sherpa-onnx v1.13.8](https://github.com/k2-fsa/sherpa-onnx/tree/v1.13.8)
and its Rust wrappers declare Apache-2.0. This applies to that code, not all
bundled native libraries. See its [LICENSE](https://github.com/k2-fsa/sherpa-onnx/blob/v1.13.8/LICENSE)
and [CMake dependencies](https://github.com/k2-fsa/sherpa-onnx/tree/v1.13.8/cmake).
[ONNX Runtime](https://github.com/microsoft/onnxruntime/blob/v1.28.2/LICENSE)
uses MIT and has additional third-party notices.

The pinned static archive includes `libespeak-ng.a`, `libpiper_phonemize.a`,
`libucd.a`, Kaldi/OpenFst, kissfft, SentencePiece and ONNX Runtime libraries.
The sherpa Rust sys crate's build script explicitly links the supplied static
libraries, including the TTS-related libraries, even though this application
only uses ASR. [eSpeak NG COPYING](https://github.com/espeak-ng/espeak-ng/blob/master/COPYING)
contains GPLv3. Do not assume unused functionality removes GPL obligations:
we have not established which copyrighted sections survive linking or audited
all native library notices. Base-image OS libraries also retain their licenses.

**Binary/image redistribution is blocked pending a complete native dependency
and linked-artifact license review, inclusion of required notices and any
corresponding-source provision.** The supplied Dockerfile is a local build/test
recipe, not a claim that its binaries can be distributed solely under Apache-2.0.
No binary or image is published by this project preparation.
[Apache's GPL compatibility guidance](https://www.apache.org/licenses/GPL-compatibility.html)
explains Apache-2.0/GPLv3 compatibility; compatibility does not waive GPL duties.

## Kroko / Banafo models

No weights, vocabularies or archives are included. Hashes and model identifiers
identify independently obtained external files and do not grant model rights.

* **classic:** the historical sherpa ONNX export's exact weight license remains
  unclear. Do not describe it as freely redistributable or place it in public
  Git repositories, Docker images or mirrors.
* **community-64 and community-128:** the pinned [Banafo model card](https://huggingface.co/Banafo/Kroko-ASR/blob/d45212aeb212dd66083dd22710c9954f40ff8cc1/README.md)
  identifies Community models as CC-BY-SA. However the pinned metadata says
  `license: other` / `license_name: test`, its license file is empty, and no
  versioned full license grant naming both exact files was established.
  [Banafo documentation](https://docs.kroko.ai/on-premise/) supports the category
  attribution. This is clearer than classic, but not complete distribution clearance.

CC-BY-SA generally requires attribution, identification of changes and Share-Alike
for distributed adaptations. Do not choose a CC version or infer specific
commercial/redistribution rights without establishing the applicable grant.
Before any model redistribution, obtain the exact license, required attribution,
version and file coverage from Banafo. Local compatibility testing does not resolve
those questions. The same caution applies to extracting model components.

## Credits

Kroko/Banafo supplies models; k2-fsa supplies sherpa-onnx; Wyoming/Rhasspy and
Home Assistant supply the protocol/ecosystem. No partnership or endorsement is
claimed. Wyoming's official Python implementation is MIT; it is a protocol
reference, not a runtime dependency or copied implementation in this Rust server.

## Rust notice obligations

MIT/BSD/ISC dependencies require retention of their copyright/license notices;
Apache-2.0 requires its license and applicable NOTICE text, with changes marked
where relevant. Unicode-3.0 notices apply in addition to unicode-ident's chosen
MIT/Apache branch. ring declares Apache-2.0 AND ISC; both matter. r-efi offers
MIT OR Apache-2.0 OR LGPL-2.1-or-later, so the permissive branch can be chosen.
No mandatory GPL-only Rust dependency was found in the locked package inventory.
Build-only and other-target crates are listed for transparency, not asserted to
be linked into the final linux/amd64 server. Consult exact crate license files,
not only SPDX metadata, when assembling distribution notices.
