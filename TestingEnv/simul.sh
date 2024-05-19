./genfilelist.sh
while read x; do
    vlog -work work -vopt $x
    if [[ $? != 0 ]]; then
        return
    fi
done < ./verible.filelist
vsim -batch test -do run
