import { writable } from 'svelte/store'
import { runCmd } from './general'

export type Backups = {
  dirs: DirMap
  rootPath: string
}
export type DirMap = {
  '/': DirContent
  [name: string]: DirContent
}
export type DirContent = {
  [name: string]: null
}

export const backups = (() => {
  const store = writable<Backups | null>(null)
  return {
    subscribe: store.subscribe,
    set: (value: Backups | null) => {
      store.set(value)
      loadCachedBackups()
    },
  }
})()

export const cachedBackups = writable([] as string[])
export async function loadCachedBackups() {
  const result = (await runCmd('cached_backups')) as [string, string][]
  const newOnly = result.map((b) => b[1])
  cachedBackups.set(newOnly)
}

type Page = {
  fullPath: string
  name: string
  prevPath: string
}
export const page = (() => {
  const store = writable<Page>({
    fullPath: '',
    name: '',
    prevPath: '',
  })
  return {
    subscribe: store.subscribe,
    set: (value: Page) => {
      store.set(value)
      loadCachedBackups()
    },
  }
})()

export function close() {
  page.set({
    fullPath: '',
    name: '',
    prevPath: '',
  })
  backups.set(null)
}

export async function loadBackups(refresh = false) {
  const result = (await runCmd('load_backups', { refresh })) as { map: DirMap }
  console.log(result)
  const map = result.map

  let rootPath = '/'
  if (Object.keys(map[rootPath]).length === 1 && map[rootPath]['Volumes'] !== undefined) {
    rootPath += 'Volumes'
    if (
      Object.keys(map[rootPath]).length === 1 &&
      map[rootPath]['Time Machine Backups'] !== undefined
    ) {
      rootPath += '/Time Machine Backups'
      if (
        Object.keys(map[rootPath]).length === 1 &&
        map[rootPath]['Backups.backupdb'] !== undefined
      ) {
        rootPath += '/Backups.backupdb'
      }
    }
  }

  backups.set({
    dirs: result.map,
    rootPath,
  })
}
