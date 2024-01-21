# Syncit: an easy way to share and save projects

Syncit is a folder sharing system i built myself, written completely in rust. It is organized in **drawers** (simple virtualized folders). It's made for beginners and people who don't need something as complicated as github to work for a project with someone else, or maybe for people who want an easy way to share projects between computers.

### CLI base commands:

> `syncit send <path>`

Creates or uploads the folder contents to a drawer named as the last subfolder in the given path (or the current folder name if no path is given). \
*E.g*: `syncit send ../closet` will load the content in the folder at `../closet` in a drawer named `closet`.

> `syncit get <path>`

Gets the content of a drawer and puts it in the path given location. The drawer name will be the last subfolder name in the given path (or the current folder name if no path is given).\
*E.g.*: `syncit get` executed in `~/shelf/` will get the contents from `shelf` drawer and put it in current folder.

### Config file

This system has his own all-in-one config file: `.sync`. The current version supports files to ignore (`@ignores [<files>]`), debug mode for extended output in console (`@debug`), and location path (`@location <path>`, currently unused).
For example, with this config file:
```
@ignores
box
jar
.trash

@debug

@location bob:closet
```
the `send` command won't add the `box` and `pot` folders in the drawer, together with the `.trash` file. The CLI will also enable extended debug outputs and set the drawer location to `bob:closet` (explained in todos).
The default config file will contain ignores for `.git` folder and `.gitignore` file.

### TODOs (in order of importance):
- Account and permissions system, to share and crypt files, and to give permissions to friends to edit your drawer (probably with `perm` command).
- A `way` command to select drawer name and owner user and set it to the config file, if i don't want to use the current folder as drawer name. (E.g. if I have the permissions to edit it, `syncit way bob:closet` will set the project location to *bob*'s drawer named *closet*)
- A GUI (maybe in tauri) for a better managment of permissions, drawers, and for file listing without downloading drawers.
- An optional commit system: Every user sharing a drawer will have his own drawer version, and the `send` command will update my own version of the drawer. Only on `commit` the changes will be made visible to the shared version.
- A branch system, maybe with `syncit branch <name>` accessible with `syncit switch <name>`.
