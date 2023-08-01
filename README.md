# GPX Geo filter

This project aims to provide a tool to quickly filter a big amount of gpx tracks to the tracks that are within the provided area.

## Table of Contents

- [Usage](#usage)
- [Features](#features)
- [Contributing](#contributing)
- [License](#license)

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

[Specify how others can contribute to your project. Include guidelines for bug reports, feature requests, and code submissions.]

1. Fork the repository.
2. Create a new branch: `git checkout -b my-new-feature`.
3. Make your changes and commit them: `git commit -m 'Add some feature'`.
4. Push to the branch: `git push origin my-new-feature`.
5. Submit a pull request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE). # TODO

## Acknowledgments

[If your project is based on or inspired by others, acknowledge them here.]

[Example: This project was inspired by [Name of the project or person](link-to-source).]

## Issues

Open an issue in github.
