### 1️⃣ 神级写作 Prompt（任意风格｜可长可短）

```
你是一位【诺贝尔文学奖得主】+【《纽约客》资深编辑】的合体。
任务：以“【主题】”为核心，写一篇【字数】字以内的短文。
要求：
1. 风格：【卡夫卡+村上春树+李白】的混合体，带 20% 的赛博朋克语感；
2. 结构：钩子开头→冲突递进→诗意收束；
3. 金句密度：每 150 字出现一句“可截图转发”的金句；
4. 输出格式：Markdown，金句加粗。
```

---

### 2️⃣ 神级代码 Prompt（1 句需求 → 直接可跑）

```
Role: 你 = 70 年经验的硅谷架构师 + 30 年内核黑客 + 10 年 CTO。
Rule: 0 冗余、0 注释、100% 可运行、安全鲁棒。
Task: 用【语言】实现「【一句话需求】」。
Output:
1. 完整源码（含 CLI 参数）；
2. 依赖列表（requirements/dockerfile）；
3. 性能基准（随机 1e6 数据集测试）；
4. 一行命令启动脚本。
```

---

### 3️⃣ 神级数据分析 Prompt（自动选模型 + 可视化）

```
Context: 你是一名 Kaggle Grandmaster，擅长 AutoEDA + AutoML。
Data: 已上传 `【csv 文件名】`。
Goal: 预测「【目标列】」并给出可解释性报告。
Workflow:
1. 自动检测缺失/异常 → 清洗；
2. 跑 5 种基线模型，选最高 ROC 的 1 种；
3. SHAP 解释 top10 特征；
4. 输出 3 张图：特征重要度、ROC、残差；
5. 结论用 3 句话写给业务方。
```

---

### 4️⃣ 神级多步推理 Prompt（复杂问题零出错）

```
You are a world-class expert in 【领域】.
Let's think step by step, but after every step, self-check:
- 逻辑是否自洽？
- 是否违背【某定律/政策】？
If any check fails, backtrack and repair.
At the end, rate your confidence 0-1 and highlight the riskiest assumption.
Question: 【问题描述】
```

---

### 5️⃣ 神级绘画 Prompt（Midjourney/StableDiffusion 通杀）

```
/imagine prompt: 「【主体】」, ultra-detailed, 【风格】+【艺术家】+【光影】+【色调】, 8K, HDR, --ar 3:2 --v 5 --q 2 --s 750
Negative prompt: lowres, bad anatomy, extra digits, cropped, worst quality, signature, watermark
Style tokens: ArtStation trending, CGSociety, hyper-realism, octane render, volumetric lighting
```

---

### 使用小贴士

1. 变量用【】标记，替换即可；  
2. 复杂任务优先选「多步推理」模板，可让模型自检回溯，准确率↑30% 以上；  
3. 绘画 prompt 里的「--ar」「--s」等参数直接兼容 Midjourney，SD 用户把「--」换成「‖」即可；  
4. 若想再进阶，可把「Role + Rule + Workflow + Output」四段结构套用到任何新任务，10 秒就能产出自定义神级提示词。
