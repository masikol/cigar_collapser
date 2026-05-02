# cigar_collapser

1. [Description](#description).

2. [Installation](#installation).

3. [Usage and examples](#usage-and-examples).

## Description

A program that collapses CIGAR strings from SAM/BAM files making CIGAR short and easy to see which part of the read is actually aligned and which is clipped (see simple [examples](#usage-and-examples) below).

It is useful when inspecting long reads (e.g. Oxford nanopore, PacBio), whose CIGAR strings are often very long too.

## Installation

### Download a pre-built binary from GitHub

--snip--

### Install using Cargo

--snip--

### Build from source

--snip--

## Usage and examples

No options, just pass CIGAR strings as CLI arguments or send them as stdin stream one per line.

### Pass CIGAR strings as command line arguments

Command:

```bash
./cigar_collapser 123S10M10I5D333H 123S10I5D20X30=333H
```

Output:

```
          123-S|             20|          333-H
          123-S|             60|          333-H
```

For example, the first output line effectively means that:

1. First 123 bases of the read are soft-clipped.

2. Then, 20 bases are aligned to the reference.

3. Last 333 bases are hard-clipped.

### Pass CIGAR strings to read from stdin

Command:

```bash
echo '
123S10M10I5D333H
123S10I5D20X30=333H
' | ./cigar_collapser
```

Output:

```
          123-S|             20|          333-H
          123-S|             60|          333-H
```

### Inspect a specific read in a BAM file

Command:

```bash
samtools view nanopore_mapped.bam \
    | cut -f1,6 \
    | grep '3298da8e-7f61-4c46-af7b-4e5ac01ccc99' \
    | cut -f2 \
    | cigar_collapser
```

Output:

```
        3,106-S|          5,035|          355-S
              .|          3,078|        5,418-H
```

### Rust API

There is a single public function in the crate: `collapse_cigar`.

Signature:

```rust
pub fn make_collapsed_cigar(str_arg: &String) -> Result<String, String>
```

Example code:

```rust
use cigar_collapser::collapse_cigar;

fn main() {
    let cigar_str = String::from(
        "123S10I5D20X30=333H"
    );
    let collapsed_str = collapse_cigar(&cigar_str).unwrap();
    println!("{}", collapsed_str);
}
```

Output:

```
          123-S|             60|          333-H
```
