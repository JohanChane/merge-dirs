# merge-dirs

The tool for merging two directories.

## Usage

```sh
# append: Append the files from src_dir to dst_dir based on their relative path.
# copy: Copy the files from src_dir to dst_dir based on their relative path.
# delete: Delete the files from dst_dir based on their relative path from src_dir.
mergedir -s <src_dir> -d <dst_dir> -m <append | copy | delete>
```
