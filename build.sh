while true
do
	java -jar WebDev.jar -Dwebdev.loglevel=1 "src/master.yahr"
	date
	inotifywait -e CLOSE_WRITE -r -q src/
	clear
done