# mek反应堆与锅炉属性计算器
## 功能：
* 指定反应堆大小与冷却模式，计算反应堆最大燃烧速率与最优燃料棒排布方式
* 指定锅炉大小与加热模式，计算锅炉最大产出效率与最优建造方式
## 使用方法
1. 输入长宽高
2. 选择计算对象
3. 选择冷却模式/加热模式
## 注意：
* 锅炉的分压原件高度中包括外壳的高度，如高度为2时，与底部接触。 
* 两种计算中都未纳入环境散热影响
* 反应堆燃烧速率与冷却剂消耗：（因环境散热导致略小）
    * 水：烧1产20 000
    * 钠：烧1产200 000
# 原理
## 反应堆每tick做的事：
1. 如果激活了，就烧燃料

        燃烧速率、剩余燃料、（燃料棒数量*每个燃料棒的速率限制）取最小值烧掉燃料
            默认每个燃料棒的速率限制为1
        烧掉的燃料*每单位燃料产生的热量 添加到热容
            默认每单位燃料产生的热量为1 000 000
        烧掉的燃料与废水箱已存废料相加>0则爆仓，否则相加后存到废料箱
            爆仓只会爆辐射出来，不会炸
        
2. 处理冷却剂

        有效热量=沸腾效率*(当前温度-沸腾温度)*热容
        当前温度=热容热量/热容
            热容=外壳方块数 * 1 000=(长宽高-(长-2)(宽-2)(高-2)) * 1 000
            沸腾效率=min(1,燃料组件表面积/燃料组件数/4)
            沸腾温度=373.15K
        有效散热=冷却剂热导率(水0.5 钠蒸汽1)*有效热量
            水冷却剂加热速率=蒸汽效率(0.2)*有效散热/蒸汽热焓(默认10)
            气体冷却剂加热速率=有效散热/冷却剂热焓(钠蒸汽5)
        从热容热量中减掉有效散热
3. 模拟环境降温

        降温系数=空气系数+隔热系数+导热系数
            空气系数为 10 000
            隔热系数为 10 000
            导热系数为 10
        降温温度=(热容温度-环境温度)/降温系数
            环境温度为 300+25*(平均温度系数-0.8)
            平均温度系数为多方块结构8个角的方块温度系数的平均值
            温度系数取值[ -5,5 ],平原时为0.8
        环境散热=降温温度*热容
        从热容热量中减掉环境散热
4. 更新热量
5. 处理反应堆损伤
6. 处理辐射

## 锅炉每tick做的事：

1. 环境散热
   
        同反应堆
2. 更新热量
3. （如果有）消耗过热钠变成热量
   
        过热钠消耗=min(过热钠储量 * 冷却系数(0.4) * (1-锅炉温度/冷却剂温度(100 000)),冷却的钠蒸汽距离满仓的量)
        增加热量=过热钠消耗*钠的焓(5)
4. 消耗热量烧水
   
        烧水热量=(当前温度-沸腾温度) * 锅炉热容 * 锅炉水导热率
            锅炉水导热率默认为0.7
        有效热量=min(烧水热量,加热元件*加热元件导热率)
            加热元件导热率默认为160 000 000
        烧水量=min(蒸汽效率(0.2)*有效热量/蒸汽热焓(10),水箱的水,蒸汽箱剩余容量)
            水箱容量=分压原件以下（不含）的体积（包括外壳）* 16 000
            蒸汽容量=分压原件以上（含）的体积（包括外壳）* 160 000
        减少热量=烧水量*蒸汽热焓(10)/蒸汽效率(0.2)
        
### 锅炉的加热方式

1. 直接加热模式：

        最大烧水量=min(加热元件加热率/320 000,水箱容量,蒸汽容量)
1. 钠冷加热模式：

        消耗钠导致的热量增加>=烧水导致的热量减少+环境散热（可忽略）
        设烧水导致的热量减少=烧水热量且钠蒸汽不会满箱
        得温度T=最大烧水量/热容*71.4285
        得热量消耗=最大烧水量*50
        得过热钠最大消耗量=min(过热钠储量 * 冷却系数(0.4) * (1-锅炉温度/冷却剂温度(100 000)),冷却的钠蒸汽距离满仓的量,最大烧水量*10)