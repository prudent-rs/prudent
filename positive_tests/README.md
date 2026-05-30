# Positive tests

They are also loaded by [../README.md](../README.md).

To re-generate symlinks under enforce/src/bin, lint_apply/src/bin and lint_all/src/bin, use
```bash
cd enforce/src; rm -rf bin; mkdir bin; ln -s -r -t bin ../../shared/src/bin/*
```
