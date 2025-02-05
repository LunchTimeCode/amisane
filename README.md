## Amisane

Am I sane? A simple sanity checker for your CSV files.

### Usage

```bash
# In the directory you want to set it you init.
amisane init
# Change the amisne.toml config file to your liking

# Prepare the setup by scanning your baseline csv files
amisane prepare

# Run or do your actual logic on the files

# Now are you sane? 
amisane

```

### Configuration

```toml
# amisane.toml

# A path is always an entry point for one or many sanity check on a single file
[[paths]]
path = "example/somefile.csv"
# You may want to copy the file to a different location before running your new logic, as a backup.
# This property will copy the file for you into /amisane/files/<file>
copy_files = true
# The tests it should run on the file
tests = ["FileHash"]

# You may add as many paths as you want.
[[paths]]
path = "example/otherfile.csv"
copy_files = true
tests = ["FileHash"]

```

### Sanity Checks

#### FileHash [Implemented] [Unstable]

Creates a hash in the prepeare and check step over the content of the whole file.

- Includes all Bytes in the file, turned into a [String](https://doc.rust-lang.org/std/string/struct.String.html#representation), then this String is hashed and saved into amisane/amisane_read.toml.
- Uses [Sip13](https://docs.rs/siphasher/latest/siphasher/sip128/struct.SipHasher13.html) Implementation, see the [Paper](https://github.com/veorq/SipHash) for more info.
- Excludes the metadata of the file as this is platform specific and varies by time (e.g. creation date, etc.)

#### RowHashes [Planned]

Creates a hash in the prepeare and check step over each row of the csv file.

### Platform Reproducibility

When running amisane on the same file on different platforms, the hash can vary because of the different newline characters that are used on different platforms.
So you should use prepare and check on the same platform.

### Glossary

#### File

- A file is a single csv file that is being checked for sanity.
- Not including the metadata

#### Path

- Relative path to a file that is being checked for sanity.
- Can not be extended with Glob patterns, or Regex.
- Always relative to the the directory where the `amisane` binary is run.

#### Row

- A single row in a csv file.
- A row is defined by a [newline character](https://en.wikipedia.org/wiki/Newline).
- A row therefore can be platform specific when generated on different platforms, e.g. newline on Linux `\n` or on Windows `\r\n`.