## spotifyd installation Ubuntu Linux 22.04

First of all, we need to install service which can play music from Spotify.

I'll use Spotifyd.

It's relatively easy to build as it uses Rust:

    sudo apt install libasound2-dev libssl-dev pkg-config cargo
    git clone https://github.com/Spotifyd/spotifyd.git
    cd spotifyd
    cargo build --release

Then you need to create basic configuration for it which includes login and plain text password. Create configuration folder:

    mkdir ~/.config/spotifyd

Then open file with favourite editor:

    vim ~/.config/spotifyd/spotifyd.conf

And then add following:

    [global]
    # Your Spotify account name.
    username = "xxx@gmail.com"

    # Your Spotify account password.
    password = "xxx"

And finally launch daemon:

    ~/spotifyd/target/release/spotifyd --no-daemon

Then you can see following log messages when you try to play music:

    Loading config from "/home/xxx/.config/spotifyd/spotifyd.conf"

    No proxy specified

    Using software volume controller.

    Connecting to AP "ap.spotify.com:443"

    Authenticated as "xxx" !

    Using Alsa sink with format: S16

    Country: "GB"

    Loading <Damascus> with Spotify URI <spotify:track:xxx>

    <Damascus> (122880 ms) loaded

For production use I can recommend installing it to /opt:

    sudo cp  ~/spotifyd/target/release/spotifyd  /opt/spotifyd

Then you will need to copy configuration file into system configuration path:

    sudo cp ~/.config/spotifyd/spotifyd.conf /etc

And creating systemd unit for it:

    sudo vim /lib/systemd/system/spotifyd.service

With following content:

    [Unit]

    Description=A spotify playing daemon

    Documentation=https://github.com/Spotifyd/spotifyd

    Wants=sound.target

    After=sound.target

    Wants=network-online.target

    After=network-online.target

    [Service]

    ExecStart=/opt/spotifyd --no-daemon

    Restart=always

    RestartSec=12

    [Install]

    WantedBy=default.target

And finally enable start on boot and start Spotifyd daemon:

    sudo systemctl daemon-reload
    sudo systemctl enable spotifyd
    sudo systemctl start spotifyd

After that I can recommend checking that daemon started successfully using this command:

    sudo systemctl status spotifyd

Example output:

    spotifyd.service - A spotify playing daemon

         Loaded: loaded (/lib/systemd/system/spotifyd.service; enabled; preset: enabled)

         Active: active (running) since Wed 2023-01-18 14:13:11 GMT; 3s ago

           Docs: https://github.com/Spotifyd/spotifyd

       Main PID: 8963 (spotifyd)

          Tasks: 8 (limit: 4513)

         Memory: 976.0K

            CPU: 30ms

         CGroup: /system.slice/spotifyd.service

                 └─8963 /opt/spotifyd --no-daemon


    Jan 18 14:13:11 rockpro64 systemd[1]: Started A spotify playing daemon.

    Jan 18 14:13:11 rockpro64 spotifyd[8963]: Loading config from "/etc/spotifyd.conf"

    Jan 18 14:13:11 rockpro64 spotifyd[8963]: No proxy specified

    Jan 18 14:13:11 rockpro64 spotifyd[8963]: Using software volume controller.

    Jan 18 14:13:11 rockpro64 spotifyd[8963]: Connecting to AP "ap.spotify.com:443"

    Jan 18 14:13:11 rockpro64 spotifyd[8963]: Authenticated as "xxx" !

    Jan 18 14:13:11 rockpro64 spotifyd[8963]: Country: "GB"

    Jan 18 14:13:11 rockpro64 spotifyd[8963]: Using Alsa sink with format: S16

After that you can install Spotify console client. If you see any errors from client then you will need to click "d" and select spotifyd as output device.

Source: https://www.stableit.blog/2023/01/spotifyd-installation-ubuntu-linux-2204.html
