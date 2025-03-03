COMMANDS=('-r' '-m' '-e')

cargo build --release

for file in $(ls ./examples/invalid/*);
do
	printf "%-40s " $file
	RESULT=$(./target/release/n-puzzle $command $file > /dev/null 2> /dev/null)
	if [ $? -eq 1 ]
	then
		echo "\t[OK]"
	else
		echo "\t[ERR]"
	fi
done

for file in $(ls ./examples/unsolvable/*);
do
	printf "%-40s " $file
	RESULT=$(./target/release/n-puzzle $command --dry $file > /dev/null 2> /dev/null)
	if [ $? -eq 1 ]
	then
		echo "\t[OK]"
	else
		echo "\t[ERR]"
	fi
done

for command in $COMMANDS;
do
	for file in $(ls ./examples/solvable/*);
	do
		dry=""

		if [[ $file = *5* ]]
		then
			continue
		fi

		if [[ $file = *4* && $command == "-e" ]]
		then
			continue
		fi

		if [[ $file = */*solvable-*.txt ]]
		then
			dry="--dry"
			printf "--dry "
		else
			printf "      "
		fi

		printf "%s | %-40s " $command $file
		RESULT=$(./target/release/n-puzzle $command $dry $file > /dev/null 2> /dev/null)

		if [ $? -eq 0 ]
		then
			echo "\t[OK]"
		else
			echo "\t[ERR]"
		fi
	done
done