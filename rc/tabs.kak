require-module luar

declare-user-mode tabs

# copy tabs.com file from dist/ to bin/ if it doesn't already exist
declare-option -hidden str tabs_bin %sh{
  dir="$(dirname $kak_source)/../"
  bin_from_git="$dir/dist/tabs.com"
  bin_to_use="$dir/bin/tabs.com"

  # if the binary to use is different than the one from git, copy over the one from git
  if [ ! -f "$bin_to_use" ]; then
    mkdir -p "$(dirname $bin_to_use)"
    cp "$bin_from_git" "$bin_to_use"
  elif ! cmp -s "$bin_from_git" "$bin_to_use"; then
    cp "$bin_from_git" "$bin_to_use"
  fi

  printf "$bin_to_use"
}

declare-option str modelinefmt_tabs %opt{modelinefmt}
declare-option str modeline_tabs_percentage 80
declare-option str tab_separator "|"

declare-option -hidden str modeline_buflist

define-command rename-buffer-prompt %{
  prompt -init %sh{ basename "$kak_bufname" } rename: %{
    rename-buffer %val{text}
    refresh-buflist
  }
}

define-command -hidden refresh-buflist %{
  set-option buffer modeline_buflist %sh{
    eval "$kak_opt_tabs_bin $(tput cols) "$kak_opt_modeline_tabs_percentage" '$kak_opt_tab_separator' '$kak_bufname' $kak_quoted_buflist"
  }
  set-option buffer modelinefmt "%opt{modelinefmt_tabs} - %opt{modeline_buflist}"
}

define-command tab-nav -params 1 %{
  execute-keys %sh{
    direction="$1"
    done=false

    eval "set -- $kak_quoted_buflist"
    for buf; do
      if $done; then
        break
      fi

      if [ "$buf" = "*debug*" ] && [ "$kak_bufname" != "*debug*" ]; then
        continue
      fi

      if [ "$buf" = "$kak_bufname" ]; then
        done=true
        prev="$last"
      fi
      last="$buf"
    done
    next="$buf"

    if [ "$direction" = 'prev' ] && [ -n "$prev" ] && [ "$prev" != '*debug*' ]; then
      echo ": buffer '$prev'<ret>"
    elif [ "$direction" = 'next' ] && [ -n "$next" ] && [ "$next" != '*debug*' ]; then
      echo ": buffer '$next'<ret>"
    fi
  }
  refresh-buflist
}

define-command tab-move -params 1 %{
  execute-keys %sh{
    direction="$1"
    done=false

    eval "set -- $kak_quoted_buflist"
    for buf; do
      if $done; then
        break
      fi

      if [ "$buf" = "*debug*" ] && [ "$kak_bufname" != "*debug*" ]; then
        continue
      fi

      if [ "$buf" = "$kak_bufname" ]; then
        done=true
        prev="$last"
      fi
      last="$buf"
    done
    next="$buf"
    curr="$kak_bufname"

    # prev, curr, and next are now set properly.
    # prev/next will be empty if curr is at the front /back of the buflist
    if [ "$direction" = "prev" ] && [ -n "$prev" ]; then
      swap="$prev"
    elif [ "$direction" = "next" ] && [ -n "$next" ]; then
      swap="$next"
    else
      exit
    fi

    bufs_to_arrange=""
    for buf; do
      if [ "$buf" = "$swap" ]; then
        buf="$curr"
      elif [ "$buf" = "$curr" ]; then
        buf="$swap"
      fi
      bufs_to_arrange="$bufs_to_arrange'$buf' "
    done

    if [ -n "$bufs_to_arrange" ]; then
      echo ": arrange-buffers $bufs_to_arrange<ret>"
    fi
  }
  refresh-buflist
}

define-command delete-saved-buffers -docstring "delete all saved buffers" %{
  evaluate-commands %sh{
    deleted=0
    eval "set -- $kak_quoted_buflist"
    while [ "$1" ]; do
      echo "try %{delete-buffer '$1'}"
      echo "echo -markup '{Information}$deleted buffers deleted'"
      deleted=$((deleted+1))
      shift
    done
  }
}

define-command delete-all-saved-except-current -docstring "delete all saved buffers except current one" %{
  evaluate-commands %sh{
    deleted=0
    eval "set -- $kak_quoted_buflist"
    while [ "$1" ]; do
      if [ "$1" != "$kak_bufname" ]; then
        echo "try %{delete-buffer '$1'}"
        echo "echo -markup '{Information}$deleted buffers deleted'"
        deleted=$((deleted+1))
      fi
      shift
    done
  }

  refresh-buflist
}

define-command delete-all-except-current -docstring "delete all buffers except current one" %{
  evaluate-commands %sh{
    deleted=0
    eval "set -- $kak_quoted_buflist"
    while [ "$1" ]; do
      if [ "$1" != "$kak_bufname" ]; then
        echo "delete-buffer! '$1'"
        echo "echo -markup '{Information}$deleted buffers deleted'"
        deleted=$((deleted+1))
      fi
      shift
    done
  }

  refresh-buflist
}

hook -group tabs-kak global WinCreate .* %{
  hook -group tabs-kak window WinDisplay .* %{
    evaluate-commands refresh-buflist
  }

  hook -group tabs-kak window WinResize .* %{
    evaluate-commands refresh-buflist
  }
}


# navigation
map global tabs a "ga" -docstring "↔ alt"
map global tabs h ": tab-nav prev<ret>" -docstring "← prev"
map global tabs l ": tab-nav next<ret>" -docstring "→ next"
map global tabs f ": buffer " -docstring "find"

# rearrangement
map global tabs H ": tab-move prev<ret>" -docstring "← drag left"
map global tabs L ": tab-move next<ret>" -docstring "→ drag right"

# common buffers
map global tabs c ": edit %val{config}/kakrc<ret>" -docstring "config (kakrc)"
map global tabs s ": edit -scratch *scratch*<ret>" -docstring "*scratch*"
map global tabs u ": edit -debug *debug*<ret>" -docstring "*debug*"

# modification
map global tabs r ": rename-buffer-prompt<ret>" -docstring "rename"

# deletion
map global tabs d ": delete-buffer<ret>" -docstring "delete (current)"
map global tabs D ": delete-saved-buffers<ret>" -docstring "delete (saved)"
map global tabs o ": delete-all-saved-except-current<ret>" -docstring "only (saved)"
map global tabs O ": delete-all-except-current<ret>" -docstring "only (force)"
