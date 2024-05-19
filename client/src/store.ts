import { reactive } from 'vue';

const store = reactive({
  user: undefined,
  login(username : string) {
    this.user = username;
  },
  logout() {
    this.user = undefined;
  }
});

export default store;
3
