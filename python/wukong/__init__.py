from .wukong import *
from decimal import Decimal
from datetime import datetime
import polars as pl


BACKTEST = Mode.Backtest
"""
回测
"""
SANDBOX = Mode.Sandbox
"""
模拟
"""
REAL = Mode.Real
"""
实盘
"""

LIMIT = Type.Limit
"""
限价交易
"""
MARKET = Type.Market
"""
市价交易
"""

LONG = Side.Long
"""
做多
"""
SHORT = Side.Short
"""
做空
"""

TF_1M = TimeFrame.Minute
"""
1分钟
"""
TF_3M = TimeFrame.Minute3
"""
3分钟
"""
TF_5M = TimeFrame.Minute5
"""
5分钟
"""
TF_15M = TimeFrame.Minute15
"""
15分钟
"""
TF_30M = TimeFrame.Minute30
"""
30分钟
"""
TF_1H = TimeFrame.Hour
"""
1小时
"""
TF_2H = TimeFrame.Hour2
"""
2小时
"""
TF_4H = TimeFrame.Hour4
"""
4小时
"""
TF_6H = TimeFrame.Hour6
"""
6小时
"""
TF_8H = TimeFrame.Hour8
"""
8小时
"""
TF_12H = TimeFrame.Hour12
"""
12小时
"""
TF_1D = TimeFrame.Day
"""
1天
"""
TF_3D = TimeFrame.Day3
"""
3天
"""
TF_1W = TimeFrame.Week
"""
1周
"""
TF_1MONTH = TimeFrame.Month
"""
1月
"""

CREATED = OrderStatus.Created
"""
创建
"""
SUBMITED = OrderStatus.Submited
"""
已提交
"""
PENDING = OrderStatus.Pending
"""
挂单中
"""
PARTIAL = OrderStatus.Partial
"""
部分成交
"""
COMPLETED = OrderStatus.Completed
"""
完全成交
"""
REJECTED = OrderStatus.Rejected
"""
被拒绝
"""
CANCELED = OrderStatus.Canceled
"""
已取消
"""


REDUCE = True
"""
减仓
"""
