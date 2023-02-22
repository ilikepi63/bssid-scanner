# BSSID Scanner

## Getting Started

Build script -> build-and-run.sh

Simply:

```
./build-and-run.sh
```

## Some General Notes

- Use of network-manager as the cli tool for getting the wifi information(This is documented as the output should be)
  - The host environment would need to have that installed, although the binary does not panic if it isn't, it will just skip the iteration(assuming this would be preferred behaviour)
- This is built specifically for a linux(ubuntu) based host. The reason for this assumption is to consider this project a PoC which the windows/mac implementations can be built into(shown in comments) and that as discussed Aruba deploys on a custom device(although is planning on deploying on user devices as well)
- Tokio as the runtime
  - Mostly because we make a request on an interval, which Tokio is really good at.



