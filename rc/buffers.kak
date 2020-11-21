declare-option str modelinefmt_tabs %opt{modelinefmt}
declare-option str modeline_buflist
declare-option str switch_to_buffer
declare-user-mode tabs

hook global WinDisplay .* %{
  evaluate-commands refresh-buflist
}

define-command -hidden refresh-buflist %{
  set-option buffer modeline_buflist %sh{
    tabs=""
    declare -a "buffers=($kak_quoted_buflist)"
    for buf in "${buffers[@]}"; do
      if [[ $buf = $kak_bufname ]]; then
        tabs+="│{MenuForeground}$(basename "$buf"){Default}"
      else
        tabs+="│$(basename "$buf")"
      fi
    done
    echo $tabs│
  }
  set-option buffer modelinefmt "%opt{modelinefmt_tabs} - %opt{modeline_buflist}"
}

define-command buffer-nav -params 1 %{
  execute-keys %sh{
    declare -a "buffers=($kak_quoted_buflist)"
    for buf in "${buffers[@]}"; do
      if [[ -n $prev ]]; then
        break
      fi

      if [[ $buf == $kak_bufname ]]; then
        prev=$last
      fi
      last=$buf
    done
    next=$buf

    if [[ $1 == "prev" && -n $prev ]]; then
      echo ":buffer $prev<ret>"
    elif [[ $1 == "next" ]]; then
      echo ":buffer $next<ret>"
    fi
  }
  refresh-buflist
}

map global tabs n ": buffer-nav next<ret>" -docstring "next →"
map global tabs p ": buffer-nav prev<ret>" -docstring "prev ←"
