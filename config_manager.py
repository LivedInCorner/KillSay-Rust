import json
import os
import pathlib as pl
from datetime import datetime
from utils import get_base_dir

# ==================== 配置管理类 ====================
class ConfigManager:
    """管理多配置文件的加载、保存和切换"""
    
    DEFAULT_CONFIG = {
        "sniper_list": [""],
        "join_patterns": [
            "(.+?)加入了游戏",
            "(.+?) 加入了游戏",
            "(.+?) joined the game",
            "(.+?) 加入游戏",
            "(.+?) 进入了游戏"
        ],
        "anti_snipe_messages": [
            "/PLAY SWRSOLO"
        ],
        "kill_patterns": [
            "{d}被{k}击败",
            "{d}被炸成了粉尘, 幕后黑手是{k}!",
            "{d}消逝了, 幕后黑手是{k}!",
            "{d}被{k}用弓箭射穿了",
            "{d}跑得很快, 但是他还是摔了一跤, 最终被{k}击败了",
            "{k}击败了{d}"
        ],
        "kill_messages": [
            "黄沙百战穿金甲，不破楼兰终不还。",
            "十步杀一人，千里不留行。",
            "会挽雕弓如满月，西北望，射天狼。",
            "壮志饥餐胡虏肉，笑谈渴饮匈奴血。",
            "谈笑间，樯橹灰飞烟灭。",
            "金戈铁马，气吞万里如虎。",
            "马作的卢飞快，弓如霹雳弦惊。",
            "了却君王天下事，赢得生前身后名。",
            "驾长车，踏破贺兰山缺。",
            "人生自古谁无死？留取丹心照汗青。",
            "王师北定中原日，家祭无忘告乃翁。",
            "夜阑卧听风吹雨，铁马冰河入梦来。",
            "醉卧沙场君莫笑，古来征战几人回。",
            "但使龙城飞将在，不教胡马度阴山。",
            "葡萄美酒夜光杯，欲饮琵琶马上催。",
            "秦时明月汉时关，万里长征人未还。",
            "长风破浪会有时，直挂云帆济沧海。",
            "天生我材必有用，千金散尽还复来。",
            "千磨万击还坚劲，任尔东西南北风。",
            "粉骨碎身浑不怕，要留清白在人间。",
            "不要人夸好颜色，只留清气满乾坤。",
            "将军金甲夜不脱，半夜军行戈相拨，风头如刀面如割。",
            "报君黄金台上意，提携玉龙为君死。",
            "此去泉台招旧部，旌旗十万斩阎罗。",
            "一身转战三千里，一剑曾当百万师。",
            "欲将轻骑逐，大雪满弓刀。",
            "折戟沉沙铁未销，自将磨洗认前朝。",
            "黑云压城城欲摧，甲光向日金鳞开。",
            "角声满天秋色里，塞上燕脂凝夜紫。",
            "雪暗凋旗画，风多杂鼓声。",
            "林暗草惊风，将军夜引弓。",
            "愿将腰下剑，直为斩楼兰。",
            "相看白刃血纷纷，死节从来岂顾勋。",
            "昔日长城战，咸言意气高。",
            "突营射杀呼延将，独领残兵千骑归。",
            "骝马新跨白玉鞍，战罢沙场月色寒。",
            "城头铁鼓声犹震，匣里金刀血未干。",
            "牙璋辞凤阙，铁骑绕龙城。",
            "雪净胡天牧马还，月明羌笛戍楼间。",
            "大漠风尘日色昏，红旗半卷出辕门。",
            "只解沙场为国死，何须马革裹尸还。",
            "何当金络脑，快走踏清秋。",
            "烽火照西京，心中自不平。",
            "宁为百夫长，胜作一书生。",
            "誓扫匈奴不顾身，五千貂锦丧胡尘。",
            "汉家旌帜满阴山，不遣胡儿匹马还。",
            "弯弓辞汉月，插羽破天骄。",
            "寒沙连骑迹，朔吹断边声。",
            "单醪投长河，三军尽翱翔。",
            "遥看是君家，松柏冢累累。",
            "捐躯赴国难，视死忽如归。",
            "胜败兵家事不期，包羞忍耻是男儿。",
            "凭君莫话封侯事，一将功成万骨枯。",
            "年年战骨埋荒外，空见蒲桃入汉家。",
            "战士军前半死生，美人帐下犹歌舞。",
            "青海长云暗雪山，孤城遥望玉门关。",
            "前军夜战洮河北，已报生擒吐谷浑。",
            "北斗七星高，哥舒夜带刀。",
            "晓战随金鼓，宵眠抱玉鞍。",
            "横笛闻声不见人，红旗直上天山雪。",
            "客子过壕追野马，将军韬箭射天狼。",
            "胡雁哀鸣夜夜飞，胡儿眼泪双双落。",
            "关西老将不胜愁，驻马听之双泪流。",
            "男儿何不带吴钩，收取关山五十州。",
            "请君暂上凌烟阁，若个书生万户侯。",
            "汉兵奋迅如霹雳，虏骑崩腾畏蒺藜。",
            "剑外忽传收蓟北，初闻涕泪满衣裳。",
            "位卑未敢忘忧国，事定犹须待阖棺。",
            "遗民泪尽胡尘里，南望王师又一年。",
            "楼船夜雪瓜洲渡，铁马秋风大散关。",
            "山河破碎风飘絮，身世浮沉雨打萍。",
            "三万里河东入海，五千仞岳上摩天。",
            "尔曹身与名俱灭，不废江河万古流。",
            "出师未捷身先死，长使英雄泪满襟。",
            "出师一表真名世，千载谁堪伯仲间。",
            "国破山河在，城春草木深。",
            "繁霜尽是心头血，洒向千峰秋叶丹。",
            "裹尸马革英雄事，纵死终令汗竹香。",
            "一寸丹心图报国，两行清泪为思亲。",
            "天地英雄气，千秋尚凛然。",
            "伏波惟愿裹尸还，定远何须生入关。"
        ],
        "message_prefix": "",
        "killsay_format": "{prefix}{message}"
    }
    
    def __init__(self, config_dir=None):
        if config_dir is None:
            config_dir = os.path.join(get_base_dir(), "configs")
        self.config_dir = pl.Path(config_dir)
        self.config_dir.mkdir(exist_ok=True)
        self.current_config_path = None
        self.current_config = None
        
        # 加载默认配置或上次使用的配置
        self._load_last_used()
    
    def _load_last_used(self):
        """加载上次使用的配置文件，如果不存在则创建默认配置"""
        last_used_file = self.config_dir / "last_used.json"
        if last_used_file.exists():
            try:
                with open(last_used_file, 'r', encoding='utf-8') as f:
                    data = json.load(f)
                    last_path = data.get("last_config")
                    if last_path and pl.Path(last_path).exists():
                        self.load_config(last_path)
                        return
            except:
                pass
        
        # 创建默认配置
        default_path = self.config_dir / "default.json"
        if not default_path.exists():
            self.save_config(default_path, self.DEFAULT_CONFIG)
        self.load_config(default_path)
    
    def get_config_list(self):
        """获取所有配置文件的列表"""
        configs = []
        for file in self.config_dir.glob("*.json"):
            if file.name != "last_used.json":
                configs.append({
                    "name": file.stem,
                    "path": str(file),
                    "modified": datetime.fromtimestamp(file.stat().st_mtime).strftime("%Y-%m-%d %H:%M:%S")
                })
        return sorted(configs, key=lambda x: x["modified"], reverse=True)
    
    def load_config(self, config_path):
        """加载指定的配置文件"""
        try:
            with open(config_path, 'r', encoding='utf-8') as f:
                config = json.load(f)
            
            # 确保配置包含所有必要字段
            for key in self.DEFAULT_CONFIG:
                if key not in config:
                    config[key] = self.DEFAULT_CONFIG[key]
            
            self.current_config = config
            self.current_config_path = pl.Path(config_path)
            
            # 保存最近使用的配置
            self._save_last_used()
            
            return True
        except Exception as e:
            print(f"加载配置失败: {e}")
            return False
    
    def _save_last_used(self):
        """保存最近使用的配置路径"""
        last_used_file = self.config_dir / "last_used.json"
        try:
            with open(last_used_file, 'w', encoding='utf-8') as f:
                json.dump({"last_config": str(self.current_config_path)}, f)
        except:
            pass
    
    def save_current_config(self):
        """保存当前配置到当前路径"""
        if self.current_config_path:
            self.save_config(self.current_config_path, self.current_config)
    
    def save_config(self, config_path, config_data):
        """保存配置到指定路径"""
        with open(config_path, 'w', encoding='utf-8') as f:
            json.dump(config_data, f, ensure_ascii=False, indent=2)
    
    def create_new_config(self, name):
        """创建新配置文件"""
        config_path = self.config_dir / f"{name}.json"
        if config_path.exists():
            raise Exception("配置文件已存在")
        
        self.save_config(config_path, self.DEFAULT_CONFIG)
        return config_path
    
    def delete_config(self, config_path):
        """删除配置文件"""
        path = pl.Path(config_path)
        if path.exists() and path != self.current_config_path:
            path.unlink()
            return True
        return False
