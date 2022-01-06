import { writable } from 'svelte/store'
import { runCmd } from './general'

export type Backups = {
  dirs: DirMap
  rootPath: string
}
export type DirMap = {
  '/': string[]
  [name: string]: string[]
}

export const backups = writable<Backups | null>(null)

export const page = writable({
  fullPath: '',
  name: '',
})

export function close() {
  page.set({
    fullPath: '',
    name: '',
  })
  backups.set(null)
}

function parseDirs(paths: string[]): DirMap {
  const dirmap: DirMap = { '/': [] }
  paths.forEach((path) => {
    let base = path.substring(path.lastIndexOf('/') + 1)
    let parent = path.substring(0, path.lastIndexOf('/'))
    do {
      if (parent === '') {
        dirmap['/'].push(base)
        break
      } else if (dirmap[parent] === undefined) {
        dirmap[parent] = [base]
      } else {
        dirmap[parent].push(base)
        break
      }
      base = parent.substring(parent.lastIndexOf('/') + 1)
      parent = parent.substring(0, parent.lastIndexOf('/'))
    } while (dirmap[parent] === undefined)
  })
  return dirmap
}

export function parseStdout(stdout: string | null): Backups | null {
  if (stdout === null) {
    return null
  }
  const paths = stdout.split('\n').filter((path) => path !== '')
  const dirs = parseDirs(paths)
  let rootPath = '/'
  if (dirs[rootPath].length === 1 && dirs[rootPath][0] === 'Volumes') {
    rootPath += 'Volumes'
    if (dirs[rootPath].length === 1 && dirs[rootPath][0] === 'Time Machine Backups') {
      rootPath += '/Time Machine Backups'
      if (dirs[rootPath].length === 1 && dirs[rootPath][0] === 'Backups.backupdb') {
        rootPath += '/Backups.backupdb'
      }
    }
  }
  return { dirs, rootPath }
}

export async function loadBackups() {
  const stdout = (await runCmd('load_backups')) as string | null
  backups.set(parseStdout(stdout))
  console.log(parseStdout(stdout))
}
