const { invoke } = window.__TAURI__.tauri;

async function loadApps() {
    try {
        const appsHtml = await invoke("load_apps");
        const appContainer = document.getElementById('app-container');
        appContainer.innerHTML = appsHtml || '<p>No apps configured.</p>';
    } catch (error) {
        console.error('Error loading apps:', error);
        document.getElementById('app-container').innerHTML = '<p>Error loading apps.</p>';
    }
}

async function editApp(buttonElement) {
    const appCard = buttonElement.closest('.app-card');
    const appName = appCard.getAttribute('data-app-name');

    try {
        const filePath = await invoke("select_sound_file");
        if (filePath) {
            await invoke("edit_app", { appName, newSoundPath: filePath });
            loadApps();
        }
    } catch (error) {
        console.error('Error editing app:', error);
    }
}

window.editApp = editApp;

async function deleteApp(appName) {
    const userConfirmed = await customConfirm(`Are you sure you want to delete ${appName}?`);
    if (!userConfirmed) return;

    try {
        await invoke("delete_app", { appName });
        loadApps();
    } catch (error) {
        console.error('Error deleting app:', error);
    }
}

window.deleteApp = deleteApp;

function customConfirm(message) {
    return new Promise((resolve) => {
        resolve(confirm(message));
    });
}

async function addApp() {
    const appName = document.getElementById('newAppName').value;
    const soundPath = document.getElementById('fullSoundFilePath').value;

    if (!appName.trim() || !soundPath) {
        alert('Please enter an app name and select a sound file.');
        return;
    }

    try {
        await invoke("add_app", { appName, soundPath });
        loadApps();
        hideAddAppForm();
    } catch (error) {
        console.error('Error adding app:', error);
    }
}

window.addApp = addApp;

function hideAddAppForm() {
    loadApps()
    document.getElementById('addAppForm').style.display = 'none';
    document.getElementById('newAppName').value = '';
    document.getElementById('selectedSoundFilePath').innerText = '';
    document.getElementById('fullSoundFilePath').value = '';
}


window.hideAddAppForm = hideAddAppForm;

document.getElementById('add-button').addEventListener('click', async () => {
    const addAppFormHtml = await invoke("get_add_app_form");
    document.getElementById('app-container').innerHTML = addAppFormHtml;

    document.getElementById('selectSoundFileButton').addEventListener('click', async () => {
        const filePath = await invoke("select_sound_file");
        document.getElementById('fullSoundFilePath').value = filePath || '';
        document.getElementById('selectedSoundFilePath').innerText = truncateFilePath(filePath) || 'No file selected';
    });
});

window.addEventListener("DOMContentLoaded", () => {
    loadApps();
});

function truncateFilePath(filePath, maxLength = 30) {
    if (!filePath) return '';
    if (filePath.length <= maxLength) return filePath;

    const segments = filePath.split(/[/\\]/);
    const fileName = segments.pop();

    let truncatedPath = '.../' + fileName;

    for (let i = segments.length - 1; i >= 0; i--) {
        if ((segments[i].length + truncatedPath.length + 1) > maxLength) break;
        truncatedPath = segments[i] + '/' + truncatedPath;
    }

    return truncatedPath;
}
// This will diable right click
window.addEventListener('contextmenu', (event) => {
event.preventDefault();
});
