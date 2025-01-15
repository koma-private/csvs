Output assets/interactive.gif
Set CursorBlink false
Set FontSize 16
Set Framerate 20
Set Height 600
Set Margin 0
Set PlaybackSpeed 1.0
Set Shell zsh
Set Theme "iTerm2 Default"
Set TypingSpeed 80ms
Set Width 1200

Type "csvs -i MOCK_DATA.csv"
Sleep 500ms
Enter
Sleep 2s
Ctrl+T
Sleep 500ms
Down 2
Screenshot assets/interactive.banner2.png
Sleep 1s
Escape
Sleep 1s
Enter
Sleep 500ms
Tab
Sleep 500ms
Ctrl+X
Sleep 2s
Tab
Sleep 500ms
Down 4
Sleep 500ms
Tab
Sleep 500ms
Tab
Sleep 500ms
Right 8
Backspace
Type "first_name, last_name, job_title"
Ctrl+E
Backspace
Space
Enter
Type "WHERE REGEXF ('eng.[ner]+', job_title, 'iu');"
Sleep 500ms
Ctrl+X
Sleep 2s
Tab
Sleep 500ms
Down 3
Screenshot assets/interactive.banner1.png
Sleep 500ms
Ctrl+S
Sleep 2s
Type "output.csv"
Sleep 500ms
Enter
Sleep 2s
Enter
Sleep 500ms
Ctrl+C
Sleep 1s
