<script setup>
import { ref } from 'vue';
import { my_project_backend } from 'declarations/my_project_backend/index';
let chat = ref([]);
let sending_status = ref(0);
let msg_len = ref(0);
let un_len = ref(0);

async function getChat() {
  chat.value = await my_project_backend.get_chat();
}

const stringToColor = (str) => {
  let hash = 0;
  str.split('').forEach(char => {
    hash = char.charCodeAt(0) + ((hash << 5) - hash)
  })
  let color = '#'
  for (let i = 0; i < 3; i++) {
    const value = (hash >> (i * 8)) & 0xff
    color += value.toString(16).padStart(2, '0')
  }
  return color
}

async function send(e) {
  sending_status.value++;
  const input = document.querySelector("#message");
  const username = document.querySelector("#username");
  my_project_backend.save_msg(input.value, username.value).then(() => {
    getChat();
    sending_status.value--;
  });
  input.value = "";
  msg_len.value = 0
}

const upd_msg_len = (e) => { msg_len.value = e.target.value.length; }
const upd_un_len = (e) => { un_len.value = e.target.value.length; }
const a = "a";

getChat();
</script>

<template>
  <main>
    <section id="chat">
      <div class="message" v-for="msg in chat.slice().reverse()">
        <div class="image-wrapper">
          <div class="image" v-bind:style="{ background: stringToColor(msg.sender) }">
            <img src="/user_icon.svg" />
          </div>
        </div>
        <div class="data">
          <div class="sender">{{ msg.sender || "<anonymous>" }}</div>
          <div class="content">{{ msg.content }}</div>
        </div>
      </div>
    </section>
    <section id="inputs">
      <div id="message-input">
        <input id="message" name="message" alt="Message" type="text" @keyup.enter="send" @input="upd_msg_len"
          placeholder="message" maxlength="2000"/>
        <div>{{ msg_len }}/2000</div>
        <button @click="send">
          <img src="/send.svg" />
        </button>
      </div>
      <div>
        <div style="display: flex">
          <label for="username">Username:</label>
          <div id="username-wrapper">
            <input id="username" name="sername" alt="UserName" maxlength="100" type="text" placeholder="<anonymous>" @input="upd_un_len"/>
            <div>{{ un_len }}/100</div>
          </div>
        </div>
        <div id="sending-status">
          <span v-if="sending_status > 0">Sending {{ sending_status }} message(s)..</span>
        </div>
      </div>
    </section>
  </main>
</template>
