# webview-bundle

## Webview Bundle Format (.wvb)

<table>
<thead>
<tr>
  <th colspan="5">Header</th>
  <th colspan="2">Data</th>
  <th>Footer</th>
</tr>
</thead>
<tbody>
<tr>
  <td>MagicNb</td>
  <td>Version</td>
  <td>F. Descriptors Size</td>
  <td>F. Descriptors</td>
  <td>H. Checksum</td>
  <td>Data</td>
  <td>D. Checksum</td>
  <td>C. Checksum</td>
</tr>
<tr>
  <td>8 bytes</td>
  <td>1 bytes</td>
  <td>4 bytes</td>
  <td>(...)</td>
  <td>4 bytes</td>
  <td>(...)</td>
  <td>4 bytes</td>
  <td>4 bytes</td>
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
- **File Descriptors Size (4 bytes)**
  - 4 bytes unsigned big endian value (`u32`)
  - Indicates the size of the File Descriptors field to be read next. 
- **File Descriptors**
  - This field has dynamic bytes size which is determined by the value of the File Descriptors Size field, and value is big endian format.
  - Format of file descriptors should be `HashMap` formatted:
    - Key is a path for this file.
    - Value contains offset and length.
- **Header checksum (4 bytes)**
  - Header checksum verifies that the full header data has been decoded correctly.
  - The checksum is the result of [xxHash-32 algorithm](https://github.com/Cyan4973/xxHash/blob/release/doc/xxhash_spec.md) digesting the original (decoded) data as input.

### Data

- **Data**
  - This field has dynamic bytes size which can be determined each file offset and length from File Descriptors.
  - The content of data is compressed with [lz4 block format](https://github.com/lz4/lz4/blob/dev/doc/lz4_Block_format.md).
- **Data checksum (4 bytes)**
  - Data checksum verifies that the full data data has been decoded correctly.
  - The checksum is the result of [xxHash-32 algorithm](https://github.com/Cyan4973/xxHash/blob/release/doc/xxhash_spec.md) digesting the original (decoded) data as input.

### Footer

- **Content Checksum (4 bytes)**
  - Content checksum verifies that the full data content has been decoded correctly.
  - The checksum is the result of [xxHash-32 algorithm](https://github.com/Cyan4973/xxHash/blob/release/doc/xxhash_spec.md) digesting the original (decoded) data as input.
