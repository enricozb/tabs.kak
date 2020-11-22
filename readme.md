# tabs.kak

View open buffers in status line, like tabs, and navigate between them quickly.

Inspired by [Delapouite's `kakoune-buffers`][1].

[![asciicast](https://asciinema.org/a/6JrXsCORHqIq3ZW1F9BFC7uc6.svg)][2]

## Installation & Configurationn
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

## Screenshots
![tabs.kak screenshot][4]
![tabs.kak screenshot][5]


[1]: https://github.com/Delapouite/kakoune-buffers/
[2]: https://asciinema.org/a/6JrXsCORHqIq3ZW1F9BFC7uc6
[3]: https://github.com/robertmeta/plug.kak
[4]: screenshot1.png
[5]: screenshot2.png
