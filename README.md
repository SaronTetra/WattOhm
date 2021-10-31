# WattOhm

WattOhm is a bank simulator written in Rust

using Rocket as a web framework
and Diesel for operating on the database

the database used is PostgreSQL

* Rust
  * Rocket
  * Diesel
* PostgreSQL
## 
Wattohm is meant to be used in games (e.g. Monopoly)

as a mean of easier management of the in-game economy

## How to use
Clone this repository
```bash
git clone https://github.com/SaronTetra/WattOhm.git
```
Move to cloned repository
```
cd wattohm
```
start postgres
```bash
docker-compose up -d
```
add needed sql tables (only on first run)
```bash
diesel migration run
```
build Docker Image (only on first run)
```bash
docker build -t wattohm .
```
start WattOhm
```bash
docker run wattohm -d
```

connect to Wattohm in web browser
```
host-ip:8000
```
