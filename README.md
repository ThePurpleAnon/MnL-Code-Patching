# Mario & Luigi Code Patches
This repository contains various code patches for the Mario & Luigi games.
They require the [M&L Modding armips fork](https://github.com/MnL-Modding/armips) in order to build, and
use the [`just`](https://github.com/casey/just) command runner.

## *Bowser's Inside Story*
### Requirements
* Nightly Rust toolchain, with the `rust-src` component installed
* `arm-none-eabi-gcc`
* [`mnltools.py`](https://github.com/MnL-Modding/mnltools.py) (in `PATH`)

### Building
First ensure you have the game unpacked in the required format:
```bash
mnl-nds-unpack -d bis-data bis.nds
```
Now you can build it. Remove the flags you don't need:
```bash
just build-nds bis release -definelabel F_ANTI_PIRACY_PATCH 1 -definelabel F_MIXED_SHOP 1 -definelabel F_CUSTOM_ITEM_TYPES 1
```
The game will be built as `bis.nds`.

### Flags
* **`F_ANTI_PIRACY_PATCH`:** Bypasses the anti-piracy protections of the game.
* **`F_MIXED_SHOP`:** Allows consumable item shops to sell any type of item, not just consumables.
* **`F_CUSTOM_ITEM_TYPES`:** Adds custom item types. Currently these are:
  * `0x5xxx` <ins>[Unfinished!]</ins>: Variable items. These items use scripting variables as their backing storage.
