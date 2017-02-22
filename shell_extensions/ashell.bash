source $(dirname $BASH_SOURCE)/bash-preexec.sh

preexec_functions+=(__ashell_preexec)
precmd_functions+=(__ashell_precmd)

__ashell_lastpwd=$PWD
__ashell_lastcmd=''

__ashell_playse() {
  (aplay "/tmp/ashell/sounds/${1}" >/dev/null 2>&1 &)
}

__ashell_preexec() {
  __ashell_lastcmd=$PWD
  __ashell_lastcmd=$1

  __ashell_command_entered $__ashell_lastcmd
}

__ashell_precmd() {
  __ashell_last_status=$?

  if [[ $__ashell_lastpwd != $PWD ]]
  then
    __ashell_pwd_changed
  fi

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

__ashell_command_failed() {
  __ashell_playse "se-failed.wav"
}

__ashell_command_succeeded() {
  __ashell_playse "se-succeeded.wav"
}
