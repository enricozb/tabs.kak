declare-user-mode tabs

declare-option str modelinefmt_tabs %opt{modelinefmt}
declare-option str modeline_buflist
declare-option str switch_to_tab

hook global WinDisplay .* %{
  evaluate-commands refresh-buflist
}

define-command rename-buffer-prompt %{
  prompt -init %sh{ basename $kak_bufname } rename: %{
    rename-buffer %val{text}
    refresh-buflist
  }
}

define-command -hidden refresh-buflist %{
  set-option buffer modeline_buflist %sh{
    tabs=""
    declare -a "buffers=($kak_quoted_buflist)"
    for buf in "${buffers[@]}"; do
      if [[ $buf == "*debug*" && $kak_bufname != "*debug*" ]]; then
        continue
      fi

      if [[ $buf == $kak_bufname ]]; then
        tabs+="│{Prompt} $(basename "$buf") {Default}"
      else
        tabs+="│{LineNumbers} $(basename "$buf") {Default}"
      fi
    done
    echo "$tabs│"
  }
  set-option buffer modelinefmt "%opt{modelinefmt_tabs} - %opt{modeline_buflist}"
}

define-command tab-nav -params 1 %{
  execute-keys %sh{
    declare -a "buffers=($kak_quoted_buflist)"
    done=false
    for buf in "${buffers[@]}"; do
      if $done; then
        break
      fi

      if [[ $buf == "*debug*" && $kak_bufname != "*debug*" ]]; then
        continue
      fi

      if [[ $buf == $kak_bufname ]]; then
        done=true
        prev=$last
      fi
      last=$buf
    done
    next=$buf

    if [[ $1 == "prev" && -n $prev ]]; then
      echo ": buffer $prev<ret>"
    elif [[ $1 == "next" && -n $next ]]; then
      echo ": buffer $next<ret>"
    fi
  }
  refresh-buflist
}

define-command tab-move -params 1 %{
  execute-keys %sh{
    declare -a "buffers=($kak_quoted_buflist)"
    done=false
    for buf in "${buffers[@]}"; do
      if $done; then
        break
      fi

      if [[ $buf == "*debug*" && $kak_bufname != "*debug*" ]]; then
        continue
      fi

      if [[ $buf == $kak_bufname ]]; then
        done=true
        prev=$last
      fi
      last=$buf
    done
    next=$buf
    curr=$kak_bufname

    # prev, curr, and next are now set properly.
    # prev/next will be empty if curr is at the front /back of the buflist
    if [[ $1 == "prev" && -n $prev ]]; then
      swap=$prev
    elif [[ $1 == "next" && -n $next ]]; then
      swap=$next
    else
      exit
    fi

    bufs_to_arrange=""
    for buf in "${buffers[@]}"; do
      if [[ $buf == $swap ]]; then
        buf=$curr
      elif [[ $buf == $curr ]]; then
        buf=$swap
      fi
      bufs_to_arrange+="'$buf' "
    done

    if [[ -n $bufs_to_arrange ]]; then
      echo ": arrange-buffers $bufs_to_arrange<ret>"
    fi
  }
  refresh-buflist
}

map global tabs h ": tab-nav prev<ret>" -docstring "prev ←"
map global tabs l ": tab-nav next<ret>" -docstring "next →"

map global tabs H ": tab-move prev<ret>" -docstring "rearrange ←"
map global tabs L ": tab-move next<ret>" -docstring "rearrange →"

map global tabs r ": rename-buffer-prompt<ret>" -docstring "rename"

map global tabs d ": delete-buffer<ret>" -docstring "delete"
map global tabs u ": buffer *debug*<ret>" -docstring "debug"
