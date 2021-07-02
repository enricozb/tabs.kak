require-module luar

declare-user-mode tabs

declare-option str modelinefmt_tabs %opt{modelinefmt}
declare-option str tabs_overlow "shrink"
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
  "refresh-buflist-%opt{tabs_overlow}"

  set-option buffer modelinefmt "%opt{modelinefmt_tabs} - %opt{modeline_buflist}"
}

define-command -hidden refresh-buflist-scroll %{
  lua %val{bufname} %sh{echo "$kak_quoted_buflist" | xargs -n1 printf '%s\n'} %sh{tput cols} %opt{modeline_tabs_percentage} %opt{tab_separator} %{
    local currbuf, buflist, num_cols, tabs_percentage, separator = args()

    -- basename of a path
    function basename(str)
      local base = string.gsub(str, "(.*/)(.*)", "%2")
      return base
    end

    -- reverse a list and return it
    function reverse(xs)
      for i = 1, #xs / 2 do xs[i], xs[#xs - i + 1] = xs[#xs - i + 1], xs[i] end
      return xs
    end

    -- concatenate many arrays
    function merge(ts)
      local merged = {}
      for i, t in ipairs(ts) do
        for _, e in ipairs(t) do
          merged[#merged + 1] = e
        end
      end
      return merged
    end

    -- shorten a list of tabs to fit in the max_length, optionally trimming from the left instead of the right
    function shorten(tabs, max_length, trim_left)
      -- a single separator will always be present
      local length = 1
      local shortened_tabs = {}

      -- start (s), end (e), delta (d) for iteration
      local s = 1
      local e = #tabs
      local d = 1
      if trim_left then
        s = #tabs
        e = 1
        d = -1
      end

      for i = s, e, d do
        -- +3 because of two surrounding spaces and additional separator
        length = length + string.len(tabs[i]) + 3
        if length > max_length then
          -- return before we have the chance to overflow `shortened_tabs`
          if trim_left then
            return reverse(shortened_tabs), true
          end
          return shortened_tabs, true
        end
        shortened_tabs[#shortened_tabs + 1] = tabs[i]
      end

      return tabs, false
    end

    -- max_width is the maximum width of the entire tab bar
    local max_width = math.floor(num_cols * tabs_percentage / 100)

    -- max_unselected_tabs is the maximum width of either side of the unselected tabs
    -- | ... unselected left tabs ... | selected tab | ... unselected right tabs ... |
    -- the -1 is because of this space ^ between the selected and unselected tabs
    local max_unselected_tabs = (max_width - string.len(basename(currbuf))) / 2 - 1

    -- split buflist into names left and right of the current tab
    local left = {}
    local right = {}
    local section = left
    for tab in buflist:gmatch("([^\n]+)") do
      if tab == currbuf then
        section = right
        goto continue
      end

      -- skip special *bufnames*
      if string.sub(tab, 1, 1) == "*" then
        goto continue
      end

      section[#section + 1] = basename(tab)
      ::continue::
    end

    -- shorten left and right if necessary
    local left, left_shorter = shorten(left, max_unselected_tabs, true)
    local right, right_shorter = shorten(right, max_unselected_tabs, false)

    local left_sep = (left_shorter and "…" or separator) .. " "
    local right_sep = " " .. (right_shorter and "…" or separator)

    -- join tabs with separator and formatting
    local tabs = table.concat(merge({left, {"{Prompt}" .. basename(currbuf)}, right}), " {Default}" .. separator .. "{LineNumbers} ")
    kak.set_option("buffer", "modeline_buflist", left_sep .. "{LineNumbers}" .. tabs .. "{Default}" .. right_sep)
  }
}

define-command -hidden refresh-buflist-shrink %{
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
        basename_buf=$(echo "${buf##*/}" | tail -c $max_tab_length)

        if [ "$buf" = "$kak_bufname" ]; then
          tab_color="{Prompt}"
        else
          tab_color="{LineNumbers}"
        fi

        tabs="$tabs$kak_opt_tab_separator$tab_color$padding$basename_buf$padding{Default}"
        tabs_length=$(($tabs_length + ${#kak_opt_tab_separator} + ${#padding} + ${#basename_buf} + ${#padding}))
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
