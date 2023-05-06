# ────────────── initialization ──────────────
declare-option str tabs_modelinefmt
declare-option str-list tabs_options

declare-option -hidden bool tabs_modified_buffer false
declare-option -hidden str-list tabs_modified_buffers

hook -group kak-tabs global WinDisplay .* tabs-command
hook -group kak-tabs global WinResize  .* tabs-command

hook -group kak-tabs global InsertIdle   .* tabs-set-modified
hook -group kak-tabs global NormalIdle   .* tabs-set-modified
hook -group kak-tabs global BufReload    .* tabs-set-modified


# ────────────── commands ──────────────
define-command tabs-command -params ..1 %{
  evaluate-commands %sh{
    if [ -n "$1" ]; then
      action="--action $1"
    fi

    eval "kak-tabs \
      --width '$kak_window_width' \
      --focused '$kak_bufname' \
      --modelinefmt '$kak_opt_tabs_modelinefmt' \
      $kak_opt_tabs_options \
      $kak_opt_tabs_modified_buffers \
      $action \
      $kak_quoted_buflist"
  }
}

define-command tabs-set-modified %{ try %{
  evaluate-commands %sh{
    if [ "$kak_opt_tabs_modified_buffer" = "$kak_modified" ]; then
      echo 'fail'
    fi
  }

  set-option buffer tabs_modified_buffer %val{modified}

  evaluate-commands %sh{
    if [ "$kak_modified" = "true" ]; then
      echo 'set-option -add global tabs_modified_buffers "--modified=%val{bufname}"'
    else
      echo 'set-option -remove global tabs_modified_buffers "--modified=%val{bufname}"'
    fi
  }

  tabs-command
}}

define-command tabs-only-focused -docstring "keep only the focused buffer" %{
  evaluate-commands %sh{
    eval "set -- $kak_quoted_buflist"
    while [ "$1" ]; do
      if [ "$1" != "$kak_bufname" ]; then
        echo "try %{ delete-buffer %[$1] }"
      fi
      shift
    done

    echo "echo -markup '{Information}deleted unmodified buffers'"
  }

  tabs-command
}

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
map global tabs k ": tabs-command first<ret>" -docstring "↑ first"
map global tabs j ": tabs-command last<ret>" -docstring "↓ last"

map global tabs f ": buffer " -docstring "¤ find"
map global tabs s ": edit -scratch *scratch*<ret>" -docstring "*scratch*"
map global tabs u ": edit -debug *debug*<ret>" -docstring "*debug*"

# arrange
map global tabs H ": tabs-command drag-left<ret>" -docstring "⇐ drag left"
map global tabs L ": tabs-command drag-right<ret>" -docstring "⇒ drag right"
map global tabs K ": tabs-command drag-first<ret>" -docstring "⇑ drag first"
map global tabs J ": tabs-command drag-last<ret>" -docstring "⇓ drag last"

# mutate
map global tabs d ": delete-buffer<ret>" -docstring "delete (focused)"
map global tabs o ": tabs-only-focused<ret>" -docstring "keep only (focused)"
