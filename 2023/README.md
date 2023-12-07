# Advent of Code 2023

## Prerequisites

* [Docker CE](https://docs.docker.com/install/)
* [Docker Compose](https://docs.docker.com/compose/install/)

## Build

```sh
docker build -t advent-of-code .
```

## Run

```sh
docker run -it --rm --name AoC advent-of-code cargo run -- ./src/day_1/sample.txt
```
