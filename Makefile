.PHONY: all build run test benchmark clean help

all: build

build:
	cargo build --release

run:
	cargo run --release

dev:
	cargo run

test:
	cargo test

docker-build:
	docker build -t urlslim:latest .

docker-run:
	docker compose up -d

docker-stop:
	docker compose down

docker-logs:
	docker compose logs -f

benchmark:
	./scripts/benchmark.sh

stress:
	./scripts/stress_test.sh

profile:
	./scripts/profile_test.sh

quick-bench:
	./scripts/quick_bench.sh

docker-bench:
	docker compose -f docker-compose.yml -f docker-compose.benchmark.yml --profile benchmark up vegeta

clean:
	cargo clean
	rm -rf benchmark_results/
	rm -rf data/

help:
	@echo "URLSlim Makefile Commands:"
	@echo ""
	@echo "  Development:"
	@echo "    build         Build release binary"
	@echo "    run           Run release binary"
	@echo "    dev           Run debug binary"
	@echo "    test          Run tests"
	@echo ""
	@echo "  Docker:"
	@echo "    docker-build  Build Docker image"
	@echo "    docker-run    Start with docker compose"
	@echo "    docker-stop   Stop containers"
	@echo "    docker-logs   View logs"
	@echo "    docker-bench  Run benchmark in Docker"
	@echo ""
	@echo "  Benchmarking:"
	@echo "    benchmark     Run standard benchmark"
	@echo "    stress        Run stress test"
	@echo "    profile       Run profile test"
	@echo "    quick-bench   Quick benchmark"
	@echo ""
	@echo "  Cleanup:"
	@echo "    clean         Remove build artifacts"
