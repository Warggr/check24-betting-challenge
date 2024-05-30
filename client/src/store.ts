import { reactive } from 'vue';

type User = { name: string, id: number };

const store = reactive({
  user:
    import.meta.env.MODE == "development"
      ? { name: "TestUser", id: 0 }
      : undefined,
  login(user : User) {
    this.user = user;
  },
  logout() {
    this.user = undefined;
  }
});

export default store;
