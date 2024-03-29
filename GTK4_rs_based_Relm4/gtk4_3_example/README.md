# eBook

- https://gtk-rs.org/gtk4-rs/stable/latest/book/introduction.html

- gtk docs https://www.gtk.org/docs/
  - https://gtk-rs.org/gtk4-rs/stable/0.8/docs/gtk4/index.html

<hr>

# ```pkg-config```version 맞춰주기


```
$ pkg-config --modversion gtk4

4.6.9

$ cargo add gtk4 --rename gtk --features v4_6

```
<hr>

# Install

- https://askubuntu.com/questions/101306/how-do-i-install-gtk-3-0

- https://stackoverflow.com/questions/68525718/cargo-error-failed-to-run-custom-build-command-for-atk-sys-v0-10-0


- https://gtk-rs.org/gtk4-rs/stable/latest/book/installation_linux.html


# Linux

- You first have to install rustup. You can find the up-to-date instructions on rustup.rs.

- Then install GTK 4 and the build essentials. To do this, execute the command belonging to the distribution you are using.

- Fedora and derivatives:

```
sudo dnf install gtk4-devel gcc
```

- Debian and derivatives:

```
sudo apt install libgtk-4-dev build-essential
```

- Arch and derivatives:

```
sudo pacman -S gtk4 base-devel
```
