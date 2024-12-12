# 基础镜像
FROM rust:latest as builder

# 设置工作目录
WORKDIR /app

# 复制项目文件
COPY . .

# 编译项目
RUN cargo build --bin rust-api-server --release

# 使用更小的基础镜像
FROM debian:bullseye-slim

# 设置工作目录
WORKDIR /app

# 复制构建的二进制文件
COPY --from=builder /app/target/release/rust-api-server /app

# 暴露端口（根据 API 服务的端口）
EXPOSE 8080

# 设置启动命令
CMD ["./rust-api-server"]
