# conswitch

A script to conveniently switch between several configs.

## concept of conswitch

**This is currently a draft and may change in the future.**

Imagine having to deal with two versions of a config file. For example using the same device, but sometimes you have a 1080p screen connected to it, other times a 1440p screen.

If you want to switch between your screen setups, you would have to manually edit your config file and change the settings between 1080p/1440p. *conswitch* should add a convenient solution for it.

Upon adding a config file (e.g. `conswitch --add /path/to/config.file`), it creates a `.conswitch` folder in the same location and moves the mentioned config file into this new folder (e.g. `config.file.default`). Then it symlinks this default config to the file's original location. For now, nothing would have changed.

With *conswitch* it is possible to create profiles (e.g. `conswitch --new profile1440p`), which will copy the default config file to a `config.file.profile1440p`.

Now the user is able to switch the profile (e.g. `conswitch --to profile1440p`), which will then remove the given symlink of any config file, but then symlinks the appropriate config of the mentioned profile to the original location. *(Hint: If the user hasn't changed a specific config he added before, the default one would be used.)*

This way a user is able to create profiles for whatever he needs, and with a simple command of *conswitch* the user is able to change any added configs to a desired profile.

Usages could be:

- home / work configs
- minimal / maximum settings for gaming setups
- debug / development / production environments

This software has been developed for my gaming purposes (switching configs depending if I game on my small or bigger screen). But as you can see, it could be used for any other case where switching between configs might be necessary.
