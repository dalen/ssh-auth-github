## About

This is a command that can be used as a `AuthorizedKeysCommand` for OpenSSH.
It fetches the public keys for all members of a specific GitHub team in a specific organization and allows them to login.

At the moment it is made for a single login user, so all members of the team are allowed to login as that user.

## Usage

* Build with `cargo build --release`, the binary will be in `target/release/ssh-auth-github`.
* Put a config file at `/etc/ssh-auth-github.ini`, and specify organization and team.
  The token should be a GitHub Oauth token with the `read:org` scope.
  You can create it under Settings -> Developer Settings -> Personal access tokens
* Add `AuthorizedKeysCommand /path/to/ssh-auth-github` in your `sshd_config` and reload sshd.

You can also create a `ssh-auth-github.ini` in this directory and build a container with it using
`docker build . -t sshtunnel`. That will create a container running SSH and only allow tunneling as the `tunnel` user.

## Related work:

* https://github.com/cloudposse/github-authorized-keys
* https://github.com/trevoro/sshauth
