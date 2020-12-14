# nagger

A tool to remind lazy engineers to review pull requests of their peers.

# run it

Export env vars:

```bash
export REPO_OWNER=<owner>
export REPO_NAME=<name>
export SLACK_HOOK_URL="https://hooks.slack.com/services/<>/<>/<>"
export GH_API_TOKEN=<token>
```

Run it

```
cargo build --release
./target/release/nagger
```

Also availiable as a docker image:

```bash
docker run -d --name=nagger --rm --env=REPO_OWNER=$REPO_OWNER --env=REPO_NAME=$REPO_NAME --env=SLACK_HOOK_URL=$SLACK_HOOK_URL --env=GH_API_TOKEN=$GH_API_TOKEN fbielejec/nagger -d
```

Or with docker compose:

```yaml
nagger:
  image: fbielejec/nagger:latest
  container_name: nagger
  environment:
    - REPO_OWNER=$REPO_OWNER
    - REPO_NAME=$REPO_NAME"
    - SLACK_HOOK_URL=$SLACK_HOOK_URL
    - GH_API_TOKEN=$GH_API_TOKEN
```
