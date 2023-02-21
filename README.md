## Notice
I'm using this in another project. You can just use `soundstretch`.

## Description
Elongate uses `soundstretch` to change the length of an audio file without changing the pitch.

## Usage
`elongate --tempo=-20 --input-file="input.wav" --output-file="output.wav"`

You can put these parameteres into a file named `elongate.toml` and it will use those as default.


## Requirements
- `ffmpeg`
- `soundstretch`. 

## Building 
Building for Ubuntu 18.04:

```
docker build --tag rust-docker .
docker run -v ${PWD}:/usr/src/myapp rust-docker cargo build --release
```
