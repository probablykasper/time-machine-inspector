import { invoke } from '@tauri-apps/api/tauri'
import { get, writable } from 'svelte/store'
import { popup, runCmd } from '../general'

export type Backups = {
  dirs: DirMap
  rootPath: string
  status: string
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
      backupInfos.load()
    },
  }
})()

export type BackupInfo = {
  old: string
  new: string
  loading: boolean
}
export const backupInfos = (() => {
  const store = writable([] as BackupInfo[])
  return {
    subscribe: store.subscribe,
    load: async () => {
      const result = (await runCmd('backups_info')) as BackupInfo[]
      const $page = get(page)
      result.find((info) => {
        if (info.old === $page.prevPath && info.new === $page.fullPath) {
          if ($page.loading !== info.loading) {
            page.set_loading(info.loading)
          }
        }
      })
      store.set(result)
    },
  }
})()

export const selectedPath = writable(null as string | null)

export type PageMap = {
  [name: string]: PageItems
}
export type PageItems = {
  [name: string]: PageItem
}
export type PageItem = {
  size: number
  isOpen?: boolean
}

export const pageMap = writable({} as PageMap)

type Page = {
  fullPath: string
  name: string
  prevPath: string
  loading: boolean
}
export const page = (() => {
  const store = writable<Page>({
    fullPath: '',
    name: '',
    prevPath: '',
    loading: false,
  })
  return {
    subscribe: store.subscribe,
    set: (value: Page) => {
      store.set(value)
      backupInfos.load()
    },
    set_loading: (value: boolean) => {
      store.update(($page) => {
        $page.loading = value
        return $page
      })
    },
  }
})()

export function close() {
  page.set({
    fullPath: '',
    name: '',
    prevPath: '',
    loading: false,
  })
  backups.set(null)
}

export async function loadBackups(refresh = false) {
  const result = (await invoke('load_backup_list', { refresh }).catch((msg) => {
    if (msg.trim() === 'tmutil error 1:\nNo machine directory found for host.') {
      return { map: { '/': [] }, status: 'No backups found' }
    } else {
      popup(msg)
      throw msg
    }
  })) as { map: DirMap; status?: string }
  console.log('Loaded backups', result)
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
    status: result.status || '',
  })
}
