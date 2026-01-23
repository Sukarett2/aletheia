# SPDX-FileCopyrightText: 2025 Spencer
# SPDX-License-Identifier: CC0-1.0

set -l commands backup restore update update_gamedb update_custom_gamedbs

complete -c aletheia -n "not __fish_seen_subcommand_from $commands" -a "backup" -d "Create a backup"
complete -c aletheia -n "not __fish_seen_subcommand_from $commands" -a "restore" -d "Restore from backup"
complete -c aletheia -n "not __fish_seen_subcommand_from $commands" -a "update" -d "Update the application"
complete -c aletheia -n "not __fish_seen_subcommand_from $commands" -a "update_gamedb" -d "Update GameDB"
complete -c aletheia -n "not __fish_seen_subcommand_from $commands" -a "update_custom_gamedbs" -d "Update custom GameDBs"
