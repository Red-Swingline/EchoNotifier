# Echo Notifier

I have developed this application that monitors dbus events for notifications and plays custom sounds for specific applications. Users can configure the application and customize the sound to be played. The reason behind creating this application was my frustration with using the Microsoft Teams web app on Linux. I was missing notifications as the app was not playing sounds. This application is probably not of much use to anyone else as it was developed to solve my specific problem.
<div style="text-align: center;">
    <a href="https://www.buymeacoffee.com/swingline" target="_blank">
        <img src="https://cdn.buymeacoffee.com/buttons/default-orange.png" alt="Buy Me A Coffee" height="41" width="174">
    </a>
</div>

## Whats mostly working 
- Adding applications
- Editing sound file selection for applications
- deleting applications

## TODO
- [ ] Restart the thread watching dbus after config changes are made.
- [ ] Clean up application card style. 

## Screenshots

<img src="screenshots/main_view.png" alt="Screenshot of Application" width="300">
<img src="screenshots/add_app.png" alt="Screenshot of Application" width="300">
<img src="screenshots/delete_app.png" alt="Screenshot of Application" width="300">
