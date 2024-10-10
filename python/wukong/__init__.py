from .wukong import *
from decimal import Decimal
from datetime import datetime
from typing import List, Optional, Self


BACKTEST = Mode.Backtest
"""回测"""
SANDBOX = Mode.Sandbox
"""模拟"""
REAL = Mode.Real
"""实盘"""

LIMIT = Type.Limit
"""限价交易"""
MARKET = Type.Market
"""市价交易"""

LONG = Side.Long
"""做多"""
SHORT = Side.Short
"""做空"""

TF_1M = TimeFrame.Minute
TF_3M = TimeFrame.Minute3
TF_5M = TimeFrame.Minute5
TF_15M = TimeFrame.Minute15
TF_30M = TimeFrame.Minute30
TF_1H = TimeFrame.Hour
TF_2H = TimeFrame.Hour2
TF_4H = TimeFrame.Hour4
TF_6H = TimeFrame.Hour6
TF_8H = TimeFrame.Hour8
TF_12H = TimeFrame.Hour12
TF_1D = TimeFrame.Day
TF_3D = TimeFrame.Day3
TF_1W = TimeFrame.Week
TF_1MONTH = TimeFrame.Month

CREATED = OrderStatus.Created
SUBMITED = OrderStatus.Submited
PENDING = OrderStatus.Pending
PARTIAL = OrderStatus.Partial
COMPLETED = OrderStatus.Completed
REJECTED = OrderStatus.Rejected
CANCELED = OrderStatus.Canceled


REDUCE = True
"""减仓"""
