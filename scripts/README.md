# 性能测试

## 前置条件

安装压测工具（任选其一或全部）：

```bash
# hey (推荐，Go 实现)
brew install hey

# wrk (高性能)
brew install wrk

# vegeta (功能丰富)
brew install vegeta
```

## 快速测试

确保服务已启动：

```bash
# 启动服务
docker compose up -d

# 运行基准测试
./scripts/benchmark.sh

# 或使用默认配置
BASE_URL=http://localhost:3000 ./scripts/benchmark.sh
```

## 测试脚本说明

### benchmark.sh
基础性能测试，包含：
- 重定向性能（读密集）
- 创建链接性能（写）
- 二维码生成性能

```bash
DURATION=30s RATE=1000 ./scripts/benchmark.sh
```

### stress_test.sh
阶梯式压力测试，逐步增加负载直到目标 QPS：

```bash
STEPS=5 MAX_RATE=10000 ./scripts/stress_test.sh
```

### profile_test.sh
详细性能分析，包含多种场景：

```bash
./scripts/profile_test.sh
```

## 预期性能指标

| 指标 | 目标值 | 说明 |
|------|--------|------|
| QPS (重定向) | > 10,000 | 缓存命中时 |
| QPS (创建链接) | > 500 | 含数据库写入 |
| P99 延迟 | < 20ms | 重定向 |
| 内存占用 | < 200MB | 空载+运行时 |

## 使用 Docker 进行压测

```bash
# 启动服务
docker compose up -d

# 使用 vegeta 容器压测
docker run --rm -it --network host peterevans/vegeta \
    attack -duration=30s -rate=5000 -targets=http://localhost:3000/test | \
    vegeta report

# 或使用 hey 容器
docker run --rm -it --network host williamyeh/hey \
    -z 30s -q 5000 -c 200 http://localhost:3000/test
```

## 结果分析

结果保存在 `benchmark_results/` 目录：

```
benchmark_results/
├── redirect_hey.txt      # 重定向测试结果
├── create_hey.txt        # 创建链接测试结果
├── vegeta_report.txt     # vegeta 文本报告
├── plot.html             # vegeta 可视化图表
└── stress/               # 压力测试结果
    ├── stress_2000.json
    ├── stress_4000.json
    └── ...
```

### 查看结果

```bash
# 查看汇总
cat benchmark_results/redirect_hey.txt

# 查看 vegeta 图表
open benchmark_results/plot.html
```
