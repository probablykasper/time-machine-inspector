import { get, writable } from 'svelte/store'
import commands from '../lib/commands'
import type { Backup } from '../../bindings'

export const backups = (() => {
  const store = writable<Backup[] | null>(null)
  return {
    subscribe: store.subscribe,
    set: (value: Backup[] | null) => {
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
      const result = await commands.backupsInfo()
      const $page = get(page)

      for (const info of result) {
        if (info.new === $page.backup?.path) {
          if ($page.loading !== info.loading) {
            page.set_loading(info.loading)
          }
        }
      }
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
  backup: Backup | null
  loading: boolean
}
export const page = (() => {
  const store = writable<Page>({
    backup: null,
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
    backup: null,
    loading: false,
  })
  backups.set(null)
}
