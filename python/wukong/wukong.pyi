from enum import Enum, auto
from decimal import Decimal
from datetime import datetime
from typing import List, Optional, Self

BANNER: str = ...
"""横幅"""

class Mode(Enum):
    """运行模式"""

    Backtest = auto()
    """回测"""
    Sandbox = auto()
    """模拟"""
    Real = auto()
    """实盘"""

class Type(Enum):
    """交易类型"""

    Limit = auto()
    """限价交易"""
    Market = auto()
    """市价交易"""

class Side(Enum):
    """交易方向"""

    Long = auto()
    """做多"""
    Short = auto()
    """做空"""

class TimeFrame(Enum):
    """时间周期"""

    Minute = auto()
    """1分钟"""
    Minute3 = auto()
    """3分钟"""
    Minute5 = auto()
    """5分钟"""
    Minute15 = auto()
    """15分钟"""
    Minute30 = auto()
    """30分钟"""
    Hour = auto()
    """1小时"""
    Hour2 = auto()
    """2小时"""
    Hour4 = auto()
    """4小时"""
    Hour6 = auto()
    """6小时"""
    Hour8 = auto()
    """8小时"""
    Hour12 = auto()
    """12小时"""
    Day = auto()
    """1天"""
    Day3 = auto()
    """3天"""
    Week = auto()
    """1周"""
    Month = auto()
    """1月"""

class OrderStatus(Enum):
    """订单状态"""

    Created = auto()
    """创建"""
    Submited = auto()
    """已提交"""
    Pending = auto()
    """挂单中"""
    Partial = auto()
    """部分成交"""
    Completed = auto()
    """完全成交"""
    Rejected = auto()
    """被拒绝"""
    Canceled = auto()
    """已取消"""

class Order:
    """订单"""

    symbol: str
    """交易对"""
    id: str
    """ID"""
    type: Type
    """类型"""
    side: Side
    """方向"""
    reduce: bool
    """减仓"""
    leverage: Decimal
    """杠杆倍数"""
    size: Decimal
    """数量"""
    price: Decimal
    """价格"""
    time: datetime
    """下单时间"""
    margin: Decimal
    """保证金"""
    deal_size: Decimal
    """成交数量"""
    deal_price: Decimal
    """成交均价"""
    deal_fee: Decimal
    """成交手续费"""
    status: OrderStatus
    """状态"""
