import { Community } from './community'
import store from '../store'

async function apiFetch (path : string, options = {}, json=true) {
    let response = await fetch('/api' + path, options);
    if(! response.ok ) throw new Error(`Server error (${response.status})`);
    if(json) return await response.json();
    else return response;
};

async function userApiFetch(path : string, options = {}, json=true) {
    return apiFetch('/user/' + store.user!.id + path, options, json);
};

export {
    Community,
    apiFetch, userApiFetch,
}
