#!/bin/bash
set -e

# logo
echo "=========================================================================="
echo "  ______             _____           _     _____                      "
echo " |___  /            / ____|         | |   / ____|                     "
echo "    / / ___ _ __ ___| |     ___  ___| |_ | |      ____ __ _ _ __  ___ "
echo "   / / / _ \ '__/ _ \ |    / _ \/ __| __|| |_ |/ / _' | '_ \` _ \/ _ \\"
echo "  / /_|  __/ | | (_) | |___| (_) \__ \ |_| |__| | (_| | | | | | |  __/"
echo " /_____\___|_|  \___/ \_____\___/|___/\__|\_____|\__,_|_| |_| |_|\___|"
echo "                                                                        "
echo "                         S E T U P   M O D E                          "
echo "=========================================================================="                
                                                                                                                     

cd "$(dirname "$0")" || exit 1

# Security Verification
if [ ! -f "Cargo.toml" ]; then
    echo "Can't locate project's root."
    exit 1
fi

# Compiling the project
cargo build --release --package oracle

# Player_directory archive
mkdir game_directory

mkdir game_directory/challenges
cp -r challenges game_directory/
cp -r player_advices game_directory
cp target/release/oracle game_directory

# Compressing
tar -czf game_directory.tar.gz game_directory

# Clean up
rm -rf game_directory
