# Ouroboros GUI

The libadwaita/gtk based gui installer using ouroboros as the backend.

## 💡 Features

- Beatiful UI
- Easy to use
- Uses accessible technologies

## 🫂 Support

**Support** is available in our [Discord](https://getcryst.al/discord) and the [Matrix](https://matrix.to/#/#space:getcryst.al). If you face any issues with the software, feel free to open an issue on this repository.


## 👥 Contributors

A list of all **Ouroboros GUI** contributors is available in [CONTRIBUTORS.md](CONTRIBUTORS.md)

For a list of **Ouroboros GUI** maintainers specifically, see [.gitlab/CODEOWNERS](.gitlab/CODEOWNERS)


## 💾 Installation
### 🏗 From Source

**Install dependencies**

```bash
 # pacman -S ouroboros openssl python-pytz gparted vte4 meson ninja libadwaita desktop-file-utils appstream-glib gtk4
 ```

**Install ouroboros-GUI from source using `meson` and `ninja`**
```bash
 $ git clone https://github.com/crystal-linux/ouroboros-gui
 $ cd ouroboros-gui
 $ meson --prefix=/usr _build
 $ ninja -C _build
 $ cd _build
 $ sudo ninja install
```

**Install ouroboros-GUI as a flatpak using `flatpak-builder`** (Recommended for development)
```sh
 $ git clone https://github.com/crystal-linux/ouroboros-gui
 $ cd ouroboros-gui
 $ flatpak-builder --user --install --install-deps-from=flathub --force-clean build-dir al.fifthgnu.ouroborosgui.yml
 $ flatpak run al.fifthgnu.ouroborosgui
```
## 📸 Screenshots

![App Screenshot](main-page-screenshot.png)


## 🙌 Contributing

If you'd like to contribute to **Ouroboros GUI**, please follow the [Fifth Circle GNU contributing guidelines](https://git.getcryst.al/crystal/info/-/blob/main/CONTRIBUTING.md)!

This project uses `meson`, and `ninja` for **Ouroboros GUI** development, please follow the guidelines below:

https://pypi.org/project/ninja/

https://mesonbuild.com/

## 📚 Documentation
We are also constantly looking for translators for our i18n-enabled projects! If you speak more than one language, consider helping out on our [Weblate](https://i18n.getcryst.al)!

![https://i18n.getcryst.al/engage/crystal-linux/](https://i18n.getcryst.al/widgets/crystal-linux/-/287x66-black.png)


## 📜 License

[GPLv3-only](https://choosealicense.com/licenses/gpl-3.0/)

![](https://git.getcryst.al/crystal/misc/branding/-/raw/main/banners/README-banner.png)
