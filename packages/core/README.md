# webview-bundle

<img height="56" alt="Webview Bundle" src="https://static.wvb.dev/banner.png" />

Webview Bundle is an "Offline First" distributing system for web applications on webview-based frameworks and platforms.

Instead of fetching your web application code over the network and rendering it in the webview,                                                                                              
it loads web application code packed into a webview bundle file (`.wvb`) from local and renders it in the webview. A remote can also be configured to download webview bundles from the remote, effectively delivering up-to-date application code without requiring a native update.

Webview Bundle supports all the platforms that provide a webview — [Electron](https://www.electronjs.org/), [Tauri](https://tauri.app), Android, iOS, and [Deno Desktop](https://docs.deno.com/runtime/desktop/) (experimental) — and also ships with a bridge for communicating with the webview.

For more details, see the [documentation](https://wvb.dev/docs).

## Webview Bundle Format (.wvb)

<table>
<thead>
<tr>
  <th colspan="4">Header</th>
  <th colspan="2">Index</th>
  <th>Data</th>
</tr>
</thead>
<tbody>
<tr>
  <td>MagicNb</td>
  <td>Version</td>
  <td>Index Size</td>
  <td>Checksum</td>
  <td>Index</td>
  <td>Checksum</td>
  <td>Data</td>
</tr>
<tr>
  <td>8 bytes</td>
  <td>1 byte</td>
  <td>4 bytes</td>
  <td>4 bytes</td>
  <td>Size of index</td>
  <td>4 bytes</td>
  <td>(...)</td>
</tr>
</tbody>
</table>

### Header

- **Magic Number (8 bytes)**
  - Big endian format. Value : `0xf09f8c90` / `0xf09f8e81`
  - Represents "🌐🎁" as utf8 encoding.
- **Version (1 byte)**
  - Version field for this webview bundle format.
  - Available versions:
    - version1: `0x01`
- **Index Size (4 bytes)**
  - 4 bytes unsigned big endian value (`u32`)
  - Indicates the size of the Index field to be read next.
- **Header checksum (4 bytes)**
  - Checksum verifies that the full header data has been decoded correctly.
  - The checksum is the result
    of [xxHash-32 algorithm](https://github.com/Cyan4973/xxHash/blob/release/doc/xxhash_spec.md)
    digesting the header data as input.

### Index

- **Index**
  - This field has dynamic bytes size which is determined by the value of the Index Size field, and
    value is a big endian format.
  - Format of index should be `HashMap` formatted with binary encoded:
    - Key is a path for this file.
    - Value contains offset, length for reading contents from the data field, content type and
      uncompressed content length.
    - Value can contain http headers which will be included in the response.
- **Index checksum (4 bytes)**
  - Checksum verifies that the full index data has been decoded correctly.
  - The checksum is the result
    of [xxHash-32 algorithm](https://github.com/Cyan4973/xxHash/blob/release/doc/xxhash_spec.md)
    digesting the encoded
    index data as input.

### Data

- **Data**
  - This field has dynamic bytes size which can be determined each file offset and length from
    Index.
  - Each file is compressed
    with [lz4 block format](https://github.com/lz4/lz4/blob/dev/doc/lz4_Block_format.md),
    prefixed with its uncompressed size as a 4-byte little endian value.
  - Each compressed file is followed by a 4-byte checksum which is the result
    of [xxHash-32 algorithm](https://github.com/Cyan4973/xxHash/blob/release/doc/xxhash_spec.md)
    digesting the
    size-prefixed compressed data as input.

## Source

A Source is a structure that stores multiple webview bundles locally, composed of a combination of
directories and a
`manifest.json` file.

The directory structure is as follows:

```
{dir}
|- {bundle_name}
|  |- {version}.wvb
|- app
|  |- 1.0.0.wvb
|  |- 1.1.0.wvb
|- manifest.json
```

The `manifest.json` records version information and other metadata for the bundles located in the
directory, as well as the current version of each bundle.

Below is the manifest v1 schema:

```json
{
  "manifestVersion": 1,
  "bundles": {
    "<bundle_name>": {
      "versions": {
        "<version>": {
          "integrity": "(optional)",
          "metadata": { "key": "value (optional)" }
        }
      },
      "currentVersion": "<version> (optional)",
      "previousVersion": "<version> (optional)",
      "stagedVersion": "<version> (optional)"
    },
    "app": {
      "versions": {
        "1.0.0": {}
      },
      "currentVersion": "1.0.0"
    }
  }
}
```

Directories can be separated into two types within the application:

- **`builtin`**: A bundle Source that is shipped together with the application distribution.
  It is used as a fallback when the latest bundles cannot be downloaded from the remote, such as
  during the application's first launch. It is generally treated as **read-only**.
- **`remote`**: Contains the latest bundles downloaded from the remote. If a bundle with
  the same name exists in both `builtin` and `remote`, the **`remote` version takes priority**.

## Remote HTTP Spec

### Get the current update (`GET /update`)

This returns the current update, including a list of bundles and their versions.

The request includes the following headers:

- `wvb-update-protocol-version` : Update protocol version (`1`).
- `wvb-runtime-version` : Supported update runtime version (`1`).
- `wvb-update-channel` : (Optional) Channel of this update.
- `If-None-Match` : (Optional) ETag from the previous response.
- `wvb-expect-signature` : (Optional) Verification key and algorithm, formatted as
  `key_id="key1", alg="ed25519"`.

```http request
GET /update
Host: wvb.example.com
Accept: application/json
Wvb-Update-Protocol-Version: 1
Wvb-Runtime-Version: 1

### Response
HTTP 200 OK
Content-Type: application/json
ETag: "update1"

{
  "id": "a91bb7cb-1d45-4f78-bc48-6521371ed7f2",
  "createdAt": "2026-10-12T00:00:00Z",
  "runtimeVersion": 1,
  "bundles": [{ "name": "bundle1", "version": "1.0.0" }]
}
```

Bundle metadata can include optional `integrity`, `downloadUrl` and string-valued `metadata`.
The optional `wvb-signature` response header contains a signature of the raw update response body,
formatted as `key_id="key1", alg="ed25519", sig="<base64 signature>"`.

#### Exceptions

If no update is available, the server will return a 204 No Content response.
If the update matches `If-None-Match`, the server will return a 304 Not Modified response.

### Download a specific version of the bundle (`GET /bundles/{name}/{version}`)

Get a specific version of the bundle with the specified name and version.
If `downloadUrl` is specified in the update, the client uses that URL instead.

```http request
GET /bundles/{name}/{version}
Host: wvb.example.com

### Response
HTTP 200 OK
Content-Type: application/webview-bundle

(binary data)
```

#### Exceptions

If the bundle version does not exist, the server will return a 404 Not Found response.

```http request
GET /bundles/{name}/{version}
Host: wvb.example.com

### Response
HTTP 404 Not Found
```

## License

MIT License
