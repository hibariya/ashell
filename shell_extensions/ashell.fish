set __ashell_lastdir $PWD

function __ashell_playse
  aplay "/tmp/ashell/sounds/$argv[1]" >/dev/null ^&1 &
end

function __ashell_on_before_command --on-event fish_preexec
  if test -n $argv
    set __ashell_lastdir $PWD
    __ashell_command_entered $argv[1]
  end
end

function __ashell_on_after_command --on-event fish_postexec
  set __ashell_last_status $status

  if test $__ashell_lastdir != $PWD
    __ashell_pwd_changed
  end

  if test -n $argv
    if test $__ashell_last_status -eq 0
      __ashell_command_succeeded $argv[1]
    else
      __ashell_command_failed $argv[1] $__ashell_last_status
    end
  end
end

function __ashell_pwd_changed --on-variable PWD
  __ashell_playse "se-chdir.wav"
end

function __ashell_command_entered
  __ashell_playse "se-preexec.wav"
end

function __ashell_command_succeeded
  __ashell_playse "se-succeeded.wav"
end

function __ashell_command_failed
  __ashell_playse "se-failed.wav"
end
