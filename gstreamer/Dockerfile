FROM ubuntu:24.04

ENV TZ=Asia/Tokyo
RUN apt-get update && \
      DEBIAN_FRONTEND=noninteractive apt-get install -y tzdata && \
      apt-get install -y curl git

# Install GStreamer
RUN apt-get install -y libgstreamer1.0-dev libgstreamer-plugins-base1.0-dev libgstreamer-plugins-bad1.0-dev gstreamer1.0-plugins-base gstreamer1.0-plugins-good gstreamer1.0-plugins-bad gstreamer1.0-plugins-ugly gstreamer1.0-libav gstreamer1.0-tools gstreamer1.0-x gstreamer1.0-alsa gstreamer1.0-gl gstreamer1.0-gtk3 gstreamer1.0-qt5 gstreamer1.0-pulseaudio

# Install Rust
ENV RUST_VERSION stable
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain ${RUST_VERSION}
ENV PATH="/root/.cargo/bin:$PATH"

# Install gst-plugins-rs
RUN apt-get install -y build-essential libssl-dev pkg-config && \
      git clone https://gitlab.freedesktop.org/gstreamer/gst-plugins-rs.git ~/gst-plugins-rs

WORKDIR /root/gst-plugins-rs
