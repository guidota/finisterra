!#bin/sh

echo 'Exporting human body...'
aseprite -b -script-param output=../assets/finisterra/bodies/human/ -script-param filename=bodies/human.aseprite -script scripts/export-body.lua

echo 'Exporting drow body...'
aseprite -b -script-param output=../assets/finisterra/bodies/drow/ -script-param filename=bodies/drow.aseprite -script scripts/export-body.lua

echo 'Exporting dwarf body...'
aseprite -b -script-param output=../assets/finisterra/bodies/dwarf/ -script-param filename=bodies/dwarf.aseprite -script scripts/export-body.lua

echo 'Exporting elf body...'
aseprite -b -script-param output=../assets/finisterra/bodies/elf/ -script-param filename=bodies/elf.aseprite -script scripts/export-body.lua

echo 'Exporting weapons...'
aseprite -b -script-param output=../assets/finisterra/weapons/ -script-param filename=weapons.aseprite -script scripts/export-weapon.lua

echo 'Exporting shields...'
aseprite -b -script-param output=../assets/finisterra/shields/ -script-param filename=shields.aseprite -script scripts/export-shield.lua

echo 'Exporting helmets...'
aseprite -b -script-param output=../assets/finisterra/helmets/ -script-param filename=helmets.aseprite -script scripts/export-helmet.lua

echo 'Exporting clothing...'
aseprite -b -script-param output=../assets/finisterra/clothing/ -script-param filename=clothing-tall.aseprite -script scripts/export-clothing.lua

echo 'Exporting heads...'
aseprite -b -script-param output=../assets/finisterra/heads/ -script-param filename=heads.aseprite -script scripts/export-heads.lua

