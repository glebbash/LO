## How to extract info from C header into JSON

```sh
# clone c2ffi repo
git clone https://github.com/rpav/c2ffi.git

cd c2ffi

# add entrypoint to dockerfile
echo "ENTRYPOINT [\"/c2ffi/build/bin/c2ffi\"]" >> Docker/Test-Build-Archlinux.docker

# build and tag the image
docker build -f Docker/Test-Build-Archlinux.docker . -t c2ffi

# extract info (replace lib.h and lib.json with your local input and output file names)
docker run -v $(pwd):/data c2ffi /data/lib.h > lib.json
```
