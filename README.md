# cigar_collapser

A program that collapses CIGAR strings from SAM/BAM files making it short and easy to see which part of the read is actually aligned and which is clipped.

It is useful for long reads (e.g. Oxford nanopore, PacBio), whose CIGAR strings are often very long too.

## Examples

### Read CIGAR strings from command line

Command:

```bash
./cigar_collapser 123S10M10I5D333H 123S10I5D20X30=333H
```

Output:
```
123-S|             20|          333-H
123-S|             60|          333-H
```

### Read CIGAR strings from stdin

Command:

```bash
echo -e '123S10M10I5D333H\n123S10I5D20X30=333H' \
    | ./cigar_collapser
```

Output:
```
123-S|             20|          333-H
123-S|             60|          333-H
```
