declare-user-mode tabs

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

    # sets `tabs` to the modelinefmt-formatted string for the current buflist
    render_tabs() {
      eval "set -- $kak_quoted_buflist"

      tabs=""
      tabs_length=0
      num_bufs=0

      for buf; do
        # if the buffer begins with an *, and we're not on that buffer, don't show it in tabs
        if [ "${buf%${buf#?}}" = '*' ] && [ "$kak_bufname" != "$buf" ]; then
          continue
        fi

        num_bufs=$(($num_bufs + 1))
        base_bufname=$(basename "$buf" | tail -c $max_tab_length)

        # highlight the current tab
        if [ "$buf" = "$kak_bufname" ]; then
          tabs="$tabs$kak_opt_tab_separator{Prompt}$padding$base_bufname$padding{Default}"
          tabs_length=$(($tabs_length + ${#kak_opt_tab_separator} + ${#padding} + ${#base_bufname} + ${#padding}))

        # otherwise render it in gray
        else
          tabs="$tabs$kak_opt_tab_separator{LineNumbers}$padding$base_bufname$padding{Default}"
          tabs_length=$(($tabs_length + ${#kak_opt_tab_separator} + ${#padding} + ${#base_bufname} + ${#padding}))
        fi
      done

      # account for the last separator
      tabs="$tabs$kak_opt_tab_separator"
      tabs_length=$(($tabs_length + ${#kak_opt_tab_separator}))
    }

    padding=" "
    max_length=$(($(tput cols) * $kak_opt_modeline_tabs_percentage / 100))
    max_tab_length=$max_length

    render_tabs

    # if tabs are too large, first try rendering without padding
    if [ $tabs_length -ge $max_length ]; then
      padding=""
      render_tabs
    fi

    # if tabs are still too large, render shortened versions of the names
    if [ $tabs_length -ge $max_length ]; then
      max_tab_length=$(($max_length / $num_bufs))
      render_tabs
    fi

    echo "$tabs"
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

    if [ "$direction" = "prev" ] && [ -n "$prev" ]; then
      echo ": buffer $prev<ret>"
    elif [ "$direction" = "next" ] && [ -n "$next" ]; then
      echo ": buffer $next<ret>"
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

hook global WinCreate .* %{
  hook window WinDisplay .* %{
    evaluate-commands refresh-buflist
  }

  hook window NormalIdle .* %{
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
map global tabs u ": buffer *debug*<ret>" -docstring "*debug*"

# modification
map global tabs r ": rename-buffer-prompt<ret>" -docstring "rename"

# deletion
map global tabs d ": delete-buffer<ret>" -docstring "delete (current)"
map global tabs D ": delete-saved-buffers<ret>" -docstring "delete (saved)"
map global tabs o ": delete-all-saved-except-current<ret>" -docstring "only (saved)"
map global tabs O ": delete-all-except-current<ret>" -docstring "only (force)"
