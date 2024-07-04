#! /usr/bin/bash

# This script is used to build the presentation slides for the project

# Run the nix-shell to trigger env setup to trigger build
# nix-shell



# Cleanup previous builds
DOCS_DIR=$(pwd)
echo Cleaning up previous builds at "$DOCS_DIR"...
if [[ ! -d $DOCS_DIR ]]; then
    mkdir docs
else
    rm -f "$DOCS_DIR"/*.html
    rm -f "$DOCS_DIR"/*.pdf
fi

echo Cleaning up the presentation html for github pages...
PAGES_DIR=$DOCS_DIR/pages
if [[ ! -d $PAGES_DIR ]]; then
    mkdir pages
else
    rm -f "$PAGES_DIR"/*.html
fi

# Building the html and pdf decks with marp/node
PRESENTATION_FILE=$DOCS_DIR/project-presentation.md
if [[ ! -f $PRESENTATION_FILE ]]; then
    echo no \'"$(basename "$PRESENTATION_FILE")"\' found, cannot convert to HTML/PDF deck.
    exit 1
fi

# Setting name and chromium-based path
CHROME_PATH=$(which chromium)
echo Setting up chromium path for Marp at "$CHROME_PATH"...
BUILT_PRES_DECK=$DOCS_DIR/$(basename -- "$PRESENTATION_FILE" .md)
echo Building PDF decks with \'Marp\'...
marp --html --pdf "$PRESENTATION_FILE" -o "$BUILT_PRES_DECK".pdf

echo Building HTML decks for GitHub Pages with \'Marp\'...
BUILT_PRES_PAGES=$PAGES_DIR/$(basename -- "$PRESENTATION_FILE" .md)
marp --html "$PRESENTATION_FILE" -o "$BUILT_PRES_PAGES".html

echo Done! Exiting nix-shell.
exit 0