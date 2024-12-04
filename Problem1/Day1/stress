#/bin/bash

t=$4

for(( i=1; i<=t; i++)) 
do
    in=$(mktemp)
    ./$3 > $in
    f=$(mktemp)
    ./$1 < $in > $f
    s=$(mktemp)
    ./$2 < $in > $s

    mf=$(md5sum "$f" | cut -d' ' -f1)
    ms=$(md5sum "$s" | cut -d' ' -f1)

    if [ "$mf" != "$ms" ]; then
        cat $in
        echo "---"
        cat $f
        echo "---"
        cat $s
        break
    fi
    echo $i " done"
done
