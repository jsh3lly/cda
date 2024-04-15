cda() {
    path_or_error=$(cda-bin "$@")
    if [ $? -eq 0 ]; then   # Path
        cd $path_or_error
    else                    # Err
        echo $path_or_error
    fi
}
