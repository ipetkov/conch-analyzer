import * as wasm from "conch-analyzer";

let nixos_installer_script = `#!/bin/sh

# This script installs the Nix package manager on your system by
# downloading a binary distribution and running its installer script
# (which in turn creates and populates /nix).

{ # Prevent execution if this script was only partially downloaded
oops() {
    echo "$0:" "$@" >&2
    exit 1
}

umask 0022

tmpDir="$(mktemp -d -t nix-binary-tarball-unpack.XXXXXXXXXX || \
          oops "Can't create temporary directory for downloading the Nix binary tarball")"
cleanup() {
    rm -rf "$tmpDir"
}
trap cleanup EXIT INT QUIT TERM

require_util() {
    command -v "$1" > /dev/null 2>&1 ||
        oops "you do not have '$1' installed, which I need to $2"
}

case "$(uname -s).$(uname -m)" in
    Linux.x86_64) system=x86_64-linux; hash=5260ea5bb30bf3a7d023869644f28fae8b510b91e50f031cf72085f229dc769f;;
    Linux.i?86) system=i686-linux; hash=16cda1322ea63effa6f8ef018f6240240e3d8478011ce060ffba64356f758fe4;;
    Linux.aarch64) system=aarch64-linux; hash=a3be84110647d3aeadef0f8b398c59f71aaf44250ab7ec99464e9781d77b33bb;;
    Darwin.x86_64) system=x86_64-darwin; hash=bb99ed5a18383133ec5c63677fed5cc199dbbc1ef98d0ac312afe3a27d6380ad;;
    Darwin.arm64) system=aarch64-darwin; hash=5078b0a1685682b7d8c44d91bd34ee6979959505641662c37c225a0b7ed7eab3;;
    *) oops "sorry, there is no binary distribution of Nix for your platform";;
esac

url="https://releases.nixos.org/nix/nix-2.3.14/nix-2.3.14-$system.tar.xz"

tarball="$tmpDir/$(basename "$tmpDir/nix-2.3.14-$system.tar.xz")"

require_util curl "download the binary tarball"
require_util tar "unpack the binary tarball"
if [ "$(uname -s)" != "Darwin" ]; then
    require_util xz "unpack the binary tarball"
fi

echo "downloading Nix 2.3.14 binary tarball for $system from '$url' to '$tmpDir'..."
curl -L "$url" -o "$tarball" || oops "failed to download '$url'"

if command -v sha256sum > /dev/null 2>&1; then
    hash2="$(sha256sum -b "$tarball" | cut -c1-64)"
elif command -v shasum > /dev/null 2>&1; then
    hash2="$(shasum -a 256 -b "$tarball" | cut -c1-64)"
elif command -v openssl > /dev/null 2>&1; then
    hash2="$(openssl dgst -r -sha256 "$tarball" | cut -c1-64)"
else
    oops "cannot verify the SHA-256 hash of '$url'; you need one of 'shasum', 'sha256sum', or 'openssl'"
fi

if [ "$hash" != "$hash2" ]; then
    oops "SHA-256 hash mismatch in '$url'; expected $hash, got $hash2"
fi

unpack=$tmpDir/unpack
mkdir -p "$unpack"
tar -xJf "$tarball" -C "$unpack" || oops "failed to unpack '$url'"

script=$(echo "$unpack"/*/install)

[ -e "$script" ] || oops "installation script is missing from the binary tarball!"
"$script" "$@"

} # End of wrapping
`;

function makePre(t) {
  let p = document.createElement('pre');
  p.appendChild(document.createTextNode(t));

  return p
}

function analyzeScript(contents) {
  let td = document.createElement('td');
  td.appendChild(makePre(wasm.cmd_histogram(contents)))

  let tr = document.createElement('tr');
  tr.appendChild(td)

  let tbody = document.getElementById('results');
  tbody.innerHTML = ''
  tbody.appendChild(tr)
}

const scriptInput = document.getElementById('script-input');
scriptInput.innerHTML = nixos_installer_script;

const analyzeButton = document.getElementById('analyze')
analyzeButton.onclick = function() {
  analyzeScript(scriptInput.value)
}

analyzeButton.click()
