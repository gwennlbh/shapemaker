name: Tangle (mirro to tangled.sh)

on:
  push: {}
  workflow_dispatch: {}

jobs:
  tangle:
    runs-on: ubuntu-latest
    steps:
      - uses: gwennlbh/to-tangled@v0.3
        with:
          repo: gwen.works/to-tangled
          ssh-key: ${{ secrets.TANGLED_KEY }}
