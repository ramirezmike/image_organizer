# image_organizer
A simple application to organize images into folders made in Rust. 

## Proposed Features/Goals:
  - Quick startup and be able to start categorizing images with just a few key commands
  - Keyboard-focused interface
  - Iterates through images in a folder and allow user to add tags
  - Progress bar / estimated time remaining
  - User can select one of the following from a menu
    - save progress (saves a file in the working folder)
    - load new folder (uses save file if one exists in folder)
    - run a batch job with options of either copying, moving or linking images into folders based on tags
    - exit application
  - Pause and resume
  - Key commands
    - \[] - skip to next untagged image
    - {} - skip to next tagged image
    - Enter/RightArrow - next image 
    - LeftArrow - previous image
    - Up/Down - navigate folders?
    - Escape - show Menu
    - A-z - setup a tag / use tag if one has been set for key
    - Delete - tags an image to be deleted during batch job and hides it from queue
  - Mockup of UI:
    ![Mockup](ui_mockup.jpg?raw=true "UI Mockup")

## Limitations
  - Currently, the iced UI framework doesn't have asynchronous support for loading images and since it takes a second or two to load larger images (>8mb), the queue will only show file names instead of image previews. As iced gets updated almost daily, this may improve in the future.
  - Currently, the iced UI framework can't overlay elements, so the interface will be implemented without layers/modals

## Dependencies
Running on Ubuntu requires freetype2

  sudo apt install libfreetype6-dev
  export FONTCONFIG_FILE=/etc/fonts
