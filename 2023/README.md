# Advent of Code 2023

## Prerequisites

* [Docker CE](https://docs.docker.com/install/)
* [Docker Compose](https://docs.docker.com/compose/install/)

## Build and run

```sh
./run.sh sample.txt
```

## Build

```sh
docker build -t advent-of-code .
```

## Run

```sh
docker run -it --rm --name AoC advent-of-code cargo run -- sample.txt
```

## Handy tidbit

```sh
docker run --rm -u "$(id -u):$(id -g)" -v $(pwd):/usr/src/advent -w /usr/src/advent advent-of-code cargo new adventofcode
```
