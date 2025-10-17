# testeq-cli

Command-line tool for interacting with test equipment

## Command format

    testeq <uri> <command> [<arg0> ...]

* `uri`
  * For most up-to-date support, see testeq-rs repo
  * VXI-11 devices: `vxi11://<host>[:<port>]`
  * SCPI over TCP: `tcp://<host>:<port>`
  * Serial: `serial:<port>[?baud=<baud>]`

## Supported equipment

To see specific supported models, see [testeq-rs](https://github.com/farlepet/testeq-rs/tree/main)

* Power supplies
  * `status` General status
  * `set_voltage` Get/set voltage set-point
  * `set_current` Get/set current set-point
  * `read_voltage` Get current voltage
  * `read_current` Get current current
  * `read_power` Get current power
