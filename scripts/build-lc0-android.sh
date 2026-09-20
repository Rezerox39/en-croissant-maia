#!/usr/bin/env bash
# Builds Leela Chess Zero (v0.30.0) for Android aarch64 using the pure-Eigen
# CPU backend, and packages the binary as src-tauri/resources/lc0.gz so it is
# embedded into the APK next to the Maia 1700 neural network weights.
set -euo pipefail

LC0_VERSION="${LC0_VERSION:-v0.30.0}"
API_LEVEL="${ANDROID_API_LEVEL:-24}"
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT_GZ="$PROJECT_ROOT/src-tauri/resources/lc0.gz"

if [[ -n "${ANDROID_NDK_HOME:-}" ]]; then
  NDK="$ANDROID_NDK_HOME"
elif [[ -n "${ANDROID_HOME:-}" && -d "${ANDROID_HOME}/ndk" ]]; then
  NDK="$(ls -d "${ANDROID_HOME}"/ndk/*/ | sort | tail -1)"
else
  echo "Android NDK not found. Set ANDROID_NDK_HOME." >&2
  exit 1
fi

HOST_TAG="linux-x86_64"
if [[ "$(uname -s)" == "Darwin" ]]; then
  HOST_TAG="darwin-x86_64"
fi
TOOLCHAIN="$NDK/toolchains/llvm/prebuilt/$HOST_TAG"
TARGET_PREFIX="$TOOLCHAIN/bin/aarch64-linux-android${API_LEVEL}"

if [[ ! -x "${TARGET_PREFIX}-clang" ]]; then
  echo "NDK clang not found at ${TARGET_PREFIX}-clang" >&2
  exit 1
fi

WORK="$(mktemp -d)"
trap 'rm -rf "$WORK"' EXIT

echo "Downloading lc0 $LC0_VERSION..."
curl -sSL -o "$WORK/lc0.tar.gz" \
  "https://github.com/LeelaChessZero/lc0/archive/refs/tags/$LC0_VERSION.tar.gz"
tar -xzf "$WORK/lc0.tar.gz" -C "$WORK"
SRC="$WORK/lc0-$LC0_VERSION"

cat > "$WORK/cross-android.ini" <<INI
[host_machine]
system = 'android'
cpu_family = 'aarch64'
cpu = 'armv8-a'
endian = 'little'

[binaries]
c = '${TARGET_PREFIX}-clang'
cpp = '${TARGET_PREFIX}-clang++'
ar = '${TOOLCHAIN}/bin/llvm-ar'
strip = '${TOOLCHAIN}/bin/llvm-strip'
ld = '${TOOLCHAIN}/bin/ld.lld'
INI

echo "Configuring lc0 with meson..."
cd "$SRC"
meson setup build \
  --cross-file "$WORK/cross-android.ini" \
  --buildtype release \
  -Db_lto=false \
  -Dispc=false \
  -Dispc_native_only=false \
  -Dopenblas=false \
  -Dmkl=false \
  -Ddnnl=false \
  -Daccelerate=false \
  -Donednn=false \
  -Dopencl=false \
  -Dcudnn=false \
  -Dplain_cuda=false \
  -Ddx=false \
  -Dtensorflow=false \
  -Dpopcnt=false \
  -Df16c=false \
  -Dpext=false \
  -Dneon=false

echo "Compiling lc0..."
ninja -C "$SRC/build" -j"$(nproc)"

echo "Packaging lc0 -> $OUT_GZ"
gzip -9 -c "$SRC/build/lc0" > "$OUT_GZ"
ls -lh "$OUT_GZ"
