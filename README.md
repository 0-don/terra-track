# TerraTrack: World Port Scanner Database.

<!-- sudo make uninstall && sudo ./configure && sudo make && sudo make install && sudo chmod u+s /usr/local/bin/nmap  -->

<!-- ![TerraTrack Logo](https://path-to-your-logo-if-you-have-one.png) -->

Dive deep into the vast ocean of the internet's open ports with TerraTrack. Our comprehensive global port scanner database offers an exhaustive breakdown of open ports, the services they run, operating system details, and so much more. Gain unparalleled insights to bolster your threat intelligence, research, and network monitoring capabilities.

## Features:

- **Detailed Port Information**: Go beyond simply identifying open ports; understand the intricate details of the services operating on them.
- **Service Mapping**: Decode the specific services running on each identified port.
- **Operating System Identification**: Uncover the OS versions associated with these open ports, giving you a holistic view of potential vulnerabilities.
- **Geographical Distribution**: Geotag the global distribution of specific ports, services, and OS versions.
- **Time-Stamped Historical Data**: Track the chronological appearances of specific ports and services, allowing for trend analysis and historical assessments.
- **Regular Updates**: Stay updated with our continuously refreshed database that mirrors the ever-evolving landscape of the internet's open ports.
- **Exhaustive Scan Data**: For each scan, we ensure you have a comprehensive breakdown, leaving no detail overlooked.

## TODO

- parse both geolocations if one is not found

## Installation:

```bash
git clone https://github.com/don-cryptus/terra-track.git
cd terra-track
```

## Usage:

```
cargo run
```

## Contributing:

Join us in refining TerraTrack! Be it bug reports, feature suggestions, or direct code contributions, we highly value and appreciate the community's involvement.

1. Fork the repository.
2. Create your feature branch: `git checkout -b my-new-feature`
3. Commit your enhancements: `git commit -am 'Add some feature'`
4. Push to the branch: `git push origin my-new-feature`
5. Submit a pull request.

For a detailed walkthrough on contributing, please refer to our [CONTRIBUTING.md](https://github.com/don-cryptus/terra-track/CONTRIBUTING.md).
