## About

This is a command that can be used as a `AuthorizedKeysCommand` for OpenSSH.
It fetches the public keys for all members of a specific GitHub team in a specific organization and allows them to login.

At the moment it is made for a single login user, so all members of the team are allowed to login as that user.

All keys are fetched in a single API call using the GitHub V4 API,
so the request latency should be a lot lower compared to solutions using the V3 API.

## Usage

* Build with `cargo build --release`, the binary will be in `target/release/ssh-auth-github`.
* Put a config file at `/etc/ssh-auth-github.ini`, and specify organization and team.
  The token should be a GitHub Oauth token with the `read:org` scope.
  You can create it under Settings -> Developer Settings -> Personal access tokens
* Add `AuthorizedKeysCommand /path/to/ssh-auth-github` in your `sshd_config` and reload sshd.

You can also create a `ssh-auth-github.ini` in this directory and build a container with it using
`docker build . -t sshtunnel`. That will create a container running SSH and only allow tunneling as the `tunnel` user.

## Limitations

It only fetches the first 100 users in the team and the first 100 keys for each user.
It does not yet attempt to do pagination to fetch more than that.

There is no caching, so you might run in to GitHub request limits.
At the point of writing the limit is roughly 500,000 public keys per hour.
So how many login attempts that translates into depends on the size of your team.

A simple way to do caching is to run this as a cron job and write out the results to the `authorized_keys` file,
instead of running it as a `AuthorizedKeysCommand`.

## Related work:

* https://github.com/cloudposse/github-authorized-keys
* https://github.com/trevoro/sshauth
