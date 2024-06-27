# tabs.kak
#
# client-specific tab lists that render in the mode line


# ────────────── options ──────────────
declare-option -docstring "format string to render alongside tabs" str tabs_modelinefmt


# ────────────── hooks ──────────────
hook -group kak-tabs global ClientCreate .* "tabs create"
hook -group kak-tabs global ClientClose .* "tabs close"
# TODO: ClientRename

hook -group kak-tabs global WinDisplay .* tabs
hook -group kak-tabs global WinResize .* tabs

# fired when a buffer may have been modified (if so, all clients should update)
hook -group kak-tabs global InsertIdle .* tabs
hook -group kak-tabs global NormalIdle .* tabs
hook -group kak-tabs global BufReload .* tabs


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
      --width $kak_window_width
    "
  }
}


# ────────────── state ──────────────
declare-option -hidden str-list tabs_buflist
declare-option -hidden str-list tabs_buflist_prev
declare-option -docstring "an opaque representation of client buflists" -hidden str tabs_client_buflists

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
