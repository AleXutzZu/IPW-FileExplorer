<script setup lang="ts">
import {ref, Ref} from "vue";
import {invoke} from "@tauri-apps/api";
import {setMessage} from "../App.vue";

const props = defineProps<{ filename: string, deleteSelf: () => Promise<void> }>();

const content: Ref<string> = ref("");
const showEditor: Ref<boolean> = ref(false);

invoke("read_file_content", {filename: props.filename})
    .then(value => content.value = value as string)
    .catch(_error => setMessage("Could not read file contents"))

const saveFile = async () => {
    try {
        await invoke("write_to_file", {filename: props.filename, content: content.value});
        setMessage("File saved successfully");
    } catch (error) {
        setMessage("File could not be saved");
    }
    showEditor.value = false;
}

const deleteFile = async () => {
    showEditor.value = false;
    try {
        await props.deleteSelf();
        setMessage("File deleted successfully")
    } catch (error) {
        setMessage("File could not be deleted");
    }
}

</script>

<template>
    <div class="file-container">
        <p v-if="!showEditor" @click="showEditor = !showEditor">{{ filename.split("\\").slice(-1)[0] }}</p>
        <div v-else class="editor-container">
            <p>Editing <b>{{ filename.split("\\").slice(-1)[0] }}</b></p>
            <textarea v-model="content" class="editor-textarea"></textarea>
            <div class="editor-actions">
                <button @click="saveFile" class="general-input">Save file</button>
                <button @click="showEditor = !showEditor" class="general-input">Cancel</button>
                <button @click="deleteFile" class="general-input">Delete file</button>
            </div>

        </div>
    </div>
</template>

<style scoped>
.file-container {
    padding: 0.3em 1.2em;
    cursor: pointer;
}

p {
    margin: 0;
}

.editor-container {
    display: flex;
    flex-direction: column;
    align-items: start;
    justify-content: center;
}

.editor-actions {
    width: 306px;
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-top: 5px;
}

.editor-textarea {
    resize: none;
    overflow: auto;
    width: 300px;
    height: 150px;
    border: 1px solid transparent;
    border-radius: 4px;
}
</style>
