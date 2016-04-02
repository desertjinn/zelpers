#Zelpers
All sorts of helpers

##Nautilus Shortcuts (nautilus-right-click-shortcuts)

###Setup
* Make all files executable using
```Shell
chmod +x <filename>
```

* Run the initial setup script to check if valid folders are present

```Shell
./initialsetup
```

* Move _addshortcut_ and _removeshortcut_ scripts to **/usr/local/bin** so as to be able to access command from anywhere



###Usage :
    
* For adding links to **Agile Boards ->** (created during initial setup)
```Shell
addshortcut -n <name of script> -a <http/https link>
```
 
 For removing links from **Agile Boards ->**
```Shell
removeshortcut -n <name of script> -a
```
    
* For adding any links to **Start ->** (also created during initial setup)
```Shell
addshortcut -n <name of script> -l <http/https link>
```
 For removing links from **Start ->**
```Shell
removeshortcut -n <name of script> -l
```

* For adding any links to **Google Cloud ->** (also created during initial setup)
```Shell
addshortcut -n <name of script> -g <http/https link>
```
 For removing links from **Google Cloud ->**
```Shell
removeshortcut -n <name of script> -g
```
    
* For help
```Shell
addshortcut -h
```
 and
```Shell
removeshortcut -h
```
