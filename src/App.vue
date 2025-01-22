
<template class="container">
  <main class="container">
    <div class="files">
      <li v-for="file in files" :key="files">
        {{ file.name }}
      </li>
    </div>
    <textarea class="text"></textarea>
  </main>
</template>
<script lang="ts">
  import { listen } from '@tauri-apps/api/event'
import { Ref, ref } from 'vue';
  enum FileType {
    main,
    file,
    folder
  }


  type FilePath = {
    fileType: FileType,
    name: string
  }
  let files: Ref<FilePath[]> = ref([]);

  listen<[FilePath]>('open_files', (event) => {
    files = ref(event.payload);
  });

</script>