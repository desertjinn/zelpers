# Zappers
All sorts of helpers

## Nautilus Shortcuts
(nautilus-right-click-shortcuts)


    - Setup

        - Make all files executable using

          ```shell
          chmod +x <filename>
          ```
        - Run the initial setup script to check if valid folders are present

          ```shell
          ./initialsetup
          ```
        - Move _addshortcut_ and _removeshortcut_ scripts to **/usr/local/bin** so as to be able to access command from anywhere

    - Usage :
        - For adding links to **Agile Boards ->** (created during initial setup)

        ```shell
        addshortcut -n <name of script> -a <http/https link>
        ```

        - For adding any links to **Start ->** (also created during initial setup)

        ```shell
        addshortcut -n <name of script> -l <http/https link>
        ```

        - For removing links from **Agile Boards ->**

        ```shell
        removeshortcut -n <name of script> -a
        ```

        - For removing links from **Start ->**

        ```shell
        removeshortcut -n <name of script> -l
        ```

        - For help

        ```shell
        addshortcut -h
        ```
        and

        ```shell
        removeshortcut -h
        ```
