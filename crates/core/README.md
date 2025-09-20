# webview-bundle

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
  <td>1 bytes</td>
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
- **Version (1 bytes)**
  - Version field for this webview bundle format.
  - Available versions:
    - version1: `0x01`
- **Index Size (4 bytes)**
  - 4 bytes unsigned big endian value (`u32`)
  - Indicates the size of the Index field to be read next.
- **Header checksum (4 bytes)**
  - Checksum verifies that the full header data has been decoded correctly.
  - The checksum is the result of [xxHash-32 algorithm](https://github.com/Cyan4973/xxHash/blob/release/doc/xxhash_spec.md) digesting the header data as input.

### Index

- **Index**
  - This field has dynamic bytes size which is determined by the value of the Index Size field, and value is a big endian format.
  - Format of index should be `HashMap` formatted with binary encoded:
    - Key is a path for this file.
    - Value contains offset, length for reading contents from the data field.
    - Value can contain http headers which will be included in the response.
- **Index checksum (4 bytes)**
  - Checksum verifies that the full index data has been decoded correctly.
  - The checksum is the result of [xxHash-32 algorithm](https://github.com/Cyan4973/xxHash/blob/release/doc/xxhash_spec.md) digesting the encoded index data as input.

### Data

- **Data**
  - This field has dynamic bytes size which can be determined each file offset and length from Index.
  - The content of data is compressed with [lz4 block format](https://github.com/lz4/lz4/blob/dev/doc/lz4_Block_format.md).
  - The last 4 bytes are the checksum which is the result of [xxHash-32 algorithm](https://github.com/Cyan4973/xxHash/blob/release/doc/xxhash_spec.md) digesting the compressed data as input.
