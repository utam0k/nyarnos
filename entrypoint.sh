#!/bin/bash

USER_NAME=nyarn
RUN_UID=${LOCAL_UID:-9001}
RUN_GID=${LOCAL_GID:-9001}

echo "Starting with UID : $RUN_UID, GID: $RUN_GID"
groupadd --non-unique --gid $RUN_GID $USER_NAME
useradd --non-unique --create-home --uid $RUN_UID --gid $USER_NAME --groups sudo $USER_NAME

echo "$USER_NAME ALL=(ALL) NOPASSWD: ALL" > /etc/sudoers.d/user-no-sudo-password

export HOME=/home/$USER_NAME

chown $RUN_UID:$RUN_GID -R $CARGO_HOME $RUSTUP_HOME

exec /usr/sbin/gosu $USER_NAME "$@"
