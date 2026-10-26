#!/usr/bin/env bash
# Shared by the mise tasks. Sourced, never executed (no executable bit, so mise
# does not expose it as a task).
#
# Helpers:
#   die, note             one-line failure and progress output
#   load_deploy_env       sources the committed, non-secret configuration
#
# This repository has no secrets of its own: the Acta sensor credential is
# reused from Acta's podman secrets, so `.deploy.env` is the only deploy config
# file.

die() { echo "error: $*" >&2; exit 1; }
note() { echo "==> $*"; }

load_deploy_env() {
    [[ -f .deploy.env ]] || return 0
    # shellcheck disable=SC1091
    source .deploy.env
}
