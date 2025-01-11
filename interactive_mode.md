# Interactive Mode

- If neither `--query` nor` --source` is specified, **csvs** starts in *interactive mode*.
- While *interactive mode* does not support special SQLite commands (e.g., `.schema`, `.tables`), it allows you to:
  - View a list of imported tables.
  - Quickly preview table contents.
  - Save the result of SQL query to a file.

### Common shortcuts

| Key Combination | Alt. | Description                                |
|-----------------|------|--------------------------------------------|
| CTRL + C        |      | Quit the program.                          |
| CTRL + S        |      | Save the result of SQL query to a file.    |
| Tab / BackTab   |      | Cycle input focus between UI components.   |
| F1              |      | Focus on the "Available Tables" component. |
| F2              |      | Focus on the "SQL Input" component.        |
| F3              |      | Focus on the "SQL Result" component.       |

### Available Tables

| Key Combination | Alt. | Description                                                         |
|-----------------|------|---------------------------------------------------------------------|
| Enter           |      | Select a table and auto-input `SELECT * FROM`.                      |
| CTRL + I        |      | Display the columns of the selected table in the Table Info dialog. |
| Up              |      | Move the selection cursor up.                                       |
| Down            |      | Move the selection cursor down.                                     |
| PageUp          |      | Scroll the selection list up.                                       |
| PageDown        |      | Scroll the selection list down.                                     |
| Home            |      | Jump to the top of the table list.                                  |
| End             |      | Jump to the end of the table list.                                  |

### SQL Input

| Key Combination | Alt.          | Description                                                                                                                                                                                     |
|-----------------|---------------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| CTRL + X        | Shift + Enter | Execute the SQL query.<br>`Shift + Enter` works in terminals supporting [kitty keyboard protocol](https://sw.kovidgoyal.net/kitty/keyboard-protocol/) (macOS/Linux) or in Windows environments. |
| Enter           |               | Insert a new line.                                                                                                                                                                              |
| Up              |               | Move the cursor up.                                                                                                                                                                             |
| Down            |               | Move the cursor down.                                                                                                                                                                           |
| Left            | ALT + B       | Move the cursor left by one character.                                                                                                                                                          |
| Right           | ALT + F       | Move the cursor right by one character.                                                                                                                                                         |
| CTRL + Left     | CTRL + A      | Jump to the start of the current line.                                                                                                                                                          |
| CTRL + Right    | CTRL + E      | Jump to the end of the current line.                                                                                                                                                            |
| ALT + Left      |               | Jump to the previous paragraph.                                                                                                                                                                 |
| ALT + Right     |               | Jump to the next paragraph.                                                                                                                                                                     |
| SHIFT + Left    |               | Jump to the previous word.                                                                                                                                                                      |
| SHIFT + Right   |               | Jump to the next word.                                                                                                                                                                          |
| CTRL + Up       | CTRL + P      | Recall the previous input history.                                                                                                                                                              |
| CTRL + Down     | CTRL + N      | Recall the next input history.                                                                                                                                                                  |
| PageUp          |               | Scroll the text area up.                                                                                                                                                                        |
| PageDown        |               | Scroll the text area down.                                                                                                                                                                      |
| Home            |               | Jump to the top of the text area.                                                                                                                                                               |
| End             |               | Jump to the end of the text area.                                                                                                                                                               |
| Del             | CTRL + D      | Delete the character to the right of the cursor.                                                                                                                                                |
| Backspace       | CTRL + F      | Delete the character to the left of the cursor.                                                                                                                                                 |
| CTRL + Z        |               | Undo the last action.                                                                                                                                                                           |
| CTRL + Y        |               | Redo the last undone action.                                                                                                                                                                    |
| CTRL + V        |               | Paste content from the clipboard.                                                                                                                                                               |

### SQL Result

| Key Combination | Alt.     | Description                    |
|-----------------|----------|--------------------------------|
| Up              | CTRL + P | Move the cursor up.            |
| Down            | CTRL + N | Move the cursor down.          |
| PageUp          |          | Scroll the content up.         |
| PageDown        |          | Scroll the content down.       |
| Home            |          | Jump to the top of the result. |
| End             |          | Jump to the end of the result. |

### Table Info Dialog

| Key Combination | Alt.     | Description                                                 |
|-----------------|----------|-------------------------------------------------------------|
| Esc             |          | Close the dialog                                            |
| Enter           |          | Input the column name at the bottom of SQL Input component. |
| Up              | CTRL + P | Move the cursor up.                                         |
| Down            | CTRL + N | Move the cursor down.                                       |
| PageUp          |          | Scroll the content up.                                      |
| PageDown        |          | Scroll the content down.                                    |
| Home            |          | Jump to the top of the result.                              |
| End             |          | Jump to the end of the result.                              |
