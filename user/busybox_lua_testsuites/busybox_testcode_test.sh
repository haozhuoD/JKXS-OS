#!/bin/bash

# RST=result.txt
# if [ -f $RST ];then
# 	rm $RST
# fi
# touch $RST

# echo "If the CMD runs incorrectly, return value will put in $RST" > $RST
# echo -e "Else nothing will put in $RST\n" >> $RST
# echo "TEST START" >> $RST

# cat ./busybox_cmd.txt | while read line
# do
# 	eval "./busybox $line"
# 	RTN=$?
# 	if [[ $RTN -ne 0 && $line != "false" ]] ;then
# 		echo "testcase busybox $line fail"
# 		echo "return: $RTN, cmd: $line" >> $RST
# 	else
# 		echo "testcase busybox $line success"
# 	fi
# done

# echo "TEST END" >> $RST

RST=result.txt

echo "If the CMD runs incorrectly, return value will put in $RST"
echo -e "Else nothing will put in $RST\n"
echo "TEST START"

# busybox cat ./busybox_cmd.txt
# busybox echo "hello" | while read line
while true
do
	eval "./busybox ls"
	# RTN=$?
	# if [[ $RTN -ne 0 && $line != "false" ]] ;then
	# 	echo "testcase busybox $line fail"
	# 	echo "return: $RTN, cmd: $line"
	# else
	# 	echo "testcase busybox $line success"
	# fi
done

echo "TEST END"