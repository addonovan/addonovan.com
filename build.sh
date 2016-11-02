while true
do
	java -Dwebdev.loglevel=1 -jar WebDev.jar  "src/master.yahr"
	date
	inotifywait -e CLOSE_WRITE -r -q src/
	clear
done
