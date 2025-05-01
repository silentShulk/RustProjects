HOW TO USE:
1. Add the executable file (DirectoryChangesLog/src/dcl) in your PATH

  Linux:
    1) Copy the repository
    2) Choose which path to put the executable in, I have a custom directory added to my PATH where I put executables I download from the internet
    3) Execute the following command: cp ~/Path/To/DirectoryChangesLog/target/release/dcl ~/Path/To/PATH
    4) Add the directory you decided to put the executable in to your PATH (not necessary if you've added it to the default PATH of your system)
    5) Run "dcl -v" or "dcl --version" to check if it works

  Windows:
    Create a directory to store any executable you'll ever need to add to your PATH:
    https://windowsloop.com/how-to-add-to-windows-path/

2. Open a terminal window
3. Type:  dcl <path to an existing file or folder> <path to the folder in which you want to put the file or folder> <path to desired log file location>
   The path to the log file must include the name of the log file (*.txt)
   Example:  dcl ~/sigma.txt ~/Ohio ~/Ohio/skibidi.txt
4. Run "dcl -h" or "dcl --version" to see all the flags avaiable 