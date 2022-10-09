#!/bin/env sh

model="tiny"
language="English"

while true; do 
	files="$(ls | grep "_processing_ | sort -r ")"
	if [[ $files ]]; then
		oldestfile="$(echo "$files" | head -1)"
		outfilename="$( echo "${oldestfile}" | sed 's/_processing_([0-9]*)/transcribed_\1/g' )"
		ffmpeg -y -f s16le -ar 96k -ac 1 -i ${oldestfile} "tmp.wav"
		whisper "tmp.wav" --language ${language} --model ${model} >> ${outfilename}
		rm ${oldestfile} 
	else
		sleep 5
	fi
done