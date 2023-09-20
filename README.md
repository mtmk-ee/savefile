# SaveFile

Command line tool for automatically backing up files and directories when they change.

## Overview

The tool requires a profile to be created before backups can be created. The profile specifies the following:

- The base directory to watch for changes.
- Which files or directories to include in each backup.
- How long to wait after a change before creating a backup.

Once a profile is created, it can be used to create backups. The tool will watch the base directory for changes and create a backup when a change is detected. The backup will contain the files and directories specified in the profile. Backups may be reviewed and restored using the tool.

## Usage

### Creating a Profile

To create a profile:

```bash
savefile profile create --name INSERT_NAME [--edit]
```

This will create a new profile with the specified name. If the `--edit` flag is specified, the profile JSON file will be opened in the default editor.

Make sure to edit this file before trying to create any backups.

### Listing Profiles

To list all profiles:

```bash
savefile profile list [--prefix INSERT_PREFIX]
```

This will list all profiles. If the `--prefix` flag is specified, only profiles with names that start with the specified prefix will be listed.

### Removing a Profile

To remove a profile:

```bash
savefile profile delete --name INSERT_NAME
```

This will remove the profile with the specified name. WARNING: This will also remove all backups created with this profile.


### Creating a Backup

To manually create a backup:

```bash
savefile backup create --name INSERT_NAME
```

### Listing Backups

To list all backups:

```bash
savefile backup list --name INSERT_NAME [--count INSERT_COUNT]
```

This will display a table of all backups created with the specified profile. If the `--count` flag is specified, only the specified number of backups will be listed.

### Restoring a Backup

To restore a backup:

```bash
savefile backup restore --name INSERT_NAME --id INSERT_ID
```

This will restore the specified backup. The backup ID can be found by listing the backups.

WARNING: This will overwrite any files or directories that were included in the backup, and may result in data loss if the profile is not configured correctly.

### Removing a Backup

To remove a backup:

```bash
savefile backup delete --name INSERT_NAME [--id INSERT_ID]
```

This will remove the specified backup. If the `--id` flag is not specified, all backups for the specified profile will be removed.

You can also choose to only keep the latest backups:

```bash
savefile backup retain --name INSERT_NAME --count INSERT_COUNT
```

This will remove all backups except for the specified number of latest backups.

### Watching for Changes

To watch for changes and create backups automatically:

```bash
savefile watch --name INSERT_NAME
```
