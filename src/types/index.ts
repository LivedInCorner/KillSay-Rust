// 配置相关类型
export interface Config {
  sniper_list: string[];
  join_patterns: string[];
  anti_snipe_messages: string[];
  kill_patterns: string[];
  kill_messages: string[];
  message_prefix: string;
  killsay_format: string;
  chat_key: string;
}

export interface ConfigInfo {
  name: string;
  path: string;
  modified: string;
  is_default: boolean;
}

export interface HistoryItem {
  value: string;
  is_default: boolean;
}

export interface UserHistory {
  items: HistoryItem[];
}

export interface PathHistory {
  items: HistoryItem[];
}

export interface AppSettings {
  default_config: string | null;
  user_history: UserHistory;
  path_history: PathHistory;
}

// 监控相关类型
export interface MonitorParams {
  user: string;
  window_name: string;
  log_file_path: string;
  sniper_list: string[];
  join_patterns: string[];
  anti_snipe_messages: string[];
  kill_patterns: string[];
  kill_messages: string[];
  anti_snipe_enabled: boolean;
  message_prefix: string;
  killsay_format: string;
  chat_key: string;
}

export interface MonitorStatus {
  is_running: boolean;
  kills: number;
  deaths: number;
}

// 更新相关类型
export interface UpdateInfo {
  has_update: boolean;
  current_version: string;
  latest_version: string;
  release_url: string;
  release_notes: string;
  debug_info: string;
}

export interface DownloadProgress {
  downloaded: number;
  total: number;
  percentage: number;
}

export type MonitorEvent =
  | {
      type: "Kill";
      killer: string;
      victim: string;
      message: string;
      kills: number;
      deaths: number;
    }
  | {
      type: "Death";
      victim: string;
      kills: number;
      deaths: number;
    }
  | {
      type: "AntiSnipe";
      player: string;
    }
  | {
      type: "Error";
      message: string;
    }
  | {
      type: "StatusChanged";
      is_running: boolean;
    };
