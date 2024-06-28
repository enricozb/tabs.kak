# tabs.kak
#
# client-specific tab lists that render in the mode line


# ────────────── options ──────────────
declare-option -docstring "format string to render alongside tabs" str tabs_modelinefmt
declare-option -docstring "options to kak-tabs
--debug: print debugging output
" str tabs_options


# ────────────── hooks ──────────────
# NOTE: WinResize cannot be used as %val{client} is the empty string.
hook -group kak-tabs global ClientCreate .* "tabs create"
hook -group kak-tabs global WinDisplay .* "tabs display"

hook -group kak-tabs global InsertIdle .* tabs-on-modified
hook -group kak-tabs global NormalIdle .* tabs-on-modified
hook -group kak-tabs global BufReload .* tabs-on-modified


# ────────────── commands ──────────────
define-command -override tabs -params ..1 %{
  tabs-update-buflist-modified
  tabs-render %arg{1}
}

define-command -override tabs-render -params ..1 %{
  evaluate-commands %sh{
    eval "target/release/kak-tabs $1 \
      --session $kak_quoted_session \
      --client $kak_quoted_client \
      --bufname $kak_quoted_bufname \
      --session-buflist $kak_quoted_opt_tabs_buflist \
      --session-buflist-prev $kak_quoted_opt_tabs_buflist_prev \
      --client-buflists $kak_quoted_opt_tabs_client_buflists \
      --modelinefmt $kak_quoted_opt_tabs_modelinefmt \
      --width $kak_window_width \
      $kak_opt_tabs_options
    "
  }
}

define-command tabs-recommended-mapping %{
  map global normal b ': enter-user-mode tabs<ret>' -docstring 'tabs'
  map global normal B ': enter-user-mode -lock tabs<ret>' -docstring 'tabs (lock)'
}


# ────────────── mode ──────────────
declare-user-mode tabs

# navigate
map global tabs a "ga" -docstring "↔ alternate"
map global tabs h ": tabs prev<ret>" -docstring "← prev"
map global tabs l ": tabs next<ret>" -docstring "→ next"
map global tabs k ": tabs first<ret>" -docstring "↑ first"
map global tabs j ": tabs last<ret>" -docstring "↓ last"

map global tabs s ": edit -scratch *scratch*<ret>" -docstring "*scratch*"
map global tabs u ": edit -debug *debug*<ret>" -docstring "*debug*"

# arrange
map global tabs H ": tabs drag-left<ret>" -docstring "⇐ drag left"
map global tabs L ": tabs drag-right<ret>" -docstring "⇒ drag right"
map global tabs K ": tabs drag-first<ret>" -docstring "⇑ drag first"
map global tabs J ": tabs drag-last<ret>" -docstring "⇓ drag last"

# mutate
map global tabs d ": delete-buffer<ret>" -docstring "delete (focused)"
map global tabs o ": tabs only<ret>" -docstring "keep only (focused)"


# ────────────── state ──────────────
# this fake default value prevents kak-tabs from not receiving a tabs_buflist_pre on session creation.
declare-option -hidden str-list tabs_buflist "=false"
declare-option -hidden str-list tabs_buflist_prev
declare-option -hidden -docstring "an opaque representation of client buflists" str tabs_client_buflists
declare-option -hidden -docstring "whether this buffer is modified" bool tabs_modified_buffer false

define-command -override tabs-on-modified %{
  evaluate-commands %sh{
    if [ "$kak_opt_tabs_modified_buffer" != "$kak_modified" ]; then
      printf "set-option buffer tabs_modified_buffer %%val{modified}\n"
      printf "tabs modified\n"
    fi
  }
}

define-command -override tabs-update-buflist-modified %{
  set-option global tabs_buflist_prev %opt{tabs_buflist}
  set-option global tabs_buflist

  evaluate-commands %sh{
    eval set -- "$kak_quoted_buflist"
    while [ $# -gt 0 ]; do
      printf 'evaluate-commands -buffer %%{%s} %%{ set-option -add global tabs_buflist "%%val{bufname}=%%val{modified}" }\n' "$1"
      shift
    done
  }
}

# ------ testing ------

hook global ClientCreate .* %{
  set-option global tabs_modelinefmt '%val{cursor_line}:%val{cursor_char_column} {{mode_info}} {blue}%val{client}{Default}.{green}%val{session}{Default} '
  tabs-recommended-mapping
}

set-option global tabs_modelinefmt '%val{cursor_line}:%val{cursor_char_column} {{mode_info}} {blue}%val{client}{Default}.{green}%val{session}{Default} '
tabs-recommended-mapping

map global normal <backspace> ': tabs delete<ret>'

set-option global tabs_options --debug
