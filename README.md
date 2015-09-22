# Zelpers
All sorts of helpers

## Nautilus Shortcuts
(nautilus-right-click-shortcuts)


    - Setup

        * Make all files executable using
          ```
          chmod +x <filename>
          ```
        * Run the initial setup script to check if valid folders are present
          ```
          ./initialsetup
          ```
        * Move _addshortcut_ and _removeshortcut_ scripts to **/usr/local/bin** so as to be able to access command from anywhere

    - Usage :
        * For adding links to **Agile Boards ->** (created during initial setup)
        ```
        addshortcut -n <name of script> -a <http/https link>
        ```

        * For adding any links to **Start ->** (also created during initial setup)
        ```
        addshortcut -n <name of script> -l <http/https link>
        ```

        * For removing links from **Agile Boards ->**
        ```
        removeshortcut -n <name of script> -a
        ```

        * For removing links from **Start ->**
        ```
        removeshortcut -n <name of script> -l
        ```

        * For help
        ```
        addshortcut -h
        ```
        and
        ```
        removeshortcut -h
        ```
