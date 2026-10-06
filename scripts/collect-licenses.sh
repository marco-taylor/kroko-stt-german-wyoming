#!/bin/sh
# Retain original upstream license/notice texts, including build dependencies.
set -eu
destination=/opt/distribution-licenses
mkdir -p "$destination"
for root in /opt/sherpa-source /opt/sherpa-build/_deps /usr/local/cargo/registry/src; do
    find "$root" -type f \( -iname '*license*' -o -iname '*copying*' -o -iname '*notice*' -o -name COPYRIGHT \) | while IFS= read -r source; do
        case "$source" in
            *.rs|*.cc|*.cpp|*.h|*.cmake|*.py|*.sh|*.json|*.html) continue ;;
        esac
        relative=${source#/}
        mkdir -p "$destination/$(dirname "$relative")"
        cp "$source" "$destination/$relative"
    done
done
# These bundled helper licenses are embedded in headers, not separate files.
mkdir -p "$destination/simple-sentencepiece-0.7"
for header in darts threadpool; do
    awk '/\/\*/ { inside=1 } inside { print } /\*\// && inside { exit }' \
        "/opt/sherpa-build/_deps/simple-sentencepiece-src/ssentencepiece/csrc/$header.h" \
        > "$destination/simple-sentencepiece-0.7/$header-license.txt"
done
mkdir -p "$destination/onnxruntime-1.28.2"
curl --fail --location --retry 3 \
    https://raw.githubusercontent.com/microsoft/onnxruntime/v1.28.2/ThirdPartyNotices.txt \
    -o "$destination/onnxruntime-1.28.2/ThirdPartyNotices.txt"
echo "0e07b95f3a8d6230037707c5c4a2b554d12c4cb67369669ac255635528ffcee2  $destination/onnxruntime-1.28.2/ThirdPartyNotices.txt" | sha256sum --check
mkdir -p "$destination/rust-1.90.0"
for rust_docs in /usr/local/rustup/toolchains/*/share/doc/rust; do
    cp -R "$rust_docs/licenses" "$destination/rust-1.90.0/"
    cp "$rust_docs/COPYRIGHT-library.html" "$destination/rust-1.90.0/"
done
curl --fail --location --retry 3 https://raw.githubusercontent.com/rust-lang/rust/1.90.0/COPYRIGHT \
    -o "$destination/rust-1.90.0/COPYRIGHT"
echo "172020dbfd5b53a226dfde77616190a48dcff519b0bc0e6deb91a8450782c4af  $destination/rust-1.90.0/COPYRIGHT" | sha256sum --check

# Corresponding sources accompany the copyleft-covered runtime components.
# These are dependency source archives, never ASR model archives or weights.
sources="$destination/corresponding-source"
mkdir -p "$sources"
debian_pool=https://deb.debian.org/debian/pool/main/g/glibc
for archive in glibc_2.36-9+deb12u14.dsc glibc_2.36.orig.tar.xz glibc_2.36-9+deb12u14.debian.tar.xz; do
    curl --fail --location --retry 3 "$debian_pool/$archive" -o "$sources/$archive"
done
echo "cfe1f0b8dc1fa211ce5a45b3725cc38b29f88667f1140ebdca6de35cf9c6f1fd  $sources/glibc_2.36-9+deb12u14.dsc" | sha256sum --check
echo "a543c02070d46ccaf866957efd13f10c924daa74c86a90a0254db09a92a708ee  $sources/glibc_2.36.orig.tar.xz" | sha256sum --check
echo "cf4ac9cd98185452cae3ef34e2e4ee12753e3d93fd0c62c61396d4a47eec902f  $sources/glibc_2.36-9+deb12u14.debian.tar.xz" | sha256sum --check
cp /opt/sherpa-build/_deps/eigen-subbuild/eigen-populate-prefix/src/eigen-5.0.1.tar.gz "$sources/"
echo "e9c326dc8c05cd1e044c71f30f1b2e34a6161a3b6ecf445d56b53ff1669e3dec  $sources/eigen-5.0.1.tar.gz" | sha256sum --check
curl --fail --location --retry 3 \
    https://github.com/eigen-mirror/eigen/archive/1d8b82b0740839c0de7f1242a3585e3390ff5f33/eigen-1d8b82b0740839c0de7f1242a3585e3390ff5f33.zip \
    -o "$sources/eigen-1d8b82b0740839c0de7f1242a3585e3390ff5f33.zip"
echo "6a60d76351f97132669daeeb721d6bf14b008101883ad2d687a3201c5c461eb0  $sources/eigen-1d8b82b0740839c0de7f1242a3585e3390ff5f33.zip" | sha256sum --check
