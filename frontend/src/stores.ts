import { writable } from "svelte/store"
import { invoke } from "@tauri-apps/api/core"

// ### RE-EXPORT IPC STORES
export {
    babuState,
    ipcConnected,
    lastRecognizedText,
    lastExecutedCommand,
    lastError,
    connectIpc,
    enableIpc,
    disableIpc,
    disconnectIpc,
    sendAction,
    sendIpcMessage,
    sendTextCommand,
    stopBabuApp,
    reloadCommands
} from "./lib/ipc"

// re-export i18n
export {
    translations,
    currentLanguage,
    translate,
    loadTranslations,
    setLanguage,
    loadLanguage,
    getSupportedLanguages
} from "./lib/i18n"

// ### RUNNING STATE
export const isBabuRunning = writable(false)
export const babuRamUsage = writable(0)
export const babuCpuUsage = writable(0)

// ### ASSISTANT VOICE
export const assistantVoice = writable("")

// ### APP INFO
export const appInfo = writable({
    tgOfficialLink: "",
    feedbackLink: "",
    repositoryLink: "",
    boostySupportLink: "",
    patreonSupportLink: "",
    logFilePath: ""
})

// ### INIT FUNCTIONS (call these from a component)
export async function loadVoiceSetting() {
    try {
        const voice = await invoke<string>("db_read", { key: "assistant_voice" })
        assistantVoice.set(voice)
    } catch (err) {
        console.error("failed to load voice setting:", err)
    }
}

export async function loadAppInfo() {
    try {
        const [tg, feedback, repo, boosty, patreon, logPath] = await Promise.all([
            invoke<string>("get_tg_official_link"),
            invoke<string>("get_feedback_link"),
            invoke<string>("get_repository_link"),
            invoke<string>("get_boosty_link"),
            invoke<string>("get_patreon_link"),
            invoke<string>("get_log_file_path")
        ])

        appInfo.set({
            tgOfficialLink: tg,
            feedbackLink: feedback,
            repositoryLink: repo,
            boostySupportLink: boosty,
            patreonSupportLink: patreon,
            logFilePath: logPath
        })
    } catch (err) {
        console.error("failed to load app info:", err)
    }
}

export async function updateBabuStats() {
    try {
        const stats = await invoke<{running: boolean, ram_mb: number, cpu_usage: number}>("get_babu_app_stats")
        isBabuRunning.set(stats.running)
        babuRamUsage.set(stats.ram_mb)
        babuCpuUsage.set(stats.cpu_usage)
    } catch (err) {
        console.error("failed to get babu stats:", err)
    }
}

// polling manager
let statsInterval: ReturnType<typeof setInterval> | null = null

export function startStatsPolling(intervalMs = 5000) {
    if (statsInterval) return // already running
    
    updateBabuStats()
    statsInterval = setInterval(updateBabuStats, intervalMs)
}

export function stopStatsPolling() {
    if (statsInterval) {
        clearInterval(statsInterval)
        statsInterval = null
    }
}