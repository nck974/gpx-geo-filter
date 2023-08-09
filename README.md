# GPX Geo filter

This project aims to provide a tool to quickly filter a big amount of gpx tracks to the tracks that are within the provided area.

## Table of Contents

- [Usage](#usage)
- [Features](#features)
- [Contributing](#contributing)
- [Development](#development)

## Usage

1. Place all your `.gpx` files in one folder.
1. Execute the following command, where the first latitude and longitude are the point most south-west. The distance is the maximum distance a point of a file can be from the provided area to be considered:

```powershell
cargo run -- `
    --first-lat 49.454470 `
    --first-lon 10.954986 `
    --second-lat 49.506443 `
    --second-lon 11.030173 `
    --folder samples `
    --threads 8 `
    --distance 300.0 `
```

## Features

- [x] Read all gpx tracks in a folder.
- [x] Make a pre-filtering of the tracks if the first point found is it at a distance longer than `x` (default `300` km) to the closest edge of the provided area.
- [x] Analyze the resulting files excluding the ones that do not have any point in the given area.

## Contributing

1. Submit a pull request.

## Issues

Open an issue in github.

## Development

### Rust

1. Just compile the project `cargo run`.
