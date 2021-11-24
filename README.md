# lg
[![Build](https://github.com/Ewpratten/lg/actions/workflows/build.yml/badge.svg)](https://github.com/Ewpratten/lg/actions/workflows/build.yml)
[![Clippy](https://github.com/Ewpratten/lg/actions/workflows/clippy.yml/badge.svg)](https://github.com/Ewpratten/lg/actions/workflows/clippy.yml)
[![Audit](https://github.com/Ewpratten/lg/actions/workflows/audit.yml/badge.svg)](https://github.com/Ewpratten/lg/actions/workflows/audit.yml)

`lg` is my custom [Looking Glass server](https://en.wikipedia.org/wiki/Looking_Glass_server) software for use in ZZANet. The whole application is lightweight, self-contained, and easy to use even outside of ZZANet.

## Running in Docker

`lg` can be run in a Docker container with one simple command:

```sh
docker run --rm \
    -v $(pwd)/configs/local.json:/config/local.json:ro \
    -v $(pwd)/configs/global.json:/config/global.json:ro \
    --sysctl net.ipv6.conf.all.disable_ipv6=0 \
    -p 80:80 \
    ewpratten/lg
```

**Note:** by default, Docker will not work with the IPv6 stack. It is your responsibility to [enable it](https://docs.docker.com/config/daemon/ipv6/).
