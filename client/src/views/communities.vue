<template>
    <div>
        <h1>Communities</h1>

        <div v-if="loading" class="loading">Loading...</div>
        <div v-if="error" class="error">{{ error }}</div>
        <div v-if="communities" class="content">
            <Community
                v-for="community in communities" :key="community.id"
                :name="community.name"
                :id="community.id"
            ></Community>
        </div>
    </div>

    <form @submit.prevent="createCommunity">
        <h2>Create community</h2>
        <input required name="name" placeholder="name"/>
        <button type="submit">Create</button>
    </form>
</template>

<script>
    import Community from '../components/community.vue'
    import store from '../store'

    export default {
        name: 'communities',
        data() {
            return {
                loading: false,
                communities: null,
                error: null,
            };
        },
        created() {
            this.fetchCommunities();
        },
        methods: {
            async fetchCommunities() {
                this.error = this.communities = null;
                this.loading = true;

                try{
                    let response = await fetch(`/api/user/${ store.user.id }/communities`);
                    if(! response.ok) throw new Error(`Server error (${response.status})`);
                    this.communities = await response.json();
                } catch (err) {
                    this.error = err.toString();
                    throw err;
                } finally {
                    this.loading = false;
                }
            },
            async createCommunity(event) {
                console.warn(event);
                let name = event.target.children[1].value; // TODO have a reference or something
                console.warn("Creating community", name);
                try {
                    let response = await fetch('/api/communities', { method: 'POST', body: name });
                    if(! response.ok) throw new Error(`Server error (${response.status})`);
                    fetchCommunities();
                    event.target.clear();
                } catch(err) {
                    this.err = err.toString();
                    throw err;
                }
            },
        },
        components : {
            Community,
        },
    }
</script>

<style scoped>
</style>
