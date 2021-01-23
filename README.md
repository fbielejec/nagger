# nagger

![alt text](https://github.com/fbielejec/nagger/blob/master/screenshot.png?raw=true)

A github crawler and slack bot that reminds lazy engineers to review pull requests of their peers.

# run it

Create a file in the [RON format](https://github.com/ron-rs/ron) which maps github user names to slack user names / ids:

```ron
(
    map: {
        "user1" : "id-or-name1",
        "user1" : "id2",
        "user2" : "id3"
    }
)
```

Export env vars:

```bash
export REPO_OWNER=<owner>
export REPO_NAME=<name>
export SLACK_HOOK_URL="https://hooks.slack.com/services/<>/<>/<>"
export GITHUB_API_TOKEN=<token>
export TIMER_INTERVAL=86400 # seconds, defaults to 43200 (12hours)
export USER_ID_PATH="/home/$USER/users.ron"
```

Run it

```
cargo build --release
./target/release/nagger
```

Also availiable as a docker image:

```bash
docker run --name=nagger -v /home/$USER/users.ron:/nagger/users.ron --rm --env=REPO_OWNER=$REPO_OWNER --env=REPO_NAME=$REPO_NAME --env=SLACK_HOOK_URL=$SLACK_HOOK_URL --env=GITHUB_API_TOKEN=$GITHUB_API_TOKEN --env=USER_ID_PATH=/nagger/users.ron fbielejec/nagger -d
```

Or with docker compose:

```yaml
nagger:
  image: fbielejec/nagger:latest
  container_name: nagger
  volumes:
    - /home/$USER/users.ron:/nagger/users.ron
  environment:
    - REPO_OWNER=$REPO_OWNER
    - REPO_NAME=$REPO_NAME
    - SLACK_HOOK_URL=$SLACK_HOOK_URL
    - GITHUB_API_TOKEN=$GITHUB_API_TOKEN
    - USER_ID_PATH=/nagger/users.ron
    - TIMER_INTERVAL=$TIMER_INTERVAL
```

# development

```bash
cargo watch -s "cargo run"
```
