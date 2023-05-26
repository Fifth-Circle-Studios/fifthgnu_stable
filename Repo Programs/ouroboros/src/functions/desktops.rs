use crate::args::DesktopSetup;
use crate::internal::exec::*;
use crate::internal::*;

pub fn install_desktop_setup(desktop_setup: DesktopSetup) {
    log::debug!("Installing {:?}", desktop_setup);
    match desktop_setup {
        // DesktopSetup::Abyss => install_abyss(),
        DesktopSetup::Gnome => install_gnome(),
        DesktopSetup::Kde => install_kde(),
        DesktopSetup::Budgie => install_budgie(),
        DesktopSetup::Cinnamon => install_cinnamon(),
        DesktopSetup::Cutefish => install_cutefish(),
        DesktopSetup::Xfce => install_xfce(),
        DesktopSetup::Hyprland => install_hyprland(),
    }
    install_networkmanager();
}

fn install_networkmanager() {
    install(vec!["networkmanager"]);
    exec_eval(
        exec_chroot(
            "systemctl",
            vec![String::from("enable"), String::from("NetworkManager")],
        ),
        "Enable network manager",
    );
}

fn install_hyprland() {
    install(vec![
        "xorg",
        "hyprland",
        "pipewire",
        "pipewire-pulse",
        "pipewire-alsa",
        "pipewire-jack",
        "wireplumber",
        "lightdm",
        "lightdm-gtk-greeter",
        "lightdm-gtk-greeter-settings",
    ]);
    files_eval(
        files::append_file(
            "/mnt/etc/lightdm/lightdm.conf",
            "[SeatDefaults]\ngreeter-session=lightdm-gtk-greeter\n",
        ),
        "Add lightdm greeter",
    );
    enable_dm("lightdm");
}

fn install_xfce() {
    install(vec![
        "xorg",
        "xfce4",
        "lightdm",
        "lightdm-gtk-greeter",
        "lightdm-gtk-greeter-settings",
        "xfce4-goodies",
        "pipewire",
        "pipewire-pulse",
        "pipewire-jack",
        "pipewire-alsa",
        "wireplumber",
        "pavucontrol",
    ]);
    files_eval(
        files::append_file(
            "/mnt/etc/lightdm/lightdm.conf",
            "[SeatDefaults]\ngreeter-session=lightdm-gtk-greeter\n",
        ),
        "Add lightdm greeter",
    );
    enable_dm("lightdm");
}

fn install_cutefish() {
    install(vec![
        "xorg",
        "cutefish",
        "pipewire",
        "pipewire-pulse",
        "pipewire-alsa",
        "pipewire-jack",
        "wireplumber",
        "sddm",
        ]);
        enable_dm("sddm");
    }

fn install_cinnamon() {
    install(vec![
        "xorg",
        "cinnamon",
        "pipewire",
        "pipewire-pulse",
        "pipewire-alsa",
        "pipewire-jack",
        "wireplumber",
        "lightdm",
        "lightdm-gtk-greeter",
        "lightdm-gtk-greeter-settings",
        "metacity",
        "gnome-shell",
        "gnome-terminal",
    ]);
    files_eval(
        files::append_file(
            "/mnt/etc/lightdm/lightdm.conf",
            "[SeatDefaults]\ngreeter-session=lightdm-gtk-greeter\n",
        ),
        "Add lightdm greeter",
    );
    enable_dm("lightdm");
}

fn install_budgie() {
    install(vec![
        "xorg",
        "budgie-desktop",
        "gnome",
        "pipewire",
        "pipewire-pulse",
        "pipewire-alsa",
        "pipewire-jack",
        "wireplumber",
        "lightdm",
        "lightdm-gtk-greeter",
        "lightdm-gtk-greeter-settings",
        "xdg-desktop-portal",
        "xdg-desktop-portal-gtk",
        "xdg-utils",
    ]);
    files_eval(
        files::append_file(
            "/mnt/etc/lightdm/lightdm.conf",
            "[SeatDefaults]\ngreeter-session=lightdm-gtk-greeter\n",
        ),
        "Add lightdm greeter",
    );
    enable_dm("lightdm");
}

fn install_kde() {
    install(vec![
        "xorg",
        "plasma",
        "plasma-wayland-session",
        "kde-utilities",
        "kde-system",
        "pipewire",
        "pipewire-pulse",
        "pipewire-alsa",
        "pipewire-jack",
        "wireplumber",
        "sddm",
    ]);
    enable_dm("sddm");
}
fn install_gnome() {
    install(vec![
        "xorg",
        "gnome",
        "sushi",
        "pipewire",
        "pipewire-pulse",
        "pipewire-alsa",
        "pipewire-jack",
        "wireplumber",
        "xdg-desktop-portal",
        "xdg-desktop-portal-gtk",
        "xdg-utils",
        "gdm",
    ]);
    enable_dm("gdm");
}

fn enable_dm(dm: &str) {
    log::debug!("Enabling {}", dm);
    exec_eval(
        exec_chroot("systemctl", vec![String::from("enable"), String::from(dm)]),
        format!("Enable {}", dm).as_str(),
    );
}
