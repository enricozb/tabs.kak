declare-user-mode tabs

declare-option str modelinefmt_tabs %opt{modelinefmt}
declare-option str modeline_buflist
declare-option str switch_to_tab

hook global WinDisplay .* %{
  evaluate-commands refresh-buflist
}

hook global ModeChange .*next-key[user.tabs].* %{
  echo -debug popped
}

define-command -hidden refresh-buflist %{
  set-option buffer modeline_buflist %sh{
    tabs=""
    declare -a "buffers=($kak_quoted_buflist)"
    for buf in "${buffers[@]}"; do
      if [[ $buf = $kak_bufname ]]; then
        tabs+="│{MenuBackground} $(basename "$buf") {Default}"
      else
        tabs+="│ $(basename "$buf") "
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

      if [[ "$buf" == "$kak_bufname" ]]; then
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

map global tabs ] ": tab-nav next<ret>" -docstring "next →"
map global tabs [ ": tab-nav prev<ret>" -docstring "prev ←"
map global tabs l ": enter-user-mode -lock tabs<ret>" -docstring "lock"
