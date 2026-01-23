# SPDX-FileCopyrightText: 2025 Spencer
# SPDX-License-Identifier: CC0-1.0

_aletheia() {
  local commands="backup restore update update_gamedb update_custom_gamedbs"
  local input="${COMP_WORDS[COMP_CWORD]}"

  if [[ ${COMP_CWORD} -eq 1 ]]; then
    COMPREPLY=($(compgen -W "$commands" -- "$input"))
  fi
}

complete -F _aletheia aletheia
