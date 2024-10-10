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
    """K线周期"""

    Minute = auto()
    Minute3 = auto()
    Minute5 = auto()
    Minute15 = auto()
    Minute30 = auto()
    Hour = auto()
    Hour2 = auto()
    Hour4 = auto()
    Hour6 = auto()
    Hour8 = auto()
    Hour12 = auto()
    Day = auto()
    Day3 = auto()
    Week = auto()
    Month = auto()

class OrderStatus(Enum):
    """订单状态"""

    Created = auto()
    Submited = auto()
    Pending = auto()
    Partial = auto()
    Completed = auto()
    Rejected = auto()
    Canceled = auto()
