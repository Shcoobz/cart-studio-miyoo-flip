# slot cart studio

Cart labels for a [slot](https://slot-cfw.fyi) SD card, previewed on the same cart face the
frontend draws. Runs in the browser at [studio.slot-cfw.fyi](https://studio.slot-cfw.fyi).

This copy sits beside the Miyoo Flip fork of slot and reads cards the way that fork does,
including one level of game folders (`Games/GBA/Pokemon Hacks/x.gba`).

## Platforms

One folder per platform under `Games/` and `Labels/`, with slot's own extensions:

| Folder | Platform         | Roms              |
| ------ | ---------------- | ----------------- |
| `GBA`  | Game Boy Advance | `.gba`            |
| `GBC`  | Game Boy Color   | `.gb`, `.gbc`     |
| `SNES` | Super Nintendo   | `.sfc`, `.smc`    |
| `GB`   | Game Boy         | `.gb`, `.gbc`     |
| `NES`  | NES              | `.nes`            |

Games are matched by CRC32 against libretro's No-Intro DATs. An NES rom is hashed without its
16-byte iNES header and a SNES rom without a 512-byte copier header, as No-Intro hashes them.

## Run your own copy

### Prebuilt (no building)

After a build, `web/` already holds everything the page needs, including the compiled WASM in
`web/pkg/` (committed to this repository). Serve the folder and open it in the browser:

```sh
python3 -m http.server 8765 --bind 127.0.0.1 --directory web
```

Then open <http://127.0.0.1:8765> in **Chrome or Edge**.

- Use a local server like the one above, not `file://`: browsers do not load ES modules from
  `file://`.
- Reading and writing your card uses the File System Access API, which Chrome and Edge offer
  only on a secure context. `http://127.0.0.1` and `http://localhost` count as secure;
  another machine's address over plain `http` does not.

### GitHub Pages

The page is static, so publishing `web/` (with `web/pkg/` in it) is enough. Note that the
included `.github/workflows/pages.yml` rebuilds the WASM against upstream
`BrandonKowalski/slot` at the commit in `slot.ref`, not against the Flip fork. To publish this
copy as it is, deploy the committed `web/` folder instead, for example with a workflow that
runs `actions/upload-pages-artifact` with `path: web` followed by `actions/deploy-pages`.

### Rebuilding the WASM

`task build` runs:

```sh
wasm-pack build --release --target web --no-typescript --out-dir web/pkg
```

It needs the slot checkout beside this one (`../slot`), the Rust toolchain from
`rust-toolchain.toml` (1.96 with the `wasm32-unknown-unknown` target) and wasm-pack. Without
installing those locally, run it in Docker from the folder that holds both checkouts:

```sh
docker run --rm -v "$PWD":/w -w /w/slot-cart-studio rust:1-bookworm sh -c '
  cargo install wasm-pack &&
  wasm-pack build --release --target web --no-typescript --out-dir web/pkg &&
  rm -f web/pkg/.gitignore'
```

wasm-pack writes a `.gitignore` containing `*` into `web/pkg/`; `task build` removes it so the
built package can be committed. `task serve` builds and then serves `web/` as above.

## License

GPL-3.0-or-later, same as slot. See [LICENSE](LICENSE).

Game names and CRCs come from
[libretro-database](https://github.com/libretro/libretro-database) (CC BY-SA 4.0); logos come
from [ScreenScraper](https://www.screenscraper.fr/) (CC BY-NC-SA 4.0). 

Game Boy, NES and Super Nintendo are trademarks of Nintendo. Not affiliated with Nintendo.
