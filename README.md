# Timeline Geotagger

An easy-to-use CLI tool to geotag your photos using Google Maps Timeline data. It parses your exported `Timeline.json` and writes GPS EXIF tags to matching photos based on capture time.

## Install

```bash
cargo install --path .
```

Alternatively, build and run locally:

```bash
cargo run --release
```

### Download (prebuilt binaries)

Grab the latest release from `https://github.com/imrlyunoriginal/timeline_geotagger/releases`.

Artifacts:
- `timeline_geotagger-windows-x86_64.exe` (Windows)
- `timeline_geotagger-macos-x86_64` (macOS, Intel)
- `timeline_geotagger-linux-x86_64` (Linux)

## Usage

You will be prompted for:
- Path to your `Timeline.json` (from Google Takeout)
- Path to the directory containing your photos (`.jpg`, `.jpeg`, `.png`)
- Timezone the photos were taken in (e.g. `Europe/London`)

The tool will locate your position at each photo's timestamp and write GPS EXIF tags.

## Exporting Timeline.json

- Export timeline data by following the instructions at https://support.google.com/maps/answer/6258979.
- Place the exported `Timeline.json` somewhere accessible and select it when prompted.

## Notes

- EXIF writing uses `little_exif`; photos must include `DateTimeOriginal`.
- Files are modified in-place; back up your photos first.
- Supported formats: JPG/JPEG/PNG. Non-image files are skipped.

## License

MIT. See `LICENSE`.

