# tabs.kak
#
# client-specific tab lists that render in the mode line

# ────────────── options ──────────────
declare-option str tabs_modelinefmt
declare-option str tabs_options

# ────────────── hooks ──────────────
hook -group kak-tabs global ClientCreate .* %{ tabs new }

hook -group kak-tabs global WinDisplay .* tabs
hook -group kak-tabs global WinResize  .* tabs

# fired when a buffer may have been modified (if so, all clients should update)
hook -group kak-tabs global InsertIdle .* tabs
hook -group kak-tabs global NormalIdle .* tabs
hook -group kak-tabs global BufReload  .* tabs

# ────────────── commands ──────────────
define-command -hidden tabs -params ..1 %{
  update-buflist-modified

  evaluate-commands %sh{
    eval "kak-tabs $1 \
      --session $kak_quoted_session \
      --client $kak_quoted_client \
      --width $kak_window_width \
      --bufname $kak_quoted_bufname \
      --session-buflist $kak_quoted_opt_tabs_buflist_modified \
      --session-buflist-prev $kak_quoted_opt_tabs_buflist_modified_prev \
      --client-bufindices $kak_quoted_opt_tabs_client_bufindices \
      --modelinefmt $kak_quoted_opt_tabs_modelinefmt \
      $kak_quoted_opt_tabs_options
    "
  }
}

# ────────────── state ──────────────
declare-option -hidden str-to-str-map tabs_client_bufindices
declare-option -hidden str-to-str-map tabs_buflist_modified
declare-option -hidden str-to-str-map tabs_buflist_modified_prev

define-command -override update-buflist-modified %{
  set-option global buflist_modified_prev %opt{buflist_modified}

  evaluate-commands %sh{
    eval set -- "$kak_quoted_buflist"
    while [ $# -gt 0 ]; do
      printf 'evaluate-commands -buffer %%{%s} %%{ set-option -add global buflist_modified "%%val{bufname}=%%val{modified}" }\n' "$1"
      shift
    done
  }
}
