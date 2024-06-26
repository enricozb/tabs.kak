# tabs.kak

View open buffers in status line, like tabs, and navigate between them quickly.

## Demo
[![asciicast](https://asciinema.org/a/uAg0yrrrafPQVCowu23ISqRUp.svg)](https://asciinema.org/a/uAg0yrrrafPQVCowu23ISqRUp)

## Keybindings
The recommended key bindings can be set by calling `tabs-recommended-keys`:
```
define-command tabs-recommended-keys -docstring "set the recommended kak-tabs bindings" %{
  map global normal q b
  map global normal Q B
  map global normal b ': enter-user-mode tabs<ret>' -docstring 'tabs'
  map global normal B ': enter-user-mode -lock tabs<ret>' -docstring 'tabs (lock)'
}
```
Within the `tabs` mode, the keybindings are as follows:
```
a: ↔ (alternate)      return to the previously focused buffer
h: ← (previous)       navigate to the buffer on the left
l: → (next)           navigate to the buffer on the right
s:   (*scratch*)      open the *scratch* buffer
u:   (*debug*)        open the *debug* buffer
H: ← (drag left)      swap this buffer to the left
L: → (drag right)     swap this buffer to the right
d:   (delete current) delete the current buffer
```

## Configuration

### Example Configuration
```kak
set-option global tabs_modelinefmt '%val{cursor_line}:%val{cursor_char_column} {{mode_info}} '
set-option global tabs_options --minified
```

### Options

- `tabs_modelinefmt`: when set to a modelinefmt string, it is placed in the space preceding the tabs

- `tabs_options`: a list of switches that affect tabs' appearance.
  - `--minified`
    - this will cause tabs to show as little of their paths as possible while still being unique. For example, if the currently open buffers are
      ```
      projects/1/src/main.rs projects/2/src/main.rs projects/2/Cargo.toml
      ```
      then the rendered tabs will be
      ```
      | 1/src/main.rs | 2/src/main.rs | Cargo.toml |
      ```
      because these are the smallest unique suffixes of each buffer.

## Installation
1. Install the [`rc/tabs.kak`](./rc/tabs.kak) so your kakoune instance auto-loads it.
2. Install the [kak-tabs](https://crates.io/crates/kak-tabs) rust crate.

## TODO
- the order of tabs changes how the disambiguation of files with the same names work. for example:
  - we have a buflist that looks like:
    - `| *debug* | xml.rs | pypi/mirror/mod.rs | cargo/mirror/mod.rs | mod.rs |`
  - swapping the last two buffers yields the following tabs:
    - `| *debug* | xml.rs | mirror/mod.rs | api/mod.rs | mod.rs |`
- tabs should shrink when there's not enough space
- add more buffer deletion options, see: <https://github.com/enricozb/tabs.kak/issues/3>

<hr>

> Originally inspired by [Delapouite's `kakoune-buffers`][1].

> **Warning**
> This plugin is **incompatible** with [powerline.kak][2], see [here][3] for why.


[1]: https://github.com/Delapouite/kakoune-buffers/
[2]: https://github.com/andreyorst/powerline.kak
[3]: https://github.com/enricozb/tabs.kak/issues/1#issuecomment-737410152
