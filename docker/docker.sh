sudo docker run --privileged --cap-add MKNOD --cap-add SYS_ADMIN \
    -e LOCAL_UID="$(id -u $USER)" -e LOCAL_GID="$(id -g $USER)" \
    -v nyarn-"$(id -u $USER)-$(id -g $USER)"-cargo:/usr/local/cargo \
    -v nyarn-"$(id -u $USER)-$(id -g $USER)"-rustup:/usr/local/rustup \
    -v "$(pwd):/home/utam0k/nyarn" -w "/home/utam0k/nyarn" --rm -it nyarn "$@"
