import json
import os
from utils import get_base_dir

# ==================== 主题管理系统 ====================
class ThemeManager:
    """管理应用程序主题"""

    # 预定义主题
    THEMES = {
        "dark_blue": {
            "name": "深蓝主题",
            "description": "经典深蓝配色，适合长时间使用",
            "colors": {
                "bg_primary": "#1a2a3a",
                "bg_secondary": "#2c3e50",
                "bg_tertiary": "#34495e",
                "text_primary": "#ecf0f1",
                "text_secondary": "#bdc3c7",
                "accent_primary": "#3498db",
                "accent_hover": "#2980b9",
                "accent_pressed": "#2980b9",
                "danger": "#e74c3c",
                "danger_hover": "#c0392b",
                "success": "#0c311c",
                "success_hover": "#042914",
                "secondary": "#95a5a6",
                "secondary_hover": "#7f8c8d"
            }
        },
        "dark_green": {
            "name": "深绿主题",
            "description": "深绿配色，给人清新自然的感觉",
            "colors": {
                "bg_primary": "#1a2520",
                "bg_secondary": "#2c3e2f",
                "bg_tertiary": "#344935",
                "text_primary": "#ecf0f1",
                "text_secondary": "#bdc3c7",
                "accent_primary": "#27ae60",
                "accent_hover": "#229954",
                "accent_pressed": "#229954",
                "danger": "#e74c3c",
                "danger_hover": "#c0392b",
                "success": "#0c311c",
                "success_hover": "#042914",
                "secondary": "#95a5a6",
                "secondary_hover": "#7f8c8d"
            }
        },
        "dark_purple": {
            "name": "深紫主题",
            "description": "深紫配色，神秘而优雅",
            "colors": {
                "bg_primary": "#1a1a2a",
                "bg_secondary": "#2c2c3e",
                "bg_tertiary": "#343449",
                "text_primary": "#ecf0f1",
                "text_secondary": "#bdc3c7",
                "accent_primary": "#9b59b6",
                "accent_hover": "#8e44ad",
                "accent_pressed": "#8e44ad",
                "danger": "#e74c3c",
                "danger_hover": "#c0392b",
                "success": "#0c311c",
                "success_hover": "#042914",
                "secondary": "#95a5a6",
                "secondary_hover": "#7f8c8d"
            }
        },
        "light_blue": {
            "name": "浅蓝主题",
            "description": "浅色主题，适合明亮环境",
            "colors": {
                "bg_primary": "#f8f9fa",
                "bg_secondary": "#e9ecef",
                "bg_tertiary": "#dee2e6",
                "text_primary": "#212529",
                "text_secondary": "#6c757d",
                "accent_primary": "#007bff",
                "accent_hover": "#0056b3",
                "accent_pressed": "#0056b3",
                "danger": "#dc3545",
                "danger_hover": "#c82333",
                "success": "#28a745",
                "success_hover": "#1e7e34",
                "secondary": "#6c757d",
                "secondary_hover": "#545b62"
            }
        },
        "dark_red": {
            "name": "深红主题",
            "description": "深红配色，醒目而有冲击力",
            "colors": {
                "bg_primary": "#2a1a1a",
                "bg_secondary": "#3e2c2c",
                "bg_tertiary": "#493434",
                "text_primary": "#ecf0f1",
                "text_secondary": "#bdc3c7",
                "accent_primary": "#e74c3c",
                "accent_hover": "#c0392b",
                "accent_pressed": "#c0392b",
                "danger": "#e74c3c",
                "danger_hover": "#c0392b",
                "success": "#0c311c",
                "success_hover": "#042914",
                "secondary": "#95a5a6",
                "secondary_hover": "#7f8c8d"
            }
        },
        "cyberpunk": {
            "name": "赛博朋克主题",
            "description": "霓虹灯风格，未来感十足",
            "colors": {
                "bg_primary": "#0a0a0a",
                "bg_secondary": "#1a1a1a",
                "bg_tertiary": "#2a2a2a",
                "text_primary": "#00ff41",
                "text_secondary": "#00cc33",
                "accent_primary": "#ff0080",
                "accent_hover": "#cc0066",
                "accent_pressed": "#cc0066",
                "danger": "#ff4444",
                "danger_hover": "#cc3333",
                "success": "#00ff41",
                "success_hover": "#00cc33",
                "secondary": "#666666",
                "secondary_hover": "#555555"
            }
        }
    }

    def __init__(self):
        self.current_theme = "dark_blue"
        self.custom_themes = {}  # 自定义主题存储
        self.load_custom_themes()
        self.load_theme()
        if self.current_theme not in self.THEMES and self.current_theme not in self.custom_themes:
            self.current_theme = "dark_blue"

    def load_theme(self):
        """从配置文件加载主题"""
        config_file = os.path.join(get_base_dir(), "theme_config.json")
        try:
            if os.path.exists(config_file):
                with open(config_file, 'r', encoding='utf-8') as f:
                    config = json.load(f)
                    self.current_theme = config.get("current_theme", "dark_blue")
        except:
            self.current_theme = "dark_blue"

    def load_custom_themes(self):
        """加载自定义主题"""
        custom_file = os.path.join(get_base_dir(), "custom_themes.json")
        try:
            if os.path.exists(custom_file):
                with open(custom_file, 'r', encoding='utf-8') as f:
                    data = json.load(f)
                    self.custom_themes = data if isinstance(data, dict) else {}
        except:
            self.custom_themes = {}

    def save_theme(self):
        """保存主题配置"""
        config_file = os.path.join(get_base_dir(), "theme_config.json")
        try:
            with open(config_file, 'w', encoding='utf-8') as f:
                json.dump({"current_theme": self.current_theme}, f, indent=2)
        except:
            pass

    def save_custom_themes(self):
        """保存自定义主题"""
        custom_file = os.path.join(get_base_dir(), "custom_themes.json")
        try:
            with open(custom_file, 'w', encoding='utf-8') as f:
                json.dump(self.custom_themes, f, indent=2, ensure_ascii=False)
        except:
            pass

    def create_custom_theme(self, name, description, colors):
        """创建自定义主题"""
        idx = 0
        while True:
            theme_key = f"custom_{idx}"
            if theme_key not in self.custom_themes:
                break
            idx += 1
        self.custom_themes[theme_key] = {
            "name": name,
            "description": description,
            "colors": colors,
            "custom": True
        }
        self.save_custom_themes()
        return theme_key

    def update_custom_theme(self, theme_key, name, description, colors):
        """更新自定义主题"""
        if theme_key in self.custom_themes:
            self.custom_themes[theme_key] = {
                "name": name,
                "description": description,
                "colors": colors,
                "custom": True
            }
            self.save_custom_themes()
            return True
        return False

    def delete_custom_theme(self, theme_key):
        """删除自定义主题"""
        if theme_key in self.custom_themes:
            del self.custom_themes[theme_key]
            self.save_custom_themes()
            # 如果删除的是当前主题，切换到默认主题
            if self.current_theme == theme_key:
                self.set_theme("dark_blue")
            return True
        return False

    def set_theme(self, theme_name):
        """设置当前主题"""
        if theme_name in self.THEMES or theme_name in self.custom_themes:
            self.current_theme = theme_name
            self.save_theme()
            return True
        return False

    def get_current_theme(self):
        """获取当前主题"""
        if self.current_theme in self.THEMES:
            return self.THEMES[self.current_theme]
        elif self.current_theme in self.custom_themes:
            return self.custom_themes[self.current_theme]
        else:
            return self.THEMES["dark_blue"]

    def get_theme_names(self):
        """获取所有主题名称"""
        return list(self.THEMES.keys()) + list(self.custom_themes.keys())

    def get_theme_info(self, theme_name):
        """获取主题信息"""
        if theme_name in self.THEMES:
            return self.THEMES[theme_name]
        elif theme_name in self.custom_themes:
            return self.custom_themes[theme_name]
        return {}

    def is_custom_theme(self, theme_name):
        """检查是否为自定义主题"""
        return theme_name in self.custom_themes

    def get_default_colors(self):
        """获取默认颜色模板"""
        return {
            "bg_primary": "#1a2a3a",
            "bg_secondary": "#2c3e50",
            "bg_tertiary": "#34495e",
            "text_primary": "#ecf0f1",
            "text_secondary": "#bdc3c7",
            "accent_primary": "#3498db",
            "accent_hover": "#2980b9",
            "accent_pressed": "#2980b9",
            "danger": "#e74c3c",
            "danger_hover": "#c0392b",
            "success": "#0c311c",
            "success_hover": "#042914",
            "secondary": "#95a5a6",
            "secondary_hover": "#7f8c8d"
        }

    def _hex_to_rgb(self, hex_color):
        hex_color = hex_color.lstrip('#')
        return tuple(int(hex_color[i:i+2], 16) for i in (0, 2, 4))

    def _rgb_to_hex(self, rgb):
        return '#{:02x}{:02x}{:02x}'.format(*[max(0, min(255, int(v))) for v in rgb])

    def _blend_colors(self, color_a, color_b, ratio=0.5):
        a = self._hex_to_rgb(color_a)
        b = self._hex_to_rgb(color_b)
        return self._rgb_to_hex(tuple(a[i] * (1 - ratio) + b[i] * ratio for i in range(3)))

    def _adjust_brightness(self, hex_color, factor):
        rgb = self._hex_to_rgb(hex_color)
        adjusted = tuple(max(0, min(255, int(c * factor))) for c in rgb)
        return self._rgb_to_hex(adjusted)

    def _is_light_color(self, hex_color):
        r, g, b = self._hex_to_rgb(hex_color)
        luminance = (0.299 * r + 0.587 * g + 0.114 * b) / 255
        return luminance > 0.65

    def generate_palette_from_primary(self, accent_primary, bg_primary=None):
        """根据主强调色生成一套推荐颜色"""
        try:
            accent_primary = accent_primary.strip()
            if not accent_primary.startswith('#') or len(accent_primary) != 7:
                raise ValueError("无效的主色调")

            accent_is_light = self._is_light_color(accent_primary)
            if bg_primary is None or not bg_primary.startswith('#') or len(bg_primary) != 7:
                bg_primary = self._blend_colors(accent_primary, '#0b1220', 0.92)
            else:
                bg_primary = bg_primary.strip()

            # 保持正文与背景对比
            text_primary = '#212529' if self._is_light_color(bg_primary) else '#ecf0f1'
            text_secondary = '#6c757d' if text_primary == '#212529' else '#bdc3c7'

            bg_secondary = self._blend_colors(bg_primary, accent_primary, 0.18)
            bg_tertiary = self._blend_colors(bg_primary, accent_primary, 0.12)
            secondary = self._blend_colors(text_primary, bg_primary, 0.64)
            secondary_hover = self._adjust_brightness(secondary, 0.88)

            if accent_is_light:
                accent_hover = self._adjust_brightness(accent_primary, 0.86)
                accent_pressed = self._adjust_brightness(accent_primary, 0.74)
            else:
                accent_hover = self._adjust_brightness(accent_primary, 1.25)
                accent_pressed = self._adjust_brightness(accent_primary, 0.78)

            return {
                "bg_primary": bg_primary,
                "bg_secondary": bg_secondary,
                "bg_tertiary": bg_tertiary,
                "text_primary": text_primary,
                "text_secondary": text_secondary,
                "accent_primary": accent_primary,
                "accent_hover": accent_hover,
                "accent_pressed": accent_pressed,
                "danger": self._blend_colors(accent_primary, '#e74c3c', 0.4),
                "danger_hover": self._adjust_brightness('#e74c3c', 0.85),
                "success": self._blend_colors(accent_primary, '#27ae60', 0.35),
                "success_hover": self._adjust_brightness('#27ae60', 0.9),
                "secondary": secondary,
                "secondary_hover": secondary_hover
            }

        except Exception:
            return self.get_default_colors()

    def generate_stylesheet(self, theme_name=None):
        """生成QSS样式表"""
        if theme_name is None:
            theme_name = self.current_theme

        theme = self.get_theme_info(theme_name)
        if not theme or "colors" not in theme:
            theme = self.THEMES["dark_blue"]
        colors = theme["colors"]

        return f"""
            QWidget {{
                background-color: {colors['bg_primary']};
                color: {colors['text_primary']};
                font-family: "Microsoft YaHei";
                font-size: 10pt;
            }}
            QLabel {{
                color: {colors['text_primary']};
            }}
            QLabel#title {{
                font-size: 18pt;
                font-weight: bold;
            }}
            QLabel#subtitle {{
                color: {colors['text_secondary']};
                font-size: 10pt;
            }}
            QLabel#status {{
                font-weight: bold;
            }}
            QLabel#stats {{
                background-color: {colors['bg_secondary']};
                font-size: 14pt;
                font-weight: bold;
            }}
            QPushButton {{
                background-color: {colors['accent_primary']};
                color: {colors['text_primary']};
                border: none;
                padding: 10px 20px;
                font-weight: bold;
                border-radius: 5px;
            }}
            QPushButton:hover {{
                background-color: {colors['accent_hover']};
            }}
            QPushButton:pressed {{
                background-color: {colors['accent_pressed']};
            }}
            QPushButton#danger {{
                background-color: {colors['danger']};
            }}
            QPushButton#danger:hover {{
                background-color: {colors['danger_hover']};
            }}
            QPushButton#secondary {{
                background-color: {colors['secondary']};
                font-weight: normal;
                padding: 8px 15px;
            }}
            QPushButton#secondary:hover {{
                background-color: {colors['secondary_hover']};
            }}
            QPushButton#success {{
                background-color: {colors['success']};
                padding: 8px 15px;
            }}
            QPushButton#success:hover {{
                background-color: {colors['success_hover']};
            }}
            QLineEdit {{
                background-color: {colors['bg_secondary']};
                color: {colors['text_primary']};
                border: 1px solid {colors['bg_tertiary']};
                padding: 5px;
                border-radius: 3px;
            }}
            QTextEdit {{
                background-color: {colors['bg_secondary']};
                color: {colors['text_primary']};
                border: 1px solid {colors['bg_tertiary']};
                border-radius: 3px;
            }}
            QTreeWidget {{
                background-color: {colors['bg_secondary']};
                color: {colors['text_primary']};
                border: 1px solid {colors['bg_tertiary']};
                alternate-background-color: {colors['bg_tertiary']};
            }}
            QTreeWidget::item {{
                padding: 5px;
            }}
            QTreeWidget::item:selected {{
                background-color: {colors['accent_primary']};
            }}
            QTabWidget::pane {{
                border: 1px solid {colors['bg_tertiary']};
                background-color: {colors['bg_primary']};
            }}
            QTabBar::tab {{
                background-color: {colors['bg_secondary']};
                color: {colors['text_primary']};
                padding: 8px 16px;
                border: 1px solid {colors['bg_tertiary']};
                margin-right: 2px;
            }}
            QTabBar::tab:selected {{
                background-color: {colors['accent_primary']};
            }}
            QTabBar::tab:hover {{
                background-color: {colors['accent_hover']};
            }}
            QGroupBox {{
                font-weight: bold;
                border: 2px solid {colors['accent_primary']};
                border-radius: 5px;
                margin-top: 1ex;
            }}
            QGroupBox::title {{
                subcontrol-origin: margin;
                left: 10px;
                padding: 0 5px 0 5px;
            }}
            QComboBox {{
                background-color: {colors['bg_secondary']};
                color: {colors['text_primary']};
                border: 1px solid {colors['bg_tertiary']};
                padding: 5px;
                border-radius: 3px;
            }}
            QComboBox::drop-down {{
                border: none;
            }}
            QComboBox::down-arrow {{
                image: none;
                border-left: 4px solid transparent;
                border-right: 4px solid transparent;
                border-top: 4px solid {colors['text_primary']};
                margin-right: 8px;
            }}
            QComboBox QAbstractItemView {{
                background-color: {colors['bg_secondary']};
                color: {colors['text_primary']};
                selection-background-color: {colors['accent_primary']};
            }}
        """
