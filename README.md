# Cart Studio for slot on the Miyoo Flip

A fork of **[cart-studio-web](https://github.com/slot-cfw/cart-studio-web)**, the label maker
for [slot](https://slot-cfw.fyi) cards: it makes cart labels in the browser and previews them on
the same cart face the frontend draws. The original runs at
[studio.slot-cfw.fyi](https://studio.slot-cfw.fyi) and is made for slot on the Anbernic RG SP.

This fork is made for **[slot-miyoo-flip](https://github.com/Shcoobz/slot-miyoo-flip)**, the
Miyoo Flip port of slot, and reads cards the way that port does. This page covers only what is
different here.

## What this fork changes

- **NES and SNES.** Two new platforms next to GBA, GBC and GB, each with its own shelf and tab
  (`Games/NES`, `Games/SNES`, `Labels/NES`, `Labels/SNES`).
- **Label shapes for the new carts.** A tall, narrow NES label and a wide, flat SNES label,
  sized to the label wells slot itself draws. The colour gradient runs along the label's long
  side, so it works on the tall NES label too.
- **Matching NES and SNES roms.** The CRC leaves out an NES rom's 16-byte iNES header and a
  SNES rom's 512-byte copier header, the way No-Intro hashes them, so headered dumps are still
  found in libretro's database.
- **Logos for NES and SNES** come from libretro's Named_Logos, since the studio's own logo set
  has only Game Boy and GBA.
- **One level of game folders**, as slot-miyoo-flip reads them:
  `Games/GBA/Pokemon Hacks/x.gba`. Folders only group carts on the shelf; labels stay flat in
  `Labels/GBA`.
- **Small print and outlines:** NES and SNES carts print no product code (their headers carry
  none), and only a Game Boy pak offers two outlines.
- **Runs without building.** The compiled WASM in `web/pkg/` is committed, so serving `web/`
  is enough (see below).

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

### Rebuilding the WASM

Only needed after changing the Rust code: `task build` (needs the slot-miyoo-flip checkout next
to this one as `../slot`, Rust and wasm-pack).

## Licence

GPL-3.0-or-later, same as slot. See [LICENSE](LICENSE).

Game names and CRCs come from
[libretro-database](https://github.com/libretro/libretro-database) (CC BY-SA 4.0); logos come
from [ScreenScraper](https://www.screenscraper.fr/) (CC BY-NC-SA 4.0). 

Game Boy, NES and Super Nintendo are trademarks of Nintendo. Not affiliated with Nintendo, Miyoo,
Anbernic or the original slot project.
