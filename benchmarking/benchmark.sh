#!/bin/bash

# Configuration
WARMUP_COUNT=5
MIN_RUNS=50
DIR_SIZE=1000

# Commands to benchmark
COMMAND_BACKLINK="zeke backlink"
COMMAND_GRAPH="zeke graph"
COMMAND_MV="zeke mv"
COMMAND_TAGS="zeke tags"

if ! which hyperfine > /dev/null 2>&1; then
	echo "'hyperfine' does not seem to be installed."
	echo "You can get it here: https://github.com/sharkdp/hyperfine"
	exit 1
fi

if ! which zeke > /dev/null 2>&1; then
	echo "'zeke' does not seem to be installed."
	echo "You can get it here: https://github.com/nwj/zeke"
	exit 1
fi

for ((i=0;i<$DIR_SIZE;i++));
do
	echo -ne "Creating $i / $DIR_SIZE temp files...\r"
	zeke new "$RANDOM$RANDOM" > /dev/null 2>&1
done
echo -e "$(tput el)Created $DIR_SIZE temp files.\n"

hyperfine --warmup $WARMUP_COUNT --min-runs $MIN_RUNS \
	"$COMMAND_BACKLINK" \
	"$COMMAND_GRAPH" \
	"$COMMAND_MV \$(ls | head -n 1) \$RANDOM\$RANDOM" \
	"$COMMAND_TAGS"

echo -ne "\nCleaning up temp files...\r"
find . -type f -name "[0-9][0-9][0-9][0-9][0-9][0-9][0-9][0-9]-*.md" -delete
echo -e "$(tput el)Cleaned up temp files."
