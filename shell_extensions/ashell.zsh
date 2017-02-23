autoload -Uz add-zsh-hook
add-zsh-hook preexec __ashell_preexec
add-zsh-hook precmd __ashell_precmd
add-zsh-hook chpwd __ashell_pwd_changed

__ashell_lastcmd=''
__ashell_command_entered() {}
__ashell_pwd_changed() {}
__ashell_command_failed() {}

if (which aplay >/dev/null 2>&1)
then
  __ashell_player=aplay
else
  __ashell_player=afplay
fi

__ashell_playse() {
  ($__ashell_player "/tmp/ashell/sounds/${1}" >/dev/null 2>&1 &)
}

__ashell_preexec() {
  __ashell_lastcmd=$1

  __ashell_command_entered $__ashell_lastcmd
}

__ashell_precmd() {
  __ashell_last_status=$?

  if [[ $__ashell_last_status -eq 0 ]]
  then
    __ashell_command_succeeded $__ashell_lastcmd
  else
    __ashell_command_failed $__ashell_lastcmd $__ashell_last_status
  fi
}

__ashell_command_entered() {
  __ashell_playse "se-preexec.wav"
}

__ashell_pwd_changed() {
    __ashell_playse "se-chdir.wav"
}

__ashell_command_succeeded() {
  __ashell_playse "se-succeeded.wav"
}

__ashell_command_failed() {
  __ashell_playse "se-failed.wav"
}
