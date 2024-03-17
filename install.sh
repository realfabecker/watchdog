#!/usr/bin/env bash
set -e

# bash message log with info format
info() {
  command printf '\033[1;32m%12s\033[0m %s\n' "$1" "$2" 1>&2
}

# bash message log with erro format
error() {
  command printf '\033[1;31mError\033[0m: %s\n\n' "$1" 1>&2
}

# check profile configuration for binary access
check_profile_configration() {
  if [[ $(command -v watchdog && echo "ok" || echo "no") == "no" ]]; then
    p='$PATH'
    cat <<EOF

  Please, include the following lines to your profile configuration

  export $p:$watchdog_dir/bin

EOF
  fi
}

# download release from latest available at github
download_from_latest() {
  info "Downloading" "release from github ${repo_url}"
  curl -s "$repo_url" \
    | grep "browser_download_url.*linux" \
    | sed -E 's/.*"(http.*)"/\1/g' \
    | xargs -n1 -I{} curl -sL -o "${down_dir}/${down_fle}" {}

  if [[ ! -f "${down_dir}/${down_fle}" ]];then
    error "unable to download release from github"
    exit 1
  fi;
}

# install latest release available at github
install_latest_release() {
  info "Extracting" "binary and creating symbolic links"
  mkdir -p "${watchdog_dir}/bin" \
    && mv "${down_dir}/${down_fle}" "${watchdog_dir}/bin/watchdog"

  if [[ -d "${HOME}/bin" ]]; then
    ln -s -f "${watchdog_dir}/bin/watchdog" "${HOME}/bin/watchdog"
  fi
  chmod +x "$watchdog_dir/bin/watchdog"
}

# install base configuration
watchdog_dir="$HOME/.watchdog"
down_dir=/tmp
down_fle=watchdog
repo_url=https://api.github.com/repos/realfabecker/watchdog/releases/latest

# download latest release from github repository
download_from_latest

# extract and link watchdog binary and user links
install_latest_release

# final completion log output from installer
info "Completed" "watchdog installation at $watchdog_dir/bin/watchdog"

# checking profile configuration for watchdog
check_profile_configration