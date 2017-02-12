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

  if begin; test -n $argv; and test $__ashell_last_status -ne 0; end
    __ashell_command_failed $argv[1] $__ashell_last_status
  end
end

function __ashell_pwd_changed --on-variable PWD
    __ashell_playse "se-chon.wav"
end

function __ashell_command_entered
  for _ in (string split ' ' $argv[1])
    __ashell_playse "se-awa.wav"
    sleep 0.05
  end
end

function __ashell_command_failed
  if test $argv[2] -eq 127
    __ashell_playse "se-kabe.wav"
    return
  end

  for _ in (string split ' ' $argv[1])
    __ashell_playse "se-kabe.wav"
    sleep 0.05
  end
end
