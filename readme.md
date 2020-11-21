# tabs.kak

View open buffers in status line, like tabs, and navigate between them quickly.

![tabs.kak screenshot][1]
![tabs.kak screenshot][2]

Inspired by [Delapouite's `kakoune-buffers`][3].

## Installation & Configurationn
The suggested configuration is with [plug.kak][4]:
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


[1]: screenshot1.png
[2]: screenshot2.png
[3]: https://github.com/Delapouite/kakoune-buffers/
[4]: https://github.com/robertmeta/plug.kak
