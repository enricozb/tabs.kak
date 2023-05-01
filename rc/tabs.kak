# ────────────── initialization ──────────────
define-command -override tabs-command -params ..1 %{
  evaluate-commands %sh{
    if [ -n "$1" ]; then
      action="--action $1"
    fi

    eval "target/release/kak-tabs \
      --width '$kak_window_width' \
      --focused '$kak_bufname' \
      $action \
      $kak_quoted_buflist"
  }
}

hook -group kak-tabs global WinDisplay .* tabs-command
hook -group kak-tabs global WinResize  .* tabs-command


# ────────────── keys ──────────────
define-command tabs-recommended-keys -docstring "set the recommended kak-tabs bindings" %{
  map global normal q b
  map global normal Q B
  map global normal b ': enter-user-mode tabs<ret>' -docstring 'tabs'
  map global normal B ': enter-user-mode -lock tabs<ret>' -docstring 'tabs (lock)'
}

declare-user-mode tabs

# navigate
map global tabs a "ga" -docstring "↔ alternate"
map global tabs h ": tabs-command prev<ret>" -docstring "← prev"
map global tabs l ": tabs-command next<ret>" -docstring "→ next"
map global tabs s ": edit -scratch *scratch*<ret>" -docstring "*scratch*"
map global tabs u ": edit -debug *debug*<ret>" -docstring "*debug*"

# arrange
map global tabs H ": tabs-command drag-left<ret>" -docstring "← drag left"
map global tabs L ": tabs-command drag-right<ret>" -docstring "→ drag right"

# mutate
map global tabs d ": delete-buffer<ret>" -docstring "delete (current)"
