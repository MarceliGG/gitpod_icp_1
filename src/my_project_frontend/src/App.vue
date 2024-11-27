<script setup>
import { ref } from 'vue';
import { my_project_backend } from 'declarations/my_project_backend/index';
let chat = ref([]);
let sending_status = ref(0);

async function getChat() {
  chat.value = await my_project_backend.get_chat();
}

async function handleSubmit(e) {
  e.preventDefault();
  const target = e.target;
  sending_status.value++;
  my_project_backend.save_msg(target['message'].value).then(() => {
    getChat();
    sending_status.value--;
  });
  target['message'].value = "";
}

getChat();
</script>

<template>
  <main>
    <section id="chat">
      <div class="message" v-for="msg in chat.slice().reverse()">
        {{ msg }}
      </div>
    </section>
    <form action="#" @submit="handleSubmit">
      <input id="message" name="message" alt="Message" type="text" />
      <button type="submit">Send</button>
    </form>
    <div id="sending-status" v-if="sending_status > 0">Sending {{ sending_status }} message(s)..</div>
  </main>
</template>
