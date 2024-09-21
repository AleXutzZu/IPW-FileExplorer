<script setup lang="ts">

import {invoke} from "@tauri-apps/api";
import {ref, Ref} from "vue";
import File from "./File.vue";
import {setMessage} from "../App.vue";

const props = defineProps<{ folder: string }>();
const showContents: Ref<boolean> = ref(false);
const isFolder = async (target: string): Promise<boolean> => {
    try {
        return await invoke("check_if_dir", {filename: target}) as boolean;
    } catch (error) {
        return false;
    }
}

const getContents = async (): Promise<string[]> => {
    try {
        return await invoke("get_files_from_dir", {dir: props.folder}) as string[];
    } catch (error) {
        return [];
    }
}

const getFiles = async (): Promise<string[]> => {
    const list: string[] = [];
    const contents = await getContents();

    for (const item of contents) {
        if (!(await isFolder(item))) list.push(item);
    }
    return list;
}

const getFolders = async (): Promise<string[]> => {
    const list: string[] = [];
    const contents = await getContents();

    for (const item of contents) {
        if (await isFolder(item)) list.push(item);
    }
    return list;
}
const folders: Ref<string[]> = ref([]);
getFolders().then(value => folders.value = value);

const files: Ref<string[]> = ref([]);
getFiles().then(value => files.value = value);

const input: Ref<string> = ref("");

const addFile = async () => {
    if (input.value.length == 0) {
        setMessage("Invalid input");
        return;
    }
    try {
        await invoke("create_file", {filename: props.folder + "\\" + input.value});

        getFolders().then(value => folders.value = value);
        getFiles().then(value => files.value = value);
        setMessage("File created successfully")
    } catch (error) {
        setMessage("Could not create file")
    }
}

const addFolder = async () => {
    if (input.value.length == 0) {
        setMessage("Invalid input");
        return;
    }
    try {
        await invoke("create_folder", {folder: props.folder + "\\" + input.value});

        getFolders().then(value => folders.value = value);
        getFiles().then(value => files.value = value);
        setMessage("Folder created successfully");
    } catch (error) {
        setMessage("Folder could not be created");
    }
}

const removeFileFactory = (file: string) => {
    return async () => {
        await invoke("delete_file", {filename: file});
        getFolders().then(value => folders.value = value);
        getFiles().then(value => files.value = value);
    }
}

</script>

<template>
    <div class="container-form">
        <div @click="showContents = !showContents" class="folder">
            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5"
                 stroke="currentColor"
                 class="size-5 folder-icon">
                <path stroke-linecap="round" stroke-linejoin="round"
                      d="M2.25 12.75V12A2.25 2.25 0 0 1 4.5 9.75h15A2.25 2.25 0 0 1 21.75 12v.75m-8.69-6.44-2.12-2.12a1.5 1.5 0 0 0-1.061-.44H4.5A2.25 2.25 0 0 0 2.25 6v12a2.25 2.25 0 0 0 2.25 2.25h15A2.25 2.25 0 0 0 21.75 18V9a2.25 2.25 0 0 0-2.25-2.25h-5.379a1.5 1.5 0 0 1-1.06-.44Z"/>
            </svg>
            {{ folder.split("\\").slice(-1)[0] }}
        </div>
        <div v-if="showContents" class="form">
            <input v-model="input" placeholder="Enter a name" class="general-input"/>
            <button @click="addFile" class="form-buttons general-input">Add file</button>
            <button @click="addFolder" class="form-buttons general-input">Add folder</button>
        </div>
    </div>

    <div v-if="showContents" class="content">
        <div v-for="folder in folders" class="spacing">
            <Folder :folder="folder"/>
        </div>
        <div v-for="file in files" class="spacing">
            <File :filename="file" :delete-self="removeFileFactory(file)"/>
        </div>
    </div>
</template>

<style scoped>
.folder {
    display: flex;
    justify-content: center;
    align-items: center;
    cursor: pointer;
    padding: 0.3em 1.2em;
}

.spacing {
    padding-left: 40px;
}

.folder-icon {
    margin-right: 10px;
}

.content {
    display: flex;
    flex-direction: column;
    align-items: start;
    justify-items: center;
}

.container-form {
    display: flex;
}

.form {
    margin-left: 10px;
}

.form-buttons {
    margin-left: 15px;
}

p {
    margin: 0;
}


</style>
