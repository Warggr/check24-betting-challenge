import { reactive, set } from 'vue';
import { apiFetch } from './api';

type User = { name: string, id: number };

type GameResult = { home: number, away: number };

const store = reactive({
  user:
    import.meta.env.MODE == "development"
      ? { name: "TestUser", id: 0 }
      : undefined,
  evtSource : undefined,
  results : {}, // TODO fetch initial value from server
  login(user : User) {
    this.user = user;
  },
  logout() {
    this.user = undefined;
  },
  async watchServer() {
    let results = await apiFetch('/results');
    this.results = Object.fromEntries(results.map(result => [ result.game_id, { home: result.home, away: result.away } ] ));

    this.evtSource = new EventSource("/test/events");
    this.evtSource.onmessage = event => {
      let data = JSON.parse(event.data);
      if(data.action == "update") {
        this.results[data.game_id] = data.new_value;
      } else {
        console.warn("Not acted upon:", data);
      }
    };
  },
  unwatchServer() {
    this.evtSource.close();
    this.evtSource = undefined;
  },
});

export default store;
