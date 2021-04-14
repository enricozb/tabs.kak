# tabs.kak

View open buffers in status line, like tabs, and navigate between them quickly.

Inspired by [Delapouite's `kakoune-buffers`][1]. This plugin is **incompatible with [powerline.kak](https://github.com/andreyorst/powerline.kak)**, see [here](https://github.com/enricozb/tabs.kak/issues/1#issuecomment-737410152) for why.

[![asciicast](https://asciinema.org/a/JuHNPO8i7nkOmCBQOCfZ8UEOf.svg)][2]

## Installation & Configuration
The suggested configuration is with [plug.kak][3]:
```
plug "enricozb/tabs.kak" %{
  set-option global modelinefmt_tabs '%val{cursor_line}:%val{cursor_char_column} {{context_info}} {{mode_info}}'
  map global normal ^ q
  map global normal <a-^> Q
  map global normal q b
  map global normal Q B
  map global normal <a-q> <a-b>
  map global normal <a-Q> <a-B>

  map global normal b ': enter-user-mode tabs<ret>' -docstring 'tabs'
  map global normal B ': enter-user-mode -lock tabs<ret>' -docstring 'tabs (lock)'
}
```
This effectively remaps the functionality of the `b` key to `q`, and frees up `b` for
buffer manipulation. The `modelinefmt_tabs` is the `modelinefmt` that comes before the
tabs in the statusline. It slightly modifies the default to remove the buffer name and
to decrease the overall size of the status line.

### Overflow
There are two behaviors when there are more tabs visible than can be displayed:
- `shrink`: All open tabs are displayed but they occupy less space. For example,
  if we have 10 tabs displayed in a narrow window, all named `readme.md` the tab bar would
  look like:
  ```
  |me.md|me.md|me.md|me.md|me.md|me.md|me.md|me.md|me.md|me.md|me.md|
  ```
  Notice how the tail of the files are shown and the tabs are much narrower.

- `scroll`: As many open tabs are shown as possible, without shrinking them, but instead
  scrolling through the tabs as they are navigated. For example, in the scenario above
  the tab bar would look like:
  ```
  … readme.md | readme.md | readme.md | readme.md …
  ```
  Notice the `…` surrounding the tab bar, indicating that there are more tabs than are
  being displayed.
  **Note: This mode requires the [luar](https://github.com/gustavo-hms/luar) plugin**

To select one of the two modes, use
```
set-option global tabs_overlow "scroll"
```
The default is `shrink`.

### Reserving Space for Tabs
Since it's common to have many tabs open at once, `tabs.kak` by default only takes up
80% of the width of the terminal in the status line, and the rest is reserved for
anything in `modelinefmt_tabs`. In order to change this percentage, to 65% for example,
in order to allow for more space for `modelinefmt_tabs`, set the following option:
```
set-option global modeline_tabs_percentage 65
```

### Custom Separator
It's also recommended to override the default tab separator `|` with the full height
vertical bar `│`. **Please be aware that there have been reports of the full height
vertical bar not rendering properly on the status-line macOS**. This can be done with
```
set-option global tab_separator `│`
```
inside of the config section of `plug enricozb/tabs.kak %{ ... }` above.


## Usage
The current keybindings for navigation are (after entering `tabs` user mode):

- **Moving between buffers**
  - **a** alternate with the previously focused buffer
  - **h** move to the buffer on the left
  - **l** move to the buffer on the right
  - **f** find a buffer by name
- **Arranging buffers**
  - **H** drag this buffer to the left
  - **L** drag this buffer to the right
- **Common buffers**
  - **c** edit `kakrc`
  - **s** navigate to the scratch buffer
  - **u** navigate to the debug buffer
- **Modification**
  - **r** rename this buffer
- **Deleting buffers**
  - **d** delete current buffer
  - **D** delete all saved buffers
  - **o** delete all saved buffers except the current one
  - **O** delete all buffers except the current one

## Screenshots
![tabs.kak screenshot][4]

## To Do
See [todo.md][5] for bugs and upcoming features.

[1]: https://github.com/Delapouite/kakoune-buffers/
[2]: https://asciinema.org/a/JuHNPO8i7nkOmCBQOCfZ8UEOf
[3]: https://github.com/robertmeta/plug.kak
[4]: screenshot1.png
[5]: todo.md
