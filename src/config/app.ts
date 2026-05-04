// 版本信息 - 从 package.json 自动读取
export const APP_INFO = {
  name: "KILLSAY",
  version: "1.0.0", // 构建时会自动更新
  team: "Hurricane Centre",
  copyright: "Copyright 2025-2026 Hurricane Centre",
  description: "Minecraft 击杀喊话脚本",
};

// 从 package.json 读取版本号的辅助函数
export async function getAppVersion(): Promise<string> {
  try {
    // 在 Tauri 环境中，可以通过 invoke 获取版本
    const { invoke } = await import("@tauri-apps/api/core");
    return await invoke<string>("get_app_version");
  } catch {
    return APP_INFO.version;
  }
}
