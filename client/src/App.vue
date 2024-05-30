<script setup>
import { RouterLink, RouterView } from 'vue-router'
import store from './store'
</script>

<template>
  <div v-if="errorBubble" background="red">{{ errorBubble }}</div><!-- TODO: a fancy pop-up -->
  <header>
    <div class="wrapper" v-if="store.user">
      <span v-if="store.user">Logged in as {{ store.user.name }}</span>
      <button v-if="store.user" @click="store.logout">Log out</button>
    </div>

    <div v-else class="wrapper">
      <span>You are not logged in.</span>
      <form @submit.prevent="login_or_register"> <!-- TODO: this could be prettier -->
        <input placeholder="Username" required/>
        <button type="submit" name="login">Log in</button>
        <button type="submit" name="register">Register</button>
      </form>
    </div>

    <div class="wrapper">
      <nav>
        <RouterLink to="/">Home</RouterLink>
        <RouterLink v-if="store.user" to="/communities">Communities</RouterLink>
        <RouterLink to="/bet">Bet!</RouterLink>
      </nav>
    </div>
  </header>

  <RouterView />
</template>

<script>
export default {
  data() { return {
    errorBubble: undefined, // TODO popup
  }; },
  beforeCreate() {
    let query = (new URL(window.location.href)).searchParams;
    if(query.has("nav_to")){
      this.$router.push(query.get("nav_to"));
    }
  },
  methods: {
    login(username) {
      if(import.meta.env.MODE == "development"){ store.login(username); return; }
      fetch("/api/login", { method: 'POST', body: username })
        .then(async response => {
          if(!response.ok) throw new Error("API error:" + response.status);
          this.on_successful_login_response(await response.json());
        })
        .catch(err => this.errorBubble = err);
    },
    register(username) {
      fetch("/api/users", { method: 'POST', body: username })
        .then(async response => {
          if(!response.ok) throw new Error("API error:" + response.status);
          const userId = response.headers.get('Location').split('/')[2];
          this.on_successful_login_response({ name: username, id: userId }); // let's not actually call the API
        })
        .catch(err => this.errorBubble = err);
    },
    on_successful_login_response(user) {
      store.login(user);
      let query = (new URL(window.location.href)).searchParams;
      if(query.has("redirect")){ // TODO: do that only on the login page, else it could get confusing
        this.$router.push(query.get("redirect"));
      }
    },
    login_or_register(event) {
      const username = event.target.children[0].value;
      if(event.submitter.name == "login") this.login(username);
      else if(event.submitter.name == "register") this.register(username);
      else throw new Error("Assertion failed: unknown action " + event.submitter.name);
    },
  }
}
</script>

<style scoped>
header {
  line-height: 1.5;
  max-height: 100vh;
}

.logo {
  display: block;
  margin: 0 auto 2rem;
}

nav {
  width: 100%;
  font-size: 12px;
  text-align: center;
  margin-top: 2rem;
}

nav a.router-link-exact-active {
  color: var(--color-text);
}

nav a.router-link-exact-active:hover {
  background-color: transparent;
}

nav a {
  display: inline-block;
  padding: 0 1rem;
  border-left: 1px solid var(--color-border);
}

nav a:first-of-type {
  border: 0;
}

@media (min-width: 1024px) {
  header {
    display: flex;
    place-items: center;
    padding-right: calc(var(--section-gap) / 2);
  }

  .logo {
    margin: 0 2rem 0 0;
  }

  header .wrapper {
    display: flex;
    place-items: flex-start;
    flex-wrap: wrap;
  }

  nav {
    text-align: left;
    margin-left: -1rem;
    font-size: 1rem;

    padding: 1rem 0;
    margin-top: 1rem;
  }
}
</style>
