HOW TO USE:
1. Add the executable file (DirectoryChangesLog/src/dcl) in your PATH
    1) Copy the repository
    2) Choose which path to put the executable in, I have a custom directory added to my PATH where I put executables I download from the internet
    3) execute the following command: cp ~/Path/To/DirectoryChangesLog/src/dcl ~/Path/To/PATH
    4) Add the directory you decided to put the executable in to your PATH (not necessary if you've added it to the default PATH of your system)
    5) Run dcl -v or dcl --version ro check if it works

2. Open a terminal window
3. Type:  dcl <path to an existing file or folder> <path to the folder in which you want to put the file> <path to desired log file location>
   The path to the log file must include the name of the log file (must be .txt)
   Example:  dcl ~/sigma.txt ~/Ohio ~/Ohio/skibidi.txt