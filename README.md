# minigit

A simple CLI tool to save 20 seconds and hand strain.

It consolidates the following commands into one:

```bash
git add .
git commit -m "<message>"
git push
```

To use it, clone the repo, build a release package locally `cargo build --release`, then add the release build directory to your `PATH`. Then run:

```bash
minigit "<message>"
```
