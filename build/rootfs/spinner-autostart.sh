#!/bin/bash
#
# SpinnerOS Autostart Script
# Runs applications after the desktop environment is ready

sleep 2

/usr/local/bin/spinner-shell &

if command -v pipewire &> /dev/null; then
    pipewire &
fi

if command -v wireplumber &> /dev/null; then
    sleep 1
    wireplumber &
fi

if [ -d "$XDG_CONFIG_HOME/autostart" ]; then
    for desktop_file in "$XDG_CONFIG_HOME/autostart"/*.desktop; do
        if [ -f "$desktop_file" ]; then
            exec_line=$(grep "^Exec=" "$desktop_file" | head -1 | cut -d= -f2-)
            if [ -n "$exec_line" ]; then
                eval "$exec_line" &
            fi
        fi
    done
fi

wait
