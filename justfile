package_prefix := "dive-"
cw_template_version := "v2"

# make node binaries available in recipes
export PATH := env_var('PWD') + "/node_modules/.bin:" + env_var('PATH')

_list:
    @just --list --no-aliases

prep: update-rs-deps

check: check-format

alias fmt := format
format FILES="'**/*'":
    dprint fmt {{FILES}}
    sort-package-json '**/package.json'

[group("prep")]
update-rs-deps:
  cargo install \
    just@1.34.0 \
    toml-cli@0.2.3 \
    cargo-workspaces@0.3.6 \
    cargo-generate@0.21.3 \
    cargo-run-script@0.2.0

[group("prep")]
update-js-deps:
  corepack enable
  pnpm install

[group("create")]
create-contract NAME:
    # add the contract to workspace members
    cargo ws create --lib --name {{ package_prefix }}{{ NAME }} --edition 2021 contracts/{{ NAME }}
    rm -rf contracts/{{ NAME }}/*
    # generate contract skeleton
    cargo generate --init \
      --git https://github.com/deepcomet/cw-template \
      --tag {{ cw_template_version }} \
      --name {{ package_prefix }}{{ NAME }} \
      --destination contracts/{{ NAME }} \
      -d minimal=true \
      -d authors="mintthemoon <mint@mintthemoon.xyz>"
    # clean template
    cd contracts/{{NAME}} && rm -rf .editorconfig .gitignore .github .circleci 
    # set init version (hardcoded to 0.1.0 in template)
    toml set contracts/{{NAME}}/Cargo.toml package.version 0.0.0 > contracts/{{NAME}}/Cargo.toml.tmp
    mv contracts/{{NAME}}/Cargo.toml.tmp contracts/{{NAME}}/Cargo.toml
    # format workspace Cargo.toml (sort members)
    just format Cargo.toml
    # format generated code
    just format "'contracts/{{NAME}}/**/*'"


[group("create")]
create-rs-package NAME:
    # create new package linked to workspace members
    cargo ws create --lib --name {{ package_prefix }}{{ NAME }} --edition 2021 packages/rs/{{ NAME }}
    # format workspace Cargo.toml (sort members)
    just format Cargo.toml
    # format generated code
    just format "'packages/rs/{{NAME}}/**/*'"

[group("create")]
create-js-package NAME:


[group("check")]
check-format:
    dprint check
