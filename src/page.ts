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

export const backups = writable<Backups | null>(null)

export const page = writable({
  fullPath: '',
  name: '',
  prevPath: '',
})

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
