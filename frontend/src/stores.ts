import { writable } from "svelte/store";

export interface UserInfo {
    id: number,
    username: string,
    orig: boolean,
    manage_links: boolean,
    manage_users: boolean,
    disabled: boolean
}
const emptyUser: UserInfo | null = null;
export const currentUser = writable(emptyUser);
