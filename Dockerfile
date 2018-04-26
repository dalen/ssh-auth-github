FROM rust:1.25 AS builder

RUN mkdir -p /home/rust
WORKDIR /home/rust
COPY . .
RUN cargo build --release

FROM debian:9.4 as runner

RUN apt-get update \
  && DEBIAN_FRONTEND=noninteractive apt-get install -y openssh-server ca-certificates libssl1.1 \
  && apt-get clean \
  && rm -rf /var/lib/apt/lists/*

COPY docker/sshd_config /etc/ssh/sshd_config
COPY docker/motd /etc/motd
COPY docker/instructions /usr/bin/instructions
RUN chmod 555 /usr/bin/instructions

COPY --from=builder /home/rust/target/release/ssh-auth-github /usr/bin
RUN adduser --disabled-password --shell /usr/bin/instructions --gecos Bastion,,,, bastion

COPY --chown=bastion:root ssh-auth-github.ini /etc/ssh-auth-github.ini
RUN chmod 440 /etc/ssh-auth-github.ini

RUN mkdir -p /run/sshd

EXPOSE 22

CMD [ "/usr/sbin/sshd", "-D", "-e" ]
