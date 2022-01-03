import { writable } from 'svelte/store'

export const page = writable({
  fullPath: '',
  name: '',
})
