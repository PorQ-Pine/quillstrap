FROM ubuntu:24.04

# Missing stuff in container
# For some reason, apt update needs to be with the first install via &&
RUN apt-get update && apt-get install -y ripgrep htop psmisc wget unzip file tio vim
ENV PATH="${PATH}:/workspaces/quillstrap/other/path"

# Rust stuff
RUN apt-get install -y gcc build-essential curl ca-certificates pkg-config libinput10 pkg-config libusb-1.0-0-dev libftdi1-dev libudev-dev
ENV PATH="${PATH}:/root/.cargo/bin"

# VS Code stuff
RUN apt-get install -y git libxkbcommon-dev libgbm-dev
RUN git config --global --add safe.directory '*'
RUN git config --global core.editor "nano"
RUN git config --global pull.rebase false

# quillstrap stuff
# No idea why but it crashed without update here.
RUN apt-get update && apt-get install -y openssl libssl-dev clang sshpass

# U-boot build
RUN apt-get install -y coreutils device-tree-compiler python3 python3-pyelftools gcc-aarch64-linux-gnu

# quill-init deployment
RUN apt-get install -y lftp

## U-boot boot menu
RUN apt-get install -y python3-pillow fonts-dejavu-core fonts-freefont-ttf fontconfig imagemagick libpng-dev cmake
### Instal u001 font
RUN mkdir -p /tmp/fonti && \
    cd /tmp/fonti && \
    wget https://fontlibrary.org/assets/downloads/u001/3ea00b3c0c8fa6ce4373e5766fafd651/u001.zip && \
    unzip u001.zip && \
    mkdir -p /usr/share/fonts/u001 && \
    cp *.ttf /usr/share/fonts/u001/ && \
    fc-cache -fv && \
    rm -rf /tmp/fonti

# Build & install rkdeveloptool (From here, because docker)
RUN apt-get install -y meson libusb-1.0-0-dev udev
RUN cd /tmp && \
    git clone https://github.com/PorQ-Pine/rkdeveloptool.git && \
    cd rkdeveloptool && \
    meson build && \
    meson compile -C build && \
    meson install -C build && \
    rm -rf /tmp/rkdeveloptool

# Kernel
RUN apt-get install -y libncurses-dev flex bison libgmp-dev libmpc-dev libssl-dev libmpfr-dev findutils perl bc zstd kmod libfontconfig-dev

# Binfmt
RUN apt-get install -y qemu-user-static binfmt-support
RUN update-binfmts --enable

# Expose eMMC & backup eMMC
# Some of those are not needed but whatever
RUN apt-get install -y picocom lrzsz ckermit qemu-utils expect parted fdisk gdisk

# Eink kernel magic
RUN apt-get install -y python3-numpy

# Rootfs
RUN apt-get install -y squashfs-tools

# Zig (for cargo zigbuild)
ARG ZIG_VERSION=0.15.1
RUN set -e; \
    arch="$(dpkg --print-architecture)"; \
    case "$arch" in \
        amd64) url="https://github.com/PorQ-Pine/zig-package/releases/download/${ZIG_VERSION}/zig-x86_64-linux-${ZIG_VERSION}.tar.xz" ;; \
        arm64) url="https://github.com/PorQ-Pine/zig-package/releases/download/${ZIG_VERSION}/zig-aarch64-linux-${ZIG_VERSION}.tar.xz" ;; \
        *) echo "Unsupported architecture: $arch" && exit 1 ;; \
    esac; \
    curl -fSL -o /tmp/zig.tar.xz "$url"; \
    tar -xJf /tmp/zig.tar.xz -C /usr/bin --strip-components=1; \
    rm /tmp/zig.tar.xz

# niri
RUN apt-get install -y git-lfs

# Koreader
RUN apt-get install -y alien

# D2
RUN apt-get update && apt-get install -y \
    libnss3 \
    libnspr4 \
    libatk1.0-0t64 \
    libatk-bridge2.0-0t64 \
    libatspi2.0-0t64 \
    libxdamage1 \
    libasound2t64

RUN curl -fsSL https://d2lang.com/install.sh | sh -s --
# Should install stuff
RUN d2 --help
