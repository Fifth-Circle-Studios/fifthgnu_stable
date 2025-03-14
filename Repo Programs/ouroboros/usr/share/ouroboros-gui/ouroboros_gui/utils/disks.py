# exec.py
#
# Copyright 2022 axtlos
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with this program.  If not, see <http://www.gnu.org/licenses/>.
#
# SPDX-License-Identifier: GPL-3.0-only

import subprocess
import shutil
from ouroboros_gui.utils.command import CommandUtils


bash_bin = shutil.which("bash")


def get_disks():
    output = CommandUtils.check_output(["lsblk", "-pdo", "name"])
    output = output.split()
    output = [x for x in output if "zram" not in x]
    output = [x for x in output if "NAME" not in x]
    output = [x for x in output if "loop" not in x]
    output = [x for x in output if "sr" not in x]
    output = [x for x in output if "fd" not in x]
    return output


def get_disk_size(disk: str) -> str:
    output = CommandUtils.check_output(["lsblk", "-pdbo", "SIZE", disk])
    output = output.split()
    output = [x for x in output if "SIZE" not in x]

    if len(output) == 0:
        print(f"No disk found with name: {disk}, assuming zero.")
        size = 0
    else:
        size = int(output[0])

    print(disk + ":" + str(size))
    if size < 1000**3:
        size = size / 1000**2
        return f"{size: .2f} MB"
    elif size < 1000**4:
        size = size / 1000**3
        return f"{size: .2f} GB"
    else:
        size = size / 1000**4
        return f"{size: .2f} TB"


def get_uefi() -> bool:
    command = subprocess.run(
        ["bash", "-c", "[ -d /sys/firmware/efi ] && echo UEFI || echo BIOS"],
        capture_output=True,
    )
    isEfi = True if command.stdout.decode("utf-8").strip("\n") == "UEFI" else False
    return isEfi


def get_disk_type(disk: str):
    output = CommandUtils.check_output(["lsblk", "-d", "-o", "rota", disk])
    output = output.split()
    output = [x for x in output if "ROTA" not in x]

    if len(output) > 0:
        if output[0] == "0":
            return "Solid-State Drive (SSD)"
        elif output[0] == "1":
            return "Hard Disk (HDD)"  # maybe Rotational Drive?

    print(f"No disk found with name: {disk}, assuming unknown.")
    return "Drive type unknown"


def get_partitions():
    output = CommandUtils.check_output(["blkid", "-o", "device"])
    output = output.split()
    output = [x for x in output if "zram" not in x]
    output = [x for x in output if "loop" not in x]
    output = [x for x in output if "sr" not in x]
    return output
