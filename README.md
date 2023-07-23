# uSIEM Utils
![workflow](https://github.com/u-siem/usiem-utils/actions/workflows/rust.yml/badge.svg)

## Enrichers

* BasicIPEnricher: Enrich all IP fields. Checks if the IP is in the block list, adds mac and hostname information to the IP.
* CloudProviderEnricher: Adds cloud provider information like Google, Azure or AWS to each IP field
* CloudServiceEnricher: Adds cloud service information like O365 to each IP field
* GeoIpEnricher: Adds geo ip information to each IP field

## Tasks

* CloudProvider: Update cloud provider dataset with AWS and Azure
* CloudService: Update cloud service dataset with O365 IPs
* GeoIp: Update geo ip dataset with maxmind. Needs `MAXMIND_API` secret in the Secrets dataset.

## Slow GeoIP
Enable the SlowGeoIP datasets using the feature `slow_geoip`.

## Async Runtime
This crate uses Tokio and reqwest.