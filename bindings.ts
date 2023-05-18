// This file was generated by [tauri-specta](https://github.com/oscartbeaumont/tauri-specta). Do not edit this file manually.

declare global {
    interface Window {
        __TAURI_INVOKE__<T>(cmd: string, args?: Record<string, unknown>): Promise<T>;
    }
}

const invoke = window.__TAURI_INVOKE__;

export function errorPopup(msg: string) {
    return invoke<null>("error_popup", { msg })
}

export function loadBackupList(destinationId: string, refresh: boolean) {
    return invoke<Backup[]>("load_backup_list", { destinationId,refresh })
}

export function getBackup(destinationId: string, newB: string, refresh: boolean) {
    return invoke<DirMap>("get_backup", { destinationId,newB,refresh })
}

export function backupsInfo() {
    return invoke<BackupInfo[]>("backups_info")
}

export function destinationinfo() {
    return invoke<DestinationXml[]>("destinationinfo")
}

export type Backup = { path: string; name: string }
export type DestinationXml = { kind: string; url: string; name: string; id: string; last_destination: number; mount_point: string }
export type DirMap = { map: { [key: string]: { [key: string]: LoadedBackupItem } } }
export type LoadedBackupItem = { size: number }
export type BackupInfo = { old: string; new: string; loading: boolean }