#! /bin/bash

# Suggestions:

# All Changes to a particular file
# git log --pretty=format:"%H" --follow <FILE> | ../timetravel.sh check test.js

# Last N commits
# git log -n 10 --pretty=format:"%H" | ../timetravel.sh check test.js

rm target/result.md

# get current branch to restore it later
current="$(git rev-parse --abbrev-ref HEAD)"

allruns=()
IFS=$'\n'
set -f

IDX=1
while read i
do
    # change to a specific commit
    git checkout "$i" -q
    git log -1 --pretty=%s

    # check if a compiled version of this commit already exists
    if [ ! -f "target/$i" ]; then
        cargo build -p biome_cli --release 2>&1 > /dev/null
        cp target/release/biome "target/$i"
    fi

    # Print details about this commit
    title=$(git log -1 --pretty=%s)
    echo "# $IDX - $title" >> target/result.md

    echo "## Details" >> target/result.md

    git log -1 --pretty=%b >> target/result.md
    echo "" >> target/result.md
    git log -1 --pretty=%ai >> target/result.md

    # Run this commit version and print the result
    echo "## Result" >> target/result.md
    echo "\`\`\`" >> target/result.md
    eval "target/$i" check $1 &>> target/result.md
    echo "\`\`\`" >> target/result.md
    echo "" >> target/result.md
    echo "" >> target/result.md

    # Save how to run this version to use it
    # later with hyperfine
    allruns+=("target/$i check $1")
    allruns+=("--command-name")
    allruns+=("$IDX")
    ((IDX++))
done

# restore original branch
git checkout "$current" -q

# Run hyperfine and generate a report
echo "# Performance" >> target/result.md
hyperfine ${allruns[*]} -i --shell=none --export-markdown target/hyperfine.md --export-json target/hyperfine.json
cat target/hyperfine.md >> target/result.md
rm target/hyperfine.md 
echo "" >> target/result.md

# Plot hyperfine result
python3 << EOF
#!/usr/bin/env python
import json
import matplotlib.pyplot as plt
f = open("target/hyperfine.json")
results = json.load(f)["results"]
labels=[b["command"] for b in results]
plt.boxplot([b["times"] for b in results])
plt.plot(list(range(1, len(results) + 1)), [b["mean"] for b in results])
plt.savefig("target/whisker.png")
EOF

echo "" >> target/result.md
echo "![benchmark](whisker.png)" >> target/result.md
echo "" >> target/result.md
