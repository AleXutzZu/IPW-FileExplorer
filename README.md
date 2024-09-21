<h1>File Explorer</h1>

This is a simple app meant to browse folders and files on the filesystem. It was created during the Tauri
Workshop I have attended at IPWorkshop 2023.

<h2>How it works</h2>
All the interactions with the filesystem are done through Rust with the use of commands that are called from
the front-end. The Vue app actually has 2 components that do all the work: a Folder and A File component.
The Folder component is defined recursively: spawning more folder components as paths are discovered.

Folders can be opened and closed, whilst clicking on a file opens a very crude editor and the option to
delete the file itself.

To keep the file structure updated, File components are actually passed a delete function from the parent
folder (there is a function factory defined). Whenever this custom delete is called, all paths in the parent
folder are updated and the component is re-rendered.

<h2>Key features</h2>

- Access any folder and its subfolders on the filesystem
- Create, delete and edit any file
- Simple and intuitive GUI

<h2>Some screenshots</h2>

<h2>Future improvements</h2>

- Optimise updates to the file structure (Possibly by not reiterating through all paths)
- Include the option to delete folders and all content inside them and integrate this into the UI
- Standardize the creation of files/folders (Currently every opened folder renders a form to add entries,
and it can get cluttered, a general form with the possibility of selecting the destination would be better suited)
- Instead of pasting the path to the root folder, have a native dialog for selecting folders (**as of right now, this
is not possible in Tauri**)
