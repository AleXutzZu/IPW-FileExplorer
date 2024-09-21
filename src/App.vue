<script setup lang="ts">
import Folder from "./components/Folder.vue";
import {ref} from "vue";
import {invoke} from "@tauri-apps/api";

const rootFolder = ref("");
const errorMessage = ref("No folder picked");
const inputFolder = ref("");

async function setFolder() {
    rootFolder.value = "";
    try {
        await invoke("check_if_dir", {filename: inputFolder.value});
        rootFolder.value = inputFolder.value;
    } catch (error) {
        errorMessage.value = "Could not read contents!"
    }
}
</script>

<script lang="ts">

const message = ref("");
const showNotification = ref(false);
const timerID = ref(-1);

export function setMessage(newMessage: string) {
    if (timerID.value > 0) clearTimeout(timerID.value);
    message.value = newMessage;
    showNotification.value = true;

    timerID.value = setTimeout(() => {
        showNotification.value = false;
    }, 5000)
}
</script>

<template>
    <div class="container">
        <h1>File Explorer</h1>
        <div class="folder-dialog">
            <input class="general-input" placeholder="Paste folder path" v-model="inputFolder"/>
            <button class="general-input" @click="setFolder">Pick folder</button>
        </div>
        <div class="folders" v-if="rootFolder.length!=0">
            <Folder :folder="rootFolder"/>
        </div>
        <h3 v-else class="error-message">{{ errorMessage }}</h3>
    </div>
    <div class="notification" v-if="showNotification">
        {{ message }}
    </div>
</template>

<style scoped>
.folders {
    align-self: start;
}

.folder-dialog {
    display: flex;
    width: 350px;
    justify-content: space-between;
    align-items: center;
    align-self: center;
    margin-bottom: 20px;
}

.error-message {
    color: red;
}

.notification {
    position: absolute;
    top: 0;
    right: 0;
    background-color: #0f0f0f69;
    opacity: 0.4;
    height: 40px;
    width: 250px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 8px;
    border: 1px solid transparent;
}

</style>

