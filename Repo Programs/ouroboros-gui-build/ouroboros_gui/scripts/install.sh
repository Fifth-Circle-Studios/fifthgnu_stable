#!/usr/bin/bash
echo "Running reflector to sort for fastest mirrors" | tee -a /tmp/ouroboros-gui-output.txt
pkexec reflector --latest 5 --sort rate --save /etc/pacman.d/mirrorlist | tee -a /tmp/ouroboros-gui-output.txt
pkexec fifthgnu-ouroboros config ~/.config/ouroboros.json | tee -a /tmp/ouroboros-gui-output.txt
