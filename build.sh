while true
do
	java -jar WebDev.jar "src/master.yahr"
	date
	inotifywait -e CLOSE_WRITE -r -q src/
	clear
done