Output assets/usage.gif
Set CursorBlink false
Set FontSize 16
Set Framerate 20
Set Height 600
Set Margin 0
Set PlaybackSpeed 1.0
Set Shell zsh
Set Theme "iTerm2 Default"
Set TypingSpeed 60ms
Set Width 1200

Type "csvs -i MOCK_DATA.csv -q '"
Type 'SELECT * FROM "MOCK_DATA.csv" WHERE id < 10;'
Type "'"
Sleep 1s
Enter
Sleep 4s

Type "csvs -i MOCK_DATA.csv -q '"
Type 'SELECT id,first_name,job_title FROM "MOCK_DATA.csv" WHERE first_name LIKE "Ros%";'
Type "'"
Sleep 1s
Enter
Sleep 4s
Screenshot assets/usage.banner.png
