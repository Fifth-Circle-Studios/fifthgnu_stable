
# Jade

Jade is an installer backend for fifthgnu linux.

## Backend usage

### Autopartition the drive
```sh
# autopartition /dev/sda with efi enabled
# fifthgnu-ouroboros partition auto /dev/sda --efi

# autopartition /dev/nvmen0 with efi disabled
# fifthgnu-ouroboros partition auto /dev/nvmen0
```

### Install base packages
```sh
# fifthgnu-ouroboros install-base
```

### Install bootloader
```sh
# install as efi with esp being /boot/efi
# fifthgnu-ouroboros bootloader grub-efi /boot/efi

# install as legacy on /dev/sda
# fifthgnu-ouroboros bootloader grub-legacy /dev/sda
```

### Generate fstab
```sh
# fifthgnu-ouroboros genfstab
```

### Configuring locale settings
```sh
# set the keyboard layout to colemak, the timezone to Europe/Berlin and set en_US.UTF-8 as the locale
# fifthgnu-ouroboros locale colemak Europe/Berlin en_US.UTF-8 UTF-8
```

### Configure network settings
```sh
# set the hostname to getcryst.al with ipv6 disabled
# fifthgnu-ouroboros networking getcryst.al 

# set the hostname to getcryst.al with ipv6 enabled
# fifthgnu-ouroboros networking getcryst.al --ipv6
```

### Setup zramd
```sh
# install and enable zramd
# fifthgnu-ouroboros zramd
```

### Configure users
```sh
# make a new user called nonRootHaver, without sudo, easytohack as the password and bash as the default shell
# fifthgnu-ouroboros users new-user nonRootHaver easytohack bash

# make a user called rootHaver, with sudo, omgsosuperhardtohack as the password and fish as the default shell
# fifthgnu-ouroboros users new-user rootHaver omgsuperhardtohack fish --hasroot
```

### Set root password
```sh
# set the root password to 'muchSecurity,veryHardToHack'
# fifthgnu-ouroboros users root-password muchSecurity,veryHardToHack
```

### Install a desktop environment
```sh
# install onyx
# fifthgnu-ouroboros desktops onyx

# install gnome
# fifthgnu-ouroboros desktops gnome
```

### Setup timeshift
```sh
# fifthgnu-ouroboros setup-timeshift
```

### Setup flatpak
```sh
# fifthgnu-ouroboros flatpak
```

### Debug logging

debug messages:
```sh
# fifthgnu-ouroboros -v
```

traces:
```sh
# fifthgnu-ouroboros -vv
```


## Non-secret Secret
$ echo "fifthgnu-ouroboros_UWU=true" >> ~/.zshrc 

$ echo "fifthgnu-ouroboros_UWU=true" >> ~/.bashrc 

$ set -Ux fifthgnu-ouroboros_UWU true 


if you want to have your log and crash output be “cute”

## 🙌 Contributing

If you'd like to contribute to **Jade**, please follow the [Crystal Linux contributing guidelines](https://git.getcryst.al/fifthgnu/info/-/blob/main/CONTRIBUTING.md)!

This project uses `rustup`, to set up `cargo` for **Jade** development, please follow the guidelines below:


#### Debug/development builds

- `cargo build`

#### Optimised/release builds

- `cargo build --release`



## 📜 License

[GPLv3-only](https://choosealicense.com/licenses/gpl-3.0/)

![](https://git.getcryst.al/fifthgnu/misc/branding/-/raw/main/banners/README-banner.png)
