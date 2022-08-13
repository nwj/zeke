#!/bin/bash

function create_note () {
  local file_name="$1"
  local tag="$2"
  local link="$3"

  local file_content
  file_content="---\ntitle: ${file_name}\ncreated: 2021-09-07T16:44:27.201207Z\n"

  if [[ -n "${tag}" ]]; then
    file_content+="tags:\n- ${tag}\n"
  else
    file_content+="tags: []\n"
  fi

  if [[ -n "${link}" ]]; then
    file_content+="links:\n- ${link}\n"
  else
    file_content+="links: []\n"
  fi

  file_content+="---\nLorem ipsum dolor sit amet, consectetur adipiscing elit."

  echo -e "${file_content}" > "${file_name}"
}

function before_all () {
  mkdir -p "tmp/template"

  local tags
  local links

  tags=(alpha beta gamma delta)
  links=(4.md 3.md 2.md 1.md)

  local mod
  local filename
  local tag
  local link
  for i in {0..99}; do
    mod=$((i % 5))
    filename="tmp/template/${i}.md"

    if [[ "${i}" -lt 5 || "${mod}" == 4 ]]; then
      create_note "${filename}"
    else
      tag="${tags[$mod]}"
      link="${links[$mod]}"
      create_note "${filename}" "${tag}" "${link}"
    fi
  done

  export -f before_each
  export -f t
}

function before_each () {
  rm -rf tmp/test
  cp -R tmp/template tmp/test
}

function after_all () {
  rm -rf ./tmp
}

function t () {
  pushd ./tmp/test && "$@" && pushd || exit
}

function main () {
  before_all

  hyperfine --warmup 10 --prepare before_each "t zeke new 'foo'" \
    "t zeke mv '1.md' 'foo.md'" \
    "t zeke tag 'gamma' '1.md'" \
    "t zeke untag 'alpha' '5.md'" \
    "t zeke tags" \
    "t zeke link 1.md 2.md" \
    "t zeke link 1.md 2.md" \
    "t zeke unlink 4.md 5.md" \
    "t zeke backlink"

  after_all
}

main "$@"
